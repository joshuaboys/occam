use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::OccamError;
use crate::task::{parse_task_table, reject_forbidden_keys, Task};

const DEFAULT_DRIVER: &str = "codex";
const DEFAULT_TIMEOUT: &str = "120s";
const DEFAULT_MAX_OUTPUT: u64 = 8 * 1024 * 1024;

fn empty_value() -> toml::Value {
    toml::Value::Table(toml::map::Map::new())
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DriverOverride {
    pub command: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct FileConfig {
    default_driver: Option<String>,
    /// Unavailable-fallback list. Named `fallback` because TOML cannot use
    /// `drivers = [...]` alongside `[drivers.codex]`.
    fallback: Option<Vec<String>>,
    timeout: Option<String>,
    max_output: Option<toml::Value>,
    #[serde(default)]
    drivers: BTreeMap<String, DriverOverride>,
    #[serde(default = "empty_value")]
    task: toml::Value,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub default_driver: String,
    pub fallback: Vec<String>,
    pub timeout: String,
    pub max_output_bytes: u64,
    pub driver_commands: BTreeMap<String, String>,
    pub tasks: BTreeMap<String, Task>,
}

impl Default for Config {
    fn default() -> Self {
        let mut driver_commands = BTreeMap::new();
        driver_commands.insert("codex".into(), "codex".into());
        driver_commands.insert("claude".into(), "claude".into());
        driver_commands.insert("grok".into(), "grok".into());
        Self {
            default_driver: DEFAULT_DRIVER.into(),
            fallback: Vec::new(),
            timeout: DEFAULT_TIMEOUT.into(),
            max_output_bytes: DEFAULT_MAX_OUTPUT,
            driver_commands,
            tasks: BTreeMap::new(),
        }
    }
}

impl Config {
    pub fn load(cwd: &Path) -> Result<Self, OccamError> {
        let mut cfg = Config::default();
        for (path, tasks_only) in config_files(cwd) {
            if !path.is_file() {
                continue;
            }
            merge_file(&mut cfg, &path, tasks_only)?;
        }
        Ok(cfg)
    }

    pub fn driver_command(&self, id: &str) -> String {
        self.driver_commands
            .get(id)
            .cloned()
            .unwrap_or_else(|| id.to_string())
    }
}

fn config_files(cwd: &Path) -> Vec<(PathBuf, bool)> {
    let mut files = Vec::new();
    if let Some(dir) = dirs::config_dir() {
        let occam = dir.join("occam");
        files.push((occam.join("config.toml"), false));
        files.push((occam.join("tasks.toml"), true));
    }
    files.push((cwd.join("occam.toml"), false));
    files.push((cwd.join(".occam.toml"), false));
    files
}

fn merge_file(cfg: &mut Config, path: &Path, tasks_only: bool) -> Result<(), OccamError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| OccamError::usage(format!("cannot read {}: {e}", path.display())))?;
    reject_forbidden_keys(&text)?;
    reject_drivers_array(&text, path)?;
    let parsed: FileConfig =
        toml::from_str(&text).map_err(|e| OccamError::usage(format!("{}: {e}", path.display())))?;
    let origin = path.parent().unwrap_or(path).to_path_buf();

    if !tasks_only {
        if let Some(d) = parsed.default_driver {
            cfg.default_driver = d;
        }
        if let Some(f) = parsed.fallback {
            for id in &f {
                if !matches!(id.as_str(), "codex" | "claude" | "grok")
                    && !cfg.driver_commands.contains_key(id)
                {
                    return Err(OccamError::usage(format!(
                        "unknown driver id '{id}' in fallback"
                    )));
                }
            }
            cfg.fallback = f;
        }
        if let Some(t) = parsed.timeout {
            cfg.timeout = t;
        }
        if let Some(m) = parsed.max_output {
            cfg.max_output_bytes = crate::task::parse_bytes_value(&m)?;
        }
        for (id, over) in parsed.drivers {
            if let Some(cmd) = over.command {
                cfg.driver_commands.insert(id, cmd);
            }
        }
    }

    let wrapped = toml::Value::Table({
        let mut t = toml::map::Map::new();
        t.insert("task".into(), parsed.task);
        t
    });
    let tasks = parse_task_table(wrapped, &origin)?;
    for (name, task) in tasks {
        cfg.tasks.insert(name, task);
    }
    Ok(())
}

/// TOML cannot attach both `drivers = [...]` and `[drivers.codex]` to the same key.
fn reject_drivers_array(text: &str, path: &Path) -> Result<(), OccamError> {
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with('#') {
            continue;
        }
        let Some(rest) = l.strip_prefix("drivers") else {
            continue;
        };
        let rest = rest.trim_start();
        if let Some(value) = rest.strip_prefix('=') {
            let value = value.trim_start();
            if value.starts_with('[') {
                return Err(OccamError::usage(format!(
                    "{}: unavailable-fallback is `fallback = [\"codex\", \"claude\"]`; `drivers` is reserved for `[drivers.codex]` command overrides",
                    path.display()
                )));
            }
        }
    }
    Ok(())
}

/// `--driver` → task → OCCAM_DRIVER → config default → codex
pub fn resolve_driver_id(
    flag: Option<&str>,
    task_driver: Option<&str>,
    config_default: &str,
) -> String {
    if let Some(s) = flag.filter(|s| !s.is_empty()) {
        return s.to_string();
    }
    if let Some(s) = task_driver.filter(|s| !s.is_empty()) {
        return s.to_string();
    }
    if let Ok(s) = std::env::var("OCCAM_DRIVER") {
        let s = s.trim();
        if !s.is_empty() {
            return s.to_string();
        }
    }
    config_default.to_string()
}
