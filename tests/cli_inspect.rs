mod common;
use common::*;

#[test]
fn drivers_lists_without_calling_a_model() {
    let h = Harness::new();
    let out = h.cmd().arg("drivers").output().unwrap();
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("codex"));
    assert!(text.contains("claude"));
    assert!(text.contains("grok"));
    assert!(text.contains("ready"));
    assert!(
        !h.log_text().contains("ARGV:"),
        "drivers must not invoke the CLI"
    );
}

#[test]
fn tasks_lists_named_tasks() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let out = h.cmd().arg("tasks").output().unwrap();
    assert!(out.status.success());
    assert!(stdout(&out).contains("review"));
}
