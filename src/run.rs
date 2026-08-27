use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::mpsc;

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
    let process_group_id = child.id();

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
    enum StreamEvent {
        Complete,
        Overflow,
    }

    let (stream_tx, mut stream_rx) = mpsc::unbounded_channel();
    let stdout_event_tx = stream_tx.clone();
    let stdout_task = tokio::spawn(async move {
        let result = read_limited(&mut stdout, max).await;
        let event = if result.1 {
            StreamEvent::Overflow
        } else {
            StreamEvent::Complete
        };
        let _ = stdout_event_tx.send(event);
        result
    });
    let stderr_event_tx = stream_tx.clone();
    let stderr_task = tokio::spawn(async move {
        let result = read_limited(&mut stderr, max).await;
        let event = if result.1 {
            StreamEvent::Overflow
        } else {
            StreamEvent::Complete
        };
        let _ = stderr_event_tx.send(event);
        result
    });
    drop(stream_tx);

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

    let mut child_status = None;
    let mut completed_streams = 0u8;
    let why = {
        let child_wait = child.wait();
        tokio::pin!(child_wait);
        loop {
            tokio::select! {
                status = &mut child_wait, if child_status.is_none() => {
                    let status = status.map_err(|e| {
                        OccamError::DriverFailed(format!("wait failed: {e}"))
                    })?;
                    if completed_streams == 2 {
                        break Why::Exited(status);
                    }
                    child_status = Some(status);
                }
                event = stream_rx.recv(), if completed_streams < 2 => {
                    match event {
                        Some(StreamEvent::Overflow) => break Why::Truncated,
                        Some(StreamEvent::Complete) => {
                            completed_streams += 1;
                            if completed_streams == 2 {
                                if let Some(status) = child_status.take() {
                                    break Why::Exited(status);
                                }
                            }
                        }
                        None => {
                            return Err(OccamError::DriverFailed(
                                "driver stream readers ended unexpectedly".into(),
                            ));
                        }
                    }
                }
                _ = &mut sleep => break Why::Timeout,
                _ = &mut interrupt => break Why::Interrupted,
            }
        }
    };

    let (timed_out, code) = match why {
        Why::Exited(status) => (false, status.code()),
        Why::Timeout => {
            terminate(&mut child, process_group_id).await;
            let _ = child.wait().await;
            (true, None)
        }
        Why::Truncated => {
            terminate(&mut child, process_group_id).await;
            let _ = child.wait().await;
            (false, None)
        }
        Why::Interrupted => {
            terminate(&mut child, process_group_id).await;
            let _ = child.wait().await;
            let _ = stdout_task.await;
            let _ = stderr_task.await;
            return Err(OccamError::Interrupted);
        }
    };

    let (stdout, stdout_truncated) = stdout_task.await.unwrap_or((Vec::new(), false));
    let (stderr, stderr_truncated) = stderr_task.await.unwrap_or((Vec::new(), false));

    Ok(SpawnResult {
        stdout,
        stderr,
        code,
        duration: started.elapsed(),
        timed_out,
        truncated: stdout_truncated || stderr_truncated,
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

async fn terminate(child: &mut tokio::process::Child, _process_group_id: Option<u32>) {
    #[cfg(unix)]
    {
        if let Some(pid) = _process_group_id {
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
