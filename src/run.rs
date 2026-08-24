use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::Notify;

use crate::driver::Invocation;
use crate::error::OccamError;

#[derive(Debug, Clone)]
pub struct RunLimits {
    pub timeout: Duration,
    pub max_output_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct SpawnResult {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub code: Option<i32>,
    pub duration: Duration,
    pub timed_out: bool,
    pub truncated: bool,
}

pub async fn spawn(
    invocation: &Invocation,
    cwd: &Path,
    limits: &RunLimits,
    child_stdin: Option<&[u8]>,
) -> Result<SpawnResult, OccamError> {
    let started = Instant::now();
    let mut cmd = Command::new(&invocation.program);
    cmd.args(&invocation.args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .env_remove("OCCAM_DRIVER");

    let stdin_bytes = child_stdin.or(invocation.stdin.as_deref());
    if stdin_bytes.is_some() {
        cmd.stdin(Stdio::piped());
    } else {
        cmd.stdin(Stdio::null());
    }

    #[cfg(unix)]
    {
        cmd.process_group(0);
    }

    let mut child = cmd.spawn().map_err(|e| {
        OccamError::DriverFailed(format!(
            "failed to spawn {}: {e}",
            invocation.program.display()
        ))
    })?;

    if let Some(bytes) = stdin_bytes {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(bytes).await;
            drop(stdin);
        }
    }

    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| OccamError::DriverFailed("child stdout not piped".into()))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| OccamError::DriverFailed("child stderr not piped".into()))?;

    let max = limits.max_output_bytes as usize;
    let overflow = Arc::new(Notify::new());
    let overflow_tx = overflow.clone();
    let stdout_task = tokio::spawn(async move {
        let result = read_limited(&mut stdout, max).await;
        if result.1 {
            overflow_tx.notify_one();
        }
        result
    });
    let stderr_task = tokio::spawn(async move {
        let mut buf = Vec::new();
        let _ = stderr.read_to_end(&mut buf).await;
        buf
    });

    let sleep = tokio::time::sleep(limits.timeout);
    tokio::pin!(sleep);
    let interrupt = wait_interrupt();
    tokio::pin!(interrupt);

    enum Why {
        Exited(std::process::ExitStatus),
        Timeout,
        Truncated,
        Interrupted,
    }

    let why = tokio::select! {
        status = child.wait() => {
            match status {
                Ok(s) => Why::Exited(s),
                Err(e) => return Err(OccamError::DriverFailed(format!("wait failed: {e}"))),
            }
        }
        _ = &mut sleep => Why::Timeout,
        _ = overflow.notified() => Why::Truncated,
        _ = &mut interrupt => Why::Interrupted,
    };

    let (timed_out, code) = match why {
        Why::Exited(status) => (false, status.code()),
        Why::Timeout => {
            terminate(&mut child).await;
            let _ = child.wait().await;
            (true, None)
        }
        Why::Truncated => {
            terminate(&mut child).await;
            let _ = child.wait().await;
            (false, None)
        }
        Why::Interrupted => {
            terminate(&mut child).await;
            let _ = child.wait().await;
            let _ = stdout_task.await;
            let _ = stderr_task.await;
            return Err(OccamError::Interrupted);
        }
    };

    let (stdout, truncated) = stdout_task.await.unwrap_or((Vec::new(), false));
    let stderr = stderr_task.await.unwrap_or_default();

    Ok(SpawnResult {
        stdout,
        stderr,
        code,
        duration: started.elapsed(),
        timed_out,
        truncated,
    })
}

async fn read_limited<R: AsyncReadExt + Unpin>(r: &mut R, max: usize) -> (Vec<u8>, bool) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 8192];
    loop {
        match r.read(&mut tmp).await {
            Ok(0) => return (buf, false),
            Ok(n) => {
                if buf.len() + n > max {
                    let take = max.saturating_sub(buf.len());
                    buf.extend_from_slice(&tmp[..take]);
                    return (buf, true);
                }
                buf.extend_from_slice(&tmp[..n]);
            }
            Err(_) => return (buf, false),
        }
    }
}

async fn terminate(child: &mut tokio::process::Child) {
    #[cfg(unix)]
    {
        if let Some(pid) = child.id() {
            unsafe {
                libc_kill(-(pid as i32), 15);
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
            unsafe {
                libc_kill(-(pid as i32), 9);
            }
        }
    }
    let _ = child.kill().await;
}

#[cfg(unix)]
unsafe fn libc_kill(pid: i32, sig: i32) {
    extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    let _ = kill(pid, sig);
}

async fn wait_interrupt() {
    #[cfg(unix)]
    {
        let mut sigint =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).ok();
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).ok();
        match (sigint.as_mut(), sigterm.as_mut()) {
            (Some(i), Some(t)) => {
                tokio::select! {
                    _ = i.recv() => {}
                    _ = t.recv() => {}
                }
            }
            (Some(i), None) => {
                let _ = i.recv().await;
            }
            (None, Some(t)) => {
                let _ = t.recv().await;
            }
            (None, None) => std::future::pending::<()>().await,
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
