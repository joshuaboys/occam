mod common;
use common::*;

const SCHEMA: &str = r#"{
  "type": "object",
  "additionalProperties": false,
  "required": ["cause"],
  "properties": { "cause": { "type": "string", "minLength": 1 } }
}"#;

#[test]
fn valid_json_body_on_success() {
    let h = Harness::new();
    h.write_schema("schema.json", SCHEMA);
    h.write_occam_toml(
        r#"
[task.diagnose]
instructions = "Diagnose"
output = "json"
schema = "schema.json"
stdin = "optional"
requires = ["structured_output"]
"#,
    );
    let out = h
        .cmd()
        .env("OCCAM_FAKE_STDOUT", r#"{"cause":"disk"}"#)
        .args(["diagnose", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let v: serde_json::Value = serde_json::from_str(&stdout(&out)).unwrap();
    assert_eq!(v["cause"], "disk");
}

#[test]
fn invalid_then_still_invalid_is_exit_7() {
    let h = Harness::new();
    h.write_schema("schema.json", SCHEMA);
    h.write_occam_toml(
        r#"
[task.diagnose]
instructions = "Diagnose"
output = "json"
schema = "schema.json"
stdin = "optional"
"#,
    );
    let out = h
        .cmd()
        .env("OCCAM_FAKE_STDOUT", "not-json")
        .args(["diagnose", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(7), "{}", stderr(&out));
    assert!(
        !stdout(&out).contains("not-json")
            || stdout(&out).is_empty()
            || stdout(&out).contains("status")
    );
}

#[test]
fn repair_succeeds_on_second_attempt() {
    let h = Harness::new();
    h.write_schema("schema.json", SCHEMA);
    h.write_occam_toml(
        r#"
[task.diagnose]
instructions = "Diagnose"
output = "json"
schema = "schema.json"
stdin = "optional"
"#,
    );
    let counter = h.path().join("counter");
    let out = h
        .cmd()
        .env("OCCAM_FAKE_COUNTER", &counter)
        .env("OCCAM_FAKE_STDOUT_FIRST", "nope")
        .env("OCCAM_FAKE_STDOUT", r#"{"cause":"ok"}"#)
        .args(["diagnose", "--prompt", "x"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    let v: serde_json::Value = serde_json::from_str(&stdout(&out)).unwrap();
    assert_eq!(v["cause"], "ok");
}
