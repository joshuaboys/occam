mod common;
use common::*;
use std::fs;

#[test]
fn missing_binary_is_unavailable() {
    let h = Harness::new();
    fs::remove_file(h.bin_dir.join("claude")).unwrap();
    let out = h.cmd().arg("drivers").output().unwrap();
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("codex") && text.contains("ready"));
    assert!(
        text.lines()
            .any(|l| l.starts_with("claude") && l.contains("unavailable")),
        "{text}"
    );
}

#[test]
fn ready_binaries_do_not_invoke_a_model() {
    let h = Harness::new();
    let _ = h.cmd().arg("drivers").output().unwrap();
    assert!(h.log_text().is_empty());
}
