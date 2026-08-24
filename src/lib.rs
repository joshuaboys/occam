use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

pub mod cli;
pub mod config;
pub mod driver;
pub mod error;
pub mod output;
pub mod prompt;
pub mod run;
pub mod task;

use cli::{Cli, Command};
use config::{resolve_driver_id, Config};
use driver::{make_driver, Capability, Driver, DriverStatus, MapRequest};
use error::{print_error, OccamError};
use output::{
    dump_stderr_on_error, emit_failure_envelope, emit_result, forward_stderr, parse_json_body,
    validate_schema, Envelope,
};
use task::{OutputMode, StdinMode, Task};

pub async fn run() -> ExitCode {
    let cli = Cli::parse_args();
    match run_cli(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            print_error(&err);
            err.as_exit_code()
        }
    }
}

async fn run_cli(cli: Cli) -> Result<(), OccamError> {
    let cwd = std::env::current_dir()?;
    let cfg = Config::load(&cwd)?;

    match cli.command {
        Some(Command::Drivers) => {
            print_drivers(&cfg);
            Ok(())
        }
        Some(Command::Tasks) => {
            print_tasks(&cfg);
            Ok(())
        }
        None => {
            let name = cli
                .task
                .as_deref()
                .ok_or_else(|| OccamError::usage("missing task name. Try occam --help"))?;
            match name {
                "drivers" => {
                    print_drivers(&cfg);
                    Ok(())
                }
                "tasks" => {
                    print_tasks(&cfg);
                    Ok(())
                }
                _ => run_task(&cli, &cfg, name, &cwd).await,
            }
        }
    }
}

fn print_drivers(cfg: &Config) {
    for id in driver::builtin_ids() {
        let command = cfg.driver_command(id);
        let d = match make_driver(id, command.clone()) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let status = d.detect();
        let resolved = driver::find_executable(&command)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| command.to_string());
        let caps = d.capabilities().list().join(",");
        println!("{id}\t{resolved}\t{}\t{caps}", status.as_str());
    }
}

fn print_tasks(cfg: &Config) {
    if cfg.tasks.is_empty() {
        eprintln!("occam: no tasks defined (looked for occam.toml, .occam.toml, ~/.config/occam)");
        return;
    }
    for (name, task) in &cfg.tasks {
        let driver = task.def.driver.as_deref().unwrap_or("-");
        let output = match task.def.output {
            OutputMode::Text => "text",
            OutputMode::Json => "json",
        };
        println!("{name}\t{driver}\t{output}");
    }
}

