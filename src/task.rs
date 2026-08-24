use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;
use serde_json::Value;

use crate::error::OccamError;

const FORBIDDEN_HINTS: &[&str] = &[
    "steps",
    "graph",
    "agents",
    "agent",
    "tools",
    "tool",
    "memory",
    "workflow",
    "retries",
    "dag",
    "skill",
    "skills",
    "provider",
    "model",
    "mcp",
    "session",
    "orchestrat",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StdinMode {
    #[default]
    Optional,
    Required,
    Forbidden,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDef {
    pub instructions: String,
    pub driver: Option<String>,
    #[serde(default)]
    pub stdin: StdinMode,
    #[serde(default)]
    pub output: OutputMode,
    pub schema: Option<String>,
    pub cwd: Option<String>,
    pub timeout: Option<String>,
    pub max_turns: Option<u32>,
    pub max_output: Option<toml::Value>,
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub def: TaskDef,
    /// Directory of the file that defined this task, for relative schema paths.
    pub origin_dir: PathBuf,
}

impl Task {
    pub fn timeout(&self) -> Result<Option<Duration>, OccamError> {
        match &self.def.timeout {
            Some(s) => parse_duration(s).map(Some),
            None => Ok(None),
        }
    }

    pub fn max_output_bytes(&self) -> Result<Option<u64>, OccamError> {
        match &self.def.max_output {
            Some(v) => parse_bytes_value(v).map(Some),
            None => Ok(None),
        }
    }

    pub fn load_schema(&self) -> Result<Option<Value>, OccamError> {
        let Some(raw) = &self.def.schema else {
            return Ok(None);
        };
        load_schema_spec(raw, &self.origin_dir)
    }
}

pub fn load_schema_spec(raw: &str, origin_dir: &Path) -> Result<Option<Value>, OccamError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if trimmed.starts_with('{') {
        let value: Value = serde_json::from_str(trimmed)
            .map_err(|e| OccamError::usage(format!("invalid inline schema: {e}")))?;
        return Ok(Some(value));
    }
    let path = if Path::new(trimmed).is_absolute() {
        PathBuf::from(trimmed)
    } else {
        origin_dir.join(trimmed)
    };
    let text = std::fs::read_to_string(&path)
        .map_err(|e| OccamError::usage(format!("cannot read schema {}: {e}", path.display())))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| OccamError::usage(format!("invalid schema {}: {e}", path.display())))?;
    Ok(Some(value))
}

pub fn parse_duration(s: &str) -> Result<Duration, OccamError> {
    humantime::parse_duration(s)
        .map_err(|e| OccamError::usage(format!("invalid duration '{s}': {e}")))
}

pub fn parse_bytes_value(v: &toml::Value) -> Result<u64, OccamError> {
    match v {
        toml::Value::Integer(n) if *n >= 0 => Ok(*n as u64),
        toml::Value::String(s) => parse_bytes(s),
        _ => Err(OccamError::usage(
            "max_output must be a byte count or size string",
        )),
    }
}

pub fn parse_bytes(s: &str) -> Result<u64, OccamError> {
    let t = s.trim().to_ascii_lowercase();
    let (num, mul) = if let Some(rest) = t.strip_suffix("kib") {
        (rest, 1024u64)
    } else if let Some(rest) = t.strip_suffix("mib") {
        (rest, 1024 * 1024)
    } else if let Some(rest) = t.strip_suffix("gib") {
        (rest, 1024 * 1024 * 1024)
    } else if let Some(rest) = t.strip_suffix("kb") {
        (rest, 1000)
    } else if let Some(rest) = t.strip_suffix("mb") {
        (rest, 1000 * 1000)
    } else if let Some(rest) = t.strip_suffix('k') {
        (rest, 1024)
    } else if let Some(rest) = t.strip_suffix('m') {
        (rest, 1024 * 1024)
    } else {
        (t.as_str(), 1)
    };
    let n: u64 = num
        .trim()
        .parse()
        .map_err(|_| OccamError::usage(format!("invalid size '{s}'")))?;
    Ok(n.saturating_mul(mul))
}

pub fn reject_forbidden_keys(raw: &str) -> Result<(), OccamError> {
    // Catch DSL-shaped keys even if they never reach serde's deny_unknown_fields
    // (e.g. a top-level `tools` table). Task-level unknown fields are denied by serde.
    for key in FORBIDDEN_HINTS {
        let needle = format!("{key} =");
        let table = format!("[{key}");
        if raw.lines().any(|l| {
            let l = l.trim();
            l.starts_with(&needle) || l.starts_with(&table)
        }) {
            return Err(OccamError::usage(format!(
                "task files cannot define '{key}' — Occam tasks are not a workflow DSL"
            )));
        }
    }
    Ok(())
}

pub fn parse_task_table(
    value: toml::Value,
    origin_dir: &Path,
) -> Result<BTreeMap<String, Task>, OccamError> {
    let table = match value.get("task") {
        Some(toml::Value::Table(t)) => t,
        _ => return Ok(BTreeMap::new()),
    };
    let mut out = BTreeMap::new();
    for (name, raw) in table {
        let def: TaskDef = raw
            .clone()
            .try_into()
            .map_err(|e| OccamError::usage(format!("invalid task '{name}': {e}")))?;
        if def.instructions.trim().is_empty() {
            return Err(OccamError::usage(format!(
                "task '{name}' is missing instructions"
            )));
        }
        out.insert(
            name.clone(),
            Task {
                name: name.clone(),
                def,
                origin_dir: origin_dir.to_path_buf(),
            },
        );
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_90s() {
        assert_eq!(parse_duration("90s").unwrap(), Duration::from_secs(90));
    }

    #[test]
    fn parse_bytes_mib() {
        assert_eq!(parse_bytes("1MiB").unwrap(), 1024 * 1024);
    }
}
