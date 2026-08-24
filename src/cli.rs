use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Stateless Unix-style execution layer for bounded, single-shot agent work.
#[derive(Debug, Parser)]
#[command(
    name = "occam",
    version,
    arg_required_else_help = true,
    after_help = "Examples:\n  git diff main...HEAD | occam review\n  cat incident.json | occam diagnose --json\n  OCCAM_DRIVER=claude git diff | occam review\n  occam drivers"
)]
pub struct Cli {
    /// Driver to invoke (codex, claude, grok). Overrides task, env, and config.
    #[arg(long, global = true)]
    pub driver: Option<String>,

    /// Emit JSON result body on stdout.
    #[arg(long, global = true)]
    pub json: bool,

    /// Wrap the result in the Occam envelope.
    #[arg(long, global = true)]
    pub envelope: bool,

    /// JSON Schema file for the result body.
    #[arg(long, global = true, value_name = "PATH")]
    pub schema: Option<PathBuf>,

    /// Working directory for the driver process.
    #[arg(long, global = true, value_name = "PATH")]
    pub cwd: Option<PathBuf>,

    /// Kill the driver after this duration (e.g. 90s).
    #[arg(long, global = true, value_name = "DURATION")]
    pub timeout: Option<String>,

    /// Extra prompt text, in addition to the task instructions and stdin.
    #[arg(long, global = true)]
    pub prompt: Option<String>,

    /// Suppress progress on stderr. Errors still print.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Option<Command>,

    /// Named task to run (from occam.toml).
    #[arg(value_name = "TASK")]
    pub task: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List local drivers and whether they look runnable.
    Drivers,
    /// List named tasks from the resolved configuration.
    Tasks,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
