use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::error::OccamError;
use crate::task::OutputMode;

pub mod claude;
pub mod codex;
pub mod grok;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Missing,
    Unauthenticated,
    Ready,
}

impl DriverStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "unavailable",
            Self::Unauthenticated => "unauthenticated",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Capability {
    StructuredOutput,
    SchemaOutput,
    StreamingOutput,
    MaxTurns,
    ToolAllowlist,
    SandboxControl,
    EphemeralMode,
}

impl Capability {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "structured_output" => Some(Self::StructuredOutput),
            "schema_output" => Some(Self::SchemaOutput),
            "streaming_output" => Some(Self::StreamingOutput),
            "max_turns" => Some(Self::MaxTurns),
            "tool_allowlist" => Some(Self::ToolAllowlist),
            "sandbox_control" => Some(Self::SandboxControl),
            "ephemeral_mode" => Some(Self::EphemeralMode),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::StructuredOutput => "structured_output",
            Self::SchemaOutput => "schema_output",
            Self::StreamingOutput => "streaming_output",
            Self::MaxTurns => "max_turns",
            Self::ToolAllowlist => "tool_allowlist",
            Self::SandboxControl => "sandbox_control",
            Self::EphemeralMode => "ephemeral_mode",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Capabilities {
    inner: BTreeSet<Capability>,
}

impl Capabilities {
    pub fn new(caps: &[Capability]) -> Self {
        Self {
            inner: caps.iter().copied().collect(),
        }
    }

    pub fn contains(&self, cap: Capability) -> bool {
        self.inner.contains(&cap)
    }

    pub fn list(&self) -> Vec<&'static str> {
        self.inner.iter().map(|c| c.as_str()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Invocation {
    pub program: PathBuf,
    pub args: Vec<String>,
    /// Bytes piped to the child. `None` means stdin is /dev/null.
    pub stdin: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct MapRequest<'a> {
    pub prompt: &'a str,
    pub output: OutputMode,
    pub schema: Option<&'a Value>,
    pub max_turns: Option<u32>,
}

pub trait Driver: Send + Sync {
    fn id(&self) -> &'static str;
    fn command(&self) -> &str;
    fn detect(&self) -> DriverStatus;
    fn capabilities(&self) -> Capabilities;
    fn login_hint(&self) -> &'static str;
    fn map(&self, req: &MapRequest<'_>) -> Result<Invocation, OccamError>;
    fn normalize_stdout(&self, raw: &[u8], output: OutputMode) -> Result<String, OccamError>;
    fn looks_like_auth_error(&self, stderr: &str, code: Option<i32>) -> bool;
}

pub fn builtin_ids() -> &'static [&'static str] {
    &["codex", "claude", "grok"]
}

pub fn make_driver(id: &str, command: String) -> Result<Box<dyn Driver>, OccamError> {
    match id {
        "codex" => Ok(Box::new(codex::CodexDriver::new(command))),
        "claude" => Ok(Box::new(claude::ClaudeDriver::new(command))),
        "grok" => Ok(Box::new(grok::GrokDriver::new(command))),
        other => Err(OccamError::usage(format!(
            "unknown driver '{other}' (expected codex, claude, or grok)"
        ))),
    }
}

pub fn find_executable(command: &str) -> Option<PathBuf> {
    let path = Path::new(command);
    if command.contains('/') || command.contains('\\') || path.is_absolute() {
        return is_runnable(path).then(|| path.to_path_buf());
    }
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(command);
        if is_runnable(&candidate) {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            for ext in ["exe", "cmd", "bat"] {
                let with = dir.join(format!("{command}.{ext}"));
                if is_runnable(&with) {
                    return Some(with);
                }
            }
        }
    }
    None
}

fn is_runnable(path: &Path) -> bool {
    let Ok(meta) = std::fs::metadata(path) else {
        return false;
    };
    if !meta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

pub fn generic_auth_error(stderr: &str) -> bool {
    let s = stderr.to_ascii_lowercase();
    const NEEDLES: &[&str] = &[
        "not logged in",
        "not authenticated",
        "please log in",
        "please login",
        "run `codex login",
        "login required",
        "unauthenticated",
        "authentication required",
        "auth required",
        "not signed in",
    ];
    NEEDLES.iter().any(|n| s.contains(n))
}

pub fn extract_json_text(raw: &str) -> String {
    let t = raw.trim();
    if let Some(rest) = t.strip_prefix("```json") {
        if let Some(end) = rest.rfind("```") {
            return rest[..end].trim().to_string();
        }
    }
    if let Some(rest) = t.strip_prefix("```") {
        if let Some(end) = rest.rfind("```") {
            return rest[..end].trim().to_string();
        }
    }
    t.to_string()
}

/// If stdout is a JSON object with a well-known wrapper, pull the inner result.
pub fn unwrap_vendor_json(text: &str) -> Result<String, OccamError> {
    let cleaned = extract_json_text(text);
    let Ok(value) = serde_json::from_str::<Value>(&cleaned) else {
        return Ok(cleaned);
    };
    if let Some(inner) = value.get("structured_output") {
        return Ok(inner.to_string());
    }
    if let Some(result) = value.get("result") {
        return Ok(match result {
            Value::String(s) => {
                if serde_json::from_str::<Value>(s).is_ok() {
                    s.clone()
                } else {
                    cleaned
                }
            }
            other => other.to_string(),
        });
    }
    Ok(cleaned)
}

pub fn argv_or_stdin_prompt(prompt: &str) -> (Vec<String>, Option<Vec<u8>>) {
    const MAX_ARG: usize = 100 * 1024;
    if prompt.len() <= MAX_ARG {
        (vec![prompt.to_string()], None)
    } else {
        (Vec::new(), Some(prompt.as_bytes().to_vec()))
    }
}
