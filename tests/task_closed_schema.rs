mod common;
use common::*;

#[test]
fn tools_key_is_rejected() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "x"
tools = ["shell"]
"#,
    );
    let out = h.cmd().arg("tasks").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = stderr(&out);
    assert!(
        err.contains("tools") || err.contains("unknown") || err.contains("workflow"),
        "{err}"
    );
}

#[test]
fn steps_key_is_rejected() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "x"
steps = ["a"]
"#,
    );
    let out = h.cmd().arg("tasks").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}
