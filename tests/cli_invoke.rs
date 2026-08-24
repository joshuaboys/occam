mod common;
use common::*;

#[test]
fn help_lists_invocation() {
    let out = common::Harness::new().cmd().arg("--help").output().unwrap();
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("occam"));
    assert!(text.contains("--driver"));
    assert!(text.contains("TASK") || text.contains("<TASK>"));
}

#[test]
fn unknown_flag_is_usage_error() {
    let out = Harness::new()
        .cmd()
        .args(["review", "--nope"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn unknown_task_is_usage_error() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let out = h.cmd().arg("missing").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("unknown task"));
}

#[test]
fn named_task_runs() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let mut cmd = h.cmd();
    cmd.arg("review").arg("--prompt").arg("n/a");
    use std::process::Stdio;
    cmd.stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    {
        use std::io::Write;
        child.stdin.as_mut().unwrap().write_all(b"diff").unwrap();
    }
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "stderr={}", stderr(&out));
    assert_eq!(stdout(&out).trim(), "ok");
}
