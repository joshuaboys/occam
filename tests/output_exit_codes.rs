mod common;
use common::*;
use std::fs;

#[test]
fn success_is_zero() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn usage_is_two() {
    let h = Harness::new();
    let out = h.cmd().arg("nope").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn missing_driver_is_three() {
    let h = Harness::new();
    fs::remove_file(h.bin_dir.join("codex")).unwrap();
    fs::remove_file(h.bin_dir.join("claude")).unwrap();
    fs::remove_file(h.bin_dir.join("grok")).unwrap();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
}

#[test]
fn auth_is_four() {
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
        .env("OCCAM_FAKE_AUTH", "1")
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(4), "{}", stderr(&out));
    assert!(stderr(&out).contains("codex login") || stderr(&out).contains("Authenticate"));
}

#[test]
fn driver_fail_is_five() {
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
        .env("OCCAM_FAKE_EXIT", "1")
        .args(["review", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(5), "{}", stderr(&out));
}

#[test]
fn unsupported_max_turns_on_codex_is_eight() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
max_turns = 3
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(8), "{}", stderr(&out));
}
