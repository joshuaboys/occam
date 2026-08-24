use crate::driver::{
    argv_or_stdin_prompt, extract_json_text, find_executable, generic_auth_error,
    unwrap_vendor_json, Capabilities, Capability, Driver, DriverStatus, Invocation, MapRequest,
};
use crate::error::OccamError;
use crate::task::OutputMode;

pub struct CodexDriver {
    command: String,
}

impl CodexDriver {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

impl Driver for CodexDriver {
    fn id(&self) -> &'static str {
        "codex"
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
        Capabilities::new(&[
            Capability::StructuredOutput,
            Capability::StreamingOutput,
            Capability::SandboxControl,
        ])
    }

    fn login_hint(&self) -> &'static str {
        "Authenticate with the Codex CLI (codex login), then retry."
    }

    fn map(&self, req: &MapRequest<'_>) -> Result<Invocation, OccamError> {
        let program =
            find_executable(&self.command).ok_or_else(|| OccamError::DriverUnavailable {
                id: self.id().into(),
                detail: format!("'{}' not found on PATH", self.command),
            })?;

        let mut args = vec!["exec".to_string()];
        let mut prompt = req.prompt.to_string();
        if req.output == OutputMode::Json {
            prompt = format!(
                "{}\n\n{}",
                crate::prompt::json_instruction(req.schema),
                prompt
            );
        }
        let (prompt_args, stdin) = argv_or_stdin_prompt(&prompt);
        if prompt_args.is_empty() {
            args.push("-".into());
        } else {
            args.extend(prompt_args);
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
