use std::process::ExitCode;

/// Stable Occam exit codes. Independent of the underlying driver.
pub const EXIT_OK: u8 = 0;
pub const EXIT_USAGE: u8 = 2;
pub const EXIT_DRIVER_UNAVAILABLE: u8 = 3;
pub const EXIT_AUTH: u8 = 4;
pub const EXIT_DRIVER_FAILED: u8 = 5;
pub const EXIT_TIMEOUT: u8 = 6;
pub const EXIT_VALIDATION: u8 = 7;
pub const EXIT_UNSUPPORTED: u8 = 8;
pub const EXIT_INTERRUPTED: u8 = 130;

#[derive(Debug, thiserror::Error)]
pub enum OccamError {
    #[error("{0}")]
    Usage(String),

    #[error("driver '{id}' is unavailable: {detail}")]
    DriverUnavailable { id: String, detail: String },

    #[error("{detail}")]
    Auth {
        id: String,
        detail: String,
        hint: String,
    },

    #[error("{0}")]
    DriverFailed(String),

    #[error("timed out after {0}")]
    Timeout(String),

    #[error("structured output failed validation: {0}")]
    Validation(String),

    #[error("unsupported capability '{capability}' on driver '{id}'")]
    Unsupported { id: String, capability: String },

    #[error("interrupted")]
    Interrupted,

    #[error("{0}")]
    Io(String),
}

impl OccamError {
    pub fn usage(msg: impl Into<String>) -> Self {
        Self::Usage(msg.into())
    }

    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Usage(_) => EXIT_USAGE,
            Self::DriverUnavailable { .. } => EXIT_DRIVER_UNAVAILABLE,
            Self::Auth { .. } => EXIT_AUTH,
            Self::DriverFailed(_) => EXIT_DRIVER_FAILED,
            Self::Timeout(_) => EXIT_TIMEOUT,
            Self::Validation(_) => EXIT_VALIDATION,
            Self::Unsupported { .. } => EXIT_UNSUPPORTED,
            Self::Interrupted => EXIT_INTERRUPTED,
            Self::Io(_) => EXIT_DRIVER_FAILED,
        }
    }

    pub fn as_exit_code(&self) -> ExitCode {
        ExitCode::from(self.exit_code())
    }

    pub fn envelope_status(&self) -> &'static str {
        match self {
            Self::Timeout(_) => "timeout",
            Self::Validation(_) => "validation_failed",
            Self::Unsupported { .. } => "unsupported",
            Self::Interrupted => "driver_failed",
            Self::Usage(_)
            | Self::DriverUnavailable { .. }
            | Self::Auth { .. }
            | Self::DriverFailed(_)
            | Self::Io(_) => "driver_failed",
        }
    }
}

impl From<std::io::Error> for OccamError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

pub fn print_error(err: &OccamError) {
    if matches!(err, OccamError::Interrupted) {
        return;
    }
    eprint!("occam: {err}");
    if let OccamError::Auth { hint, .. } = err {
        eprintln!();
        eprintln!("{hint}");
    } else {
        eprintln!();
    }
}