async fn run_task(
    cli: &Cli,
    cfg: &Config,
    name: &str,
    invoke_cwd: &Path,
) -> Result<(), OccamError> {
    let task = cfg
        .tasks
        .get(name)
        .cloned()
        .ok_or_else(|| OccamError::usage(format!("unknown task '{name}'")))?;

    let stdin = read_stdin(&task)?;
    let output = if cli.json {
        OutputMode::Json
    } else {
        task.def.output
    };
    let schema = if let Some(path) = &cli.schema {
        task::load_schema_spec(&path.to_string_lossy(), invoke_cwd)?
    } else {
        task.load_schema()?
    };
    let effective_output = if schema.is_some() {
        OutputMode::Json
    } else {
        output
    };

    let timeout = if let Some(t) = &cli.timeout {
        task::parse_duration(t)?
    } else if let Some(t) = task.timeout()? {
        t
    } else {
        task::parse_duration(&cfg.timeout)?
    };
    let max_output = task.max_output_bytes()?.unwrap_or(cfg.max_output_bytes);

    let chosen_id = resolve_driver_id(
        cli.driver.as_deref(),
        task.def.driver.as_deref(),
        &cfg.default_driver,
    );
    let (driver, driver_id) = select_driver(cfg, &chosen_id)?;

    if task.def.max_turns.is_some() && !driver.capabilities().contains(Capability::MaxTurns) {
        return Err(OccamError::Unsupported {
            id: driver_id.clone(),
            capability: "max_turns".into(),
        });
    }
    for req in &task.def.requires {
        let Some(cap) = Capability::parse(req) else {
            return Err(OccamError::usage(format!(
                "unknown capability requirement '{req}'"
            )));
        };
        if !driver.capabilities().contains(cap) {
            return Err(OccamError::Unsupported {
                id: driver_id.clone(),
                capability: req.clone(),
            });
        }
    }
    if effective_output == OutputMode::Json
        && !driver.capabilities().contains(Capability::StructuredOutput)
        && !driver.capabilities().contains(Capability::SchemaOutput)
    {
        return Err(OccamError::Unsupported {
            id: driver_id.clone(),
            capability: "structured_output".into(),
        });
    }

    let child_cwd = resolve_cwd(cli.cwd.as_deref(), task.def.cwd.as_deref(), invoke_cwd)?;
    let mut assembled = prompt::assemble(&task, cli.prompt.as_deref(), stdin.as_deref());
    if assembled.trim().is_empty() {
        return Err(OccamError::usage(
            "empty prompt: provide task instructions, --prompt, or piped stdin",
        ));
    }

    let limits = run::RunLimits {
        timeout,
        max_output_bytes: max_output,
    };

    let mut mapped = driver.map(&MapRequest {
        prompt: &assembled,
        output: effective_output,
        schema: schema.as_ref(),
        max_turns: task.def.max_turns,
    })?;

    let first = run::spawn(&mapped, &child_cwd, &limits, mapped.stdin.as_deref()).await?;
    if !cli.quiet {
        forward_stderr(&first.stderr, false);
    }

    let stderr_text = String::from_utf8_lossy(&first.stderr).into_owned();
    if first.timed_out {
        let err = OccamError::Timeout(humantime::format_duration(timeout).to_string());
        return finish_err(
            cli,
            &driver_id,
            name,
            first.duration,
            first.code,
            err,
            &first.stderr,
        );
    }
    if first.truncated {
        let err = OccamError::DriverFailed("driver output exceeded max_output".into());
        return finish_err(
            cli,
            &driver_id,
            name,
            first.duration,
            first.code,
            err,
            &first.stderr,
        );
    }
    if driver.looks_like_auth_error(&stderr_text, first.code) {
        let err = OccamError::Auth {
            id: driver_id.clone(),
            detail: format!("driver '{driver_id}' requires authentication"),
            hint: driver.login_hint().to_string(),
        };
        return finish_err(
            cli,
            &driver_id,
            name,
            first.duration,
            first.code,
            err,
            &first.stderr,
        );
    }
    if first.code.unwrap_or(1) != 0 {
        let err = OccamError::DriverFailed(format!(
            "driver '{driver_id}' exited {}",
            first.code.unwrap_or(-1)
        ));
        return finish_err(
            cli,
            &driver_id,
            name,
            first.duration,
            first.code,
            err,
            &first.stderr,
        );
    }

    let mut body = driver.normalize_stdout(&first.stdout, effective_output)?;
    let mut duration = first.duration;
    let mut driver_exit = first.code;

    let mut json_value = None;
    if effective_output == OutputMode::Json {
        match parse_and_validate(&body, schema.as_ref()) {
            Ok(v) => json_value = v,
            Err(msg) => {
                if !cli.quiet {
                    eprintln!("occam: structured output invalid, retrying once");
                }
                assembled = prompt::repair_prompt(&assembled, &body, &msg);
                mapped = driver.map(&MapRequest {
                    prompt: &assembled,
                    output: effective_output,
                    schema: schema.as_ref(),
                    max_turns: task.def.max_turns,
                })?;
                let repair =
                    run::spawn(&mapped, &child_cwd, &limits, mapped.stdin.as_deref()).await?;
                if !cli.quiet {
                    forward_stderr(&repair.stderr, false);
                }
                duration = repair.duration;
                driver_exit = repair.code;
                if repair.timed_out {
                    let err = OccamError::Timeout(humantime::format_duration(timeout).to_string());
                    return finish_err(
                        cli,
                        &driver_id,
                        name,
                        duration,
                        driver_exit,
                        err,
                        &repair.stderr,
                    );
                }
                if repair.code.unwrap_or(1) != 0 {
                    let err = OccamError::Validation(msg);
                    return finish_err(
                        cli,
                        &driver_id,
                        name,
                        duration,
                        driver_exit,
                        err,
                        &repair.stderr,
                    );
                }
                body = driver.normalize_stdout(&repair.stdout, effective_output)?;
                match parse_and_validate(&body, schema.as_ref()) {
                    Ok(v) => json_value = v,
                    Err(msg2) => {
                        let err = OccamError::Validation(msg2);
                        return finish_err(
                            cli,
                            &driver_id,
                            name,
                            duration,
                            driver_exit,
                            err,
                            &repair.stderr,
                        );
                    }
                }
            }
        }
    }

    let env = Envelope {
        version: 1,
        status: "completed".into(),
        driver: driver_id,
        task: name.into(),
        result: json_value,
        duration_ms: duration.as_millis() as u64,
        driver_exit,
        error: None,
    };
    emit_result(effective_output, cli.envelope, env, &body)
}

