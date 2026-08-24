use std::process::{Command, Stdio};

use crate::driver::{
    argv_or_stdin_prompt, extract_json_text, find_executable, generic_auth_error,
    unwrap_vendor_json, Capabilities, Capability, Driver, DriverStatus, Invocation, MapRequest,
};
use crate::error::OccamError;
use crate::task::OutputMode;

#[derive(Clone, Copy)]
enum GrokEntry {
    Exec,
    PromptLong,
    PromptShort,
}

pub struct GrokDriver {
    command: String,
}

impl GrokDriver {
    pub fn new(command: String) -> Self {
        Self { command }
    }

    fn probe_entry(&self, program: &std::path::Path) -> GrokEntry {
        let output = Command::new(program)
            .arg("--help")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        let text = match output {
            Ok(o) => {
                let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
                s.push_str(&String::from_utf8_lossy(&o.stderr));
                s
            }
            Err(_) => return GrokEntry::PromptShort,
        };
        let lower = text.to_ascii_lowercase();
        if lower.contains("grok exec")
            || lower.lines().any(|l| {
                let t = l.trim();
                t == "exec" || t.starts_with("exec ") || t.starts_with("exec\t")
            })
        {
            return GrokEntry::Exec;
        }
        if lower.contains("--prompt") {
            return GrokEntry::PromptLong;
        }
        GrokEntry::PromptShort
    }
}

impl Driver for GrokDriver {
    fn id(&self) -> &'static str {
        "grok"
    }

    fn command(&self) -> &str {
        &self.command
    }

    fn detect(&self) -> DriverStatus {
        match find_executable(&self.command) {
            Some(_) => DriverStatus::Ready,
            None => DriverStatus::Missing,
        }
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities::new(&[Capability::StructuredOutput, Capability::StreamingOutput])
    }

    fn login_hint(&self) -> &'static str {
        "Authenticate with the Grok CLI, then retry."
    }

    fn map(&self, req: &MapRequest<'_>) -> Result<Invocation, OccamError> {
        let program =
            find_executable(&self.command).ok_or_else(|| OccamError::DriverUnavailable {
                id: self.id().into(),
                detail: format!("'{}' not found on PATH", self.command),
            })?;

        let mut prompt = req.prompt.to_string();
        if req.output == OutputMode::Json {
            prompt = format!(
                "{}\n\n{}",
                crate::prompt::json_instruction(req.schema),
                prompt
            );
        }
        let (prompt_args, stdin) = argv_or_stdin_prompt(&prompt);
        let mut args = Vec::new();
        match self.probe_entry(&program) {
            GrokEntry::Exec => {
                args.push("exec".into());
                if prompt_args.is_empty() {
                    args.push("-".into());
                } else {
                    args.extend(prompt_args);
                }
            }
            GrokEntry::PromptLong => {
                args.push("--prompt".into());
                if let Some(p) = prompt_args.into_iter().next() {
                    args.push(p);
                }
            }
            GrokEntry::PromptShort => {
                args.push("-p".into());
                if let Some(p) = prompt_args.into_iter().next() {
                    args.push(p);
                }
            }
        }
        Ok(Invocation {
            program,
            args,
            stdin,
        })
    }

    fn normalize_stdout(&self, raw: &[u8], output: OutputMode) -> Result<String, OccamError> {
        let text = String::from_utf8_lossy(raw).to_string();
        match output {
            OutputMode::Text => Ok(text.trim_end().to_string()),
            OutputMode::Json => unwrap_vendor_json(&extract_json_text(&text)),
        }
    }

    fn looks_like_auth_error(&self, stderr: &str, _code: Option<i32>) -> bool {
        generic_auth_error(stderr)
    }
}
