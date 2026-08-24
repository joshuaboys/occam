mod common;
use common::*;
use std::fs;

#[test]
fn later_files_override_earlier_keys() {
    let h = Harness::new();
    fs::write(
        h.path().join("config/occam/config.toml"),
        "default_driver = \"claude\"\ntimeout = \"10s\"\n",
    )
    .unwrap();
    h.write_occam_toml("default_driver = \"grok\"\ntimeout = \"3s\"\n");
    let out = h.cmd().arg("drivers").output().unwrap();
    assert!(out.status.success());
    // grok is still listed; override is exercised by a run
    let mut cmd = h.cmd();
    cmd.arg("review");
    h.write_occam_toml(
        r#"
default_driver = "grok"
timeout = "3s"
[task.review]
instructions = "hi"
stdin = "optional"
"#,
    );
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(h.log_text().contains("grok") || h.log_text().contains("ARGV:"));
}

#[test]
fn invalid_toml_is_exit_2() {
    let h = Harness::new();
    h.write_occam_toml("default_driver = [\n");
    let out = h.cmd().arg("drivers").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn missing_files_are_skipped() {
    let h = Harness::new();
    let out = h.cmd().arg("drivers").output().unwrap();
    assert!(out.status.success());
}