fn parse_and_validate(
    body: &str,
    schema: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    let value = parse_json_body(body).map_err(|e| e.to_string())?;
    if let Some(schema) = schema {
        validate_schema(&value, schema)?;
    }
    Ok(Some(value))
}

fn finish_err(
    cli: &Cli,
    driver_id: &str,
    task: &str,
    duration: Duration,
    driver_exit: Option<i32>,
    err: OccamError,
    stderr: &[u8],
) -> Result<(), OccamError> {
    if cli.quiet {
        dump_stderr_on_error(stderr);
    }
    if cli.envelope {
        let env = Envelope {
            version: 1,
            status: err.envelope_status().into(),
            driver: driver_id.into(),
            task: task.into(),
            result: None,
            duration_ms: duration.as_millis() as u64,
            driver_exit,
            error: Some(err.to_string()),
        };
        let _ = emit_failure_envelope(&env);
    }
    Err(err)
}

fn select_driver(cfg: &Config, chosen: &str) -> Result<(Box<dyn Driver>, String), OccamError> {
    let command = cfg.driver_command(chosen);
    let driver = make_driver(chosen, command)?;
    match driver.detect() {
        DriverStatus::Ready => Ok((driver, chosen.to_string())),
        DriverStatus::Unauthenticated => Err(OccamError::Auth {
            id: chosen.into(),
            detail: format!("driver '{chosen}' is installed but not authenticated"),
            hint: driver.login_hint().to_string(),
        }),
        DriverStatus::Missing => {
            for alt in &cfg.fallback {
                if alt == chosen {
                    continue;
                }
                let cmd = cfg.driver_command(alt);
                let d = make_driver(alt, cmd)?;
                if d.detect() == DriverStatus::Ready {
                    eprintln!("occam: driver '{chosen}' unavailable, falling back to '{alt}'");
                    return Ok((d, alt.clone()));
                }
            }
            Err(OccamError::DriverUnavailable {
                id: chosen.into(),
                detail: format!("'{}' not found on PATH", cfg.driver_command(chosen)),
            })
        }
    }
}

fn resolve_cwd(
    flag: Option<&Path>,
    task_cwd: Option<&str>,
    invoke: &Path,
) -> Result<PathBuf, OccamError> {
    if let Some(p) = flag {
        return Ok(if p.is_absolute() {
            p.to_path_buf()
        } else {
            invoke.join(p)
        });
    }
    match task_cwd {
        None | Some("invoke") | Some(".") => Ok(invoke.to_path_buf()),
        Some(other) => {
            let p = PathBuf::from(other);
            Ok(if p.is_absolute() { p } else { invoke.join(p) })
        }
    }
}

fn read_stdin(task: &Task) -> Result<Option<String>, OccamError> {
    let stdin = std::io::stdin();
    let is_tty = stdin.is_terminal();
    match task.def.stdin {
        StdinMode::Forbidden => {
            if !is_tty {
                return Err(OccamError::usage(format!(
                    "task '{}' forbids stdin, but a pipe was provided",
                    task.name
                )));
            }
            Ok(None)
        }
        StdinMode::Required => {
            if is_tty {
                return Err(OccamError::usage(format!(
                    "task '{}' requires stdin (pipe a payload)",
                    task.name
                )));
            }
            let mut buf = String::new();
            stdin.lock().read_to_string(&mut buf)?;
            if buf.trim().is_empty() {
                return Err(OccamError::usage(format!(
                    "task '{}' requires stdin (pipe a payload)",
                    task.name
                )));
            }
            Ok(Some(buf))
        }
        StdinMode::Optional => {
            if is_tty {
                return Ok(None);
            }
            let mut buf = String::new();
            stdin.lock().read_to_string(&mut buf)?;
            if buf.is_empty() {
                Ok(None)
            } else {
                Ok(Some(buf))
            }
        }
    }
}
