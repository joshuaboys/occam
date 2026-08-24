mod common;
use common::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn codex_uses_exec() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let mut cmd = h.cmd();
    cmd.arg("review").stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child.stdin.as_mut().unwrap().write_all(b"diff").unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(h.log_text().contains(" exec "), "{}", h.log_text());
}

#[test]
fn claude_uses_print_mode() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
default_driver = "claude"
[task.review]
instructions = "Review"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let log = h.log_text();
    assert!(
        log.contains(" -p ") || log.contains(" -p\n") || log.contains(" -p"),
        "{log}"
    );
}

#[test]
fn grok_uses_probed_noninteractive_entry() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
default_driver = "grok"
[task.review]
instructions = "Review"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let log = h.log_text();
    assert!(
        log.contains(" exec ") || log.contains(" --prompt ") || log.contains(" -p "),
        "{log}"
    );
}
