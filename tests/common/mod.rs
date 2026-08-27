#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

use tempfile::TempDir;

pub fn bin() -> &'static Path {
    static P: OnceLock<PathBuf> = OnceLock::new();
    P.get_or_init(|| PathBuf::from(env!("CARGO_BIN_EXE_occam")))
}

pub struct Harness {
    pub dir: TempDir,
    pub bin_dir: PathBuf,
    pub log: PathBuf,
}

impl Harness {
    pub fn new() -> Self {
        let dir = TempDir::new().expect("tempdir");
        let config = dir.path().join("config");
        fs::create_dir_all(config.join("occam")).unwrap();
        let bin_dir = dir.path().join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let fixture =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fake-driver.sh");
        for name in ["codex", "claude", "grok"] {
            let dest = bin_dir.join(name);
            fs::copy(&fixture, &dest).unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&dest, fs::Permissions::from_mode(0o755)).unwrap();
            }
        }
        let driver_config = ["codex", "claude", "grok"]
            .iter()
            .map(|name| {
                let command = serde_json::to_string(&bin_dir.join(name).to_string_lossy())
                    .expect("driver path string");
                format!("[drivers.{name}]\ncommand = {command}\n")
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(config.join("occam/config.toml"), driver_config).unwrap();
        let log = dir.path().join("driver.log");
        Self { dir, bin_dir, log }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn write_occam_toml(&self, body: &str) {
        fs::write(self.path().join("occam.toml"), body).unwrap();
    }

    pub fn write_schema(&self, name: &str, body: &str) -> PathBuf {
        let p = self.path().join(name);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&p, body).unwrap();
        p
    }

    pub fn cmd(&self) -> Command {
        let mut c = Command::new(bin());
        c.current_dir(self.path());
        c.env("HOME", self.path());
        c.env("XDG_CONFIG_HOME", self.path().join("config"));
        c.env_remove("OCCAM_DRIVER");
        let mut paths = vec![self.bin_dir.clone()];
        if let Some(orig) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&orig));
        }
        let path = std::env::join_paths(paths).expect("test PATH");
        c.env("PATH", path);
        c.env("OCCAM_FAKE_LOG", &self.log);
        c.stdout(std::process::Stdio::piped());
        c.stderr(std::process::Stdio::piped());
        c
    }

    pub fn log_text(&self) -> String {
        fs::read_to_string(&self.log).unwrap_or_default()
    }
}

pub fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

pub fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

pub const REVIEW_TOML: &str = r#"
default_driver = "codex"
timeout = "5s"

[task.review]
instructions = "Review the supplied change."
output = "text"
stdin = "required"
timeout = "5s"
"#;
