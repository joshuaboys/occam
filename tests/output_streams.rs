mod common;
use common::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn progress_does_not_appear_on_stdout() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let mut cmd = h.cmd();
    cmd.arg("review").stdin(Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child.stdin.as_mut().unwrap().write_all(b"diff").unwrap();
    let out = child.wait_with_output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let so = stdout(&out);
    assert!(!so.contains("diag:"), "stdout leaked diagnostics: {so:?}");
    assert!(stderr(&out).contains("diag: fake-driver"));
}

#[test]
fn quiet_hides_progress() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h
        .cmd()
        .args(["review", "--prompt", "x", "-q"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(!stderr(&out).contains("diag: fake-driver"));
}
