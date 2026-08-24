mod common;
use common::*;

#[test]
fn task_review_becomes_a_definition() {
    let h = Harness::new();
    h.write_occam_toml(REVIEW_TOML);
    let out = h.cmd().arg("tasks").output().unwrap();
    assert!(out.status.success());
    let line = stdout(&out);
    assert!(line.contains("review"));
    assert!(line.contains("text"));
}

#[test]
fn missing_instructions_fails() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = ""
"#,
    );
    let out = h.cmd().arg("tasks").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn invalid_output_type_fails() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "x"
output = 12
"#,
    );
    let out = h.cmd().arg("tasks").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}
