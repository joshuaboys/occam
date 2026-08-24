mod common;
use common::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn pipe_is_read_as_task_input() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let mut cmd = h.cmd();
    cmd.arg("review").stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"secret-diff")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let log = h.log_text();
    assert!(
        log.contains("secret-diff"),
        "piped bytes must reach the prompt: {log}"
    );
    assert!(
        log.contains("STDIN:") && !log.contains("STDIN:secret-diff"),
        "child must not inherit the user pipe: {log}"
    );
}

#[test]
fn required_stdin_fails_on_tty_empty() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    // No pipe: cargo test attaches a non-tty empty stdin sometimes.
    // Force empty pipe.
    let mut cmd = h.cmd();
    cmd.arg("review").stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    drop(child.stdin.take());
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("requires stdin"));
}
