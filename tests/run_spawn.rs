mod common;
use common::*;
use std::io::Write;
use std::process::Stdio;
use std::time::{Duration, Instant};

#[test]
fn child_cwd_is_resolved() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let nested = h.path().join("work");
    std::fs::create_dir(&nested).unwrap();
    let mut cmd = h.cmd();
    cmd.args(["review", "--cwd"])
        .arg(&nested)
        .stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child.stdin.as_mut().unwrap().write_all(b"diff").unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
}

#[test]
fn piped_bytes_are_not_child_stdin() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let mut cmd = h.cmd();
    cmd.arg("review").stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"payload-xyz")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let log = h.log_text();
    assert!(log.contains("payload-xyz"));
    assert!(!log.contains("STDIN:payload-xyz"));
}

#[test]
fn max_output_fails_closed() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
max_output = 20
"#,
    );
    let out = h
        .cmd()
        .env("OCCAM_FAKE_STDOUT", "this is more than eight bytes")
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(5), "{}", stderr(&out));
    assert!(stderr(&out).contains("max_output"));
}

#[test]
fn max_output_applies_to_stderr() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
max_output = 20
"#,
    );
    let out = h
        .cmd()
        .env(
            "OCCAM_FAKE_STDERR",
            "this is more than twenty bytes on stderr",
        )
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(5), "{}", stderr(&out));
    assert!(stderr(&out).contains("max_output"));
}

#[test]
fn descendant_overflow_tears_down_process_group_promptly() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
max_output = 20
"#,
    );
    let started = Instant::now();
    let out = h
        .cmd()
        .env(
            "OCCAM_FAKE_DESCENDANT_STDERR",
            "this is more than twenty bytes from a descendant",
        )
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    let elapsed = started.elapsed();
    assert_eq!(out.status.code(), Some(5), "{}", stderr(&out));
    assert!(stderr(&out).contains("max_output"));
    assert!(
        elapsed < Duration::from_millis(1500),
        "process group teardown took {elapsed:?}"
    );
}
