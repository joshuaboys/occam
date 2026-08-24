use serde::Serialize;
use serde_json::Value;

use crate::error::OccamError;
use crate::task::OutputMode;

#[derive(Debug, Clone, Serialize)]
pub struct Envelope {
    pub version: u32,
    pub status: String,
    pub driver: String,
    pub task: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_exit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn validate_schema(instance: &Value, schema: &Value) -> Result<(), String> {
    let validator = jsonschema::validator_for(schema).map_err(|e| e.to_string())?;
    let errors: Vec<String> = validator
        .iter_errors(instance)
        .map(|e| {
            let path = e.instance_path.to_string();
            if path.is_empty() {
                e.to_string()
            } else {
                format!("{path}: {e}")
            }
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

pub fn parse_json_body(text: &str) -> Result<Value, OccamError> {
    serde_json::from_str(text.trim())
        .map_err(|e| OccamError::Validation(format!("driver did not emit JSON: {e}")))
}

pub fn emit_result(
    mode: OutputMode,
    envelope: bool,
    env: Envelope,
    text: &str,
) -> Result<(), OccamError> {
    if envelope {
        println!(
            "{}",
            serde_json::to_string_pretty(&env)
                .map_err(|e| OccamError::DriverFailed(e.to_string()))?
        );
        return Ok(());
    }
    match mode {
        OutputMode::Text => {
            print!("{text}");
            if !text.ends_with('\n') && !text.is_empty() {
                println!();
            }
        }
        OutputMode::Json => {
            if let Some(v) = &env.result {
                println!(
                    "{}",
                    serde_json::to_string_pretty(v)
                        .map_err(|e| OccamError::DriverFailed(e.to_string()))?
                );
            } else {
                println!("{text}");
            }
        }
    }
    Ok(())
}

pub fn emit_failure_envelope(env: &Envelope) -> Result<(), OccamError> {
    println!(
        "{}",
        serde_json::to_string_pretty(env).map_err(|e| OccamError::DriverFailed(e.to_string()))?
    );
    Ok(())
}

pub fn forward_stderr(bytes: &[u8], quiet: bool) {
    if quiet || bytes.is_empty() {
        return;
    }
    use std::io::Write;
    let mut err = std::io::stderr().lock();
    let _ = err.write_all(bytes);
    if !bytes.ends_with(b"\n") {
        let _ = err.write_all(b"\n");
    }
}

pub fn dump_stderr_on_error(bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    use std::io::Write;
    let mut err = std::io::stderr().lock();
    let _ = err.write_all(bytes);
    if !bytes.ends_with(b"\n") {
        let _ = err.write_all(b"\n");
    }
}
