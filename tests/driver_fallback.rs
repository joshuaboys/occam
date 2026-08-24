mod common;
use common::*;
use std::fs;

#[test]
fn missing_default_falls_back_to_ready_driver() {
    let h = Harness::new();
    fs::remove_file(h.bin_dir.join("codex")).unwrap();
    h.write_occam_toml(
        r#"
default_driver = "codex"
fallback = ["codex", "claude"]
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(stderr(&out).contains("falling back to 'claude'"));
    assert!(h.log_text().contains("claude"));
}

#[test]
fn mid_run_failure_does_not_rotate() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
default_driver = "codex"
fallback = ["codex", "claude"]
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h
        .cmd()
        .env("OCCAM_FAKE_EXIT", "1")
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(5), "{}", stderr(&out));
    let log = h.log_text();
    assert!(log.contains("codex"));
    assert!(!log.contains("claude"));
}
