mod common;
use common::*;
use std::time::Instant;

#[test]
fn timeout_yields_exit_6_and_does_not_hang() {
    let h = Harness::new();
    h.write_occam_toml(
        r#"
[task.review]
instructions = "hi"
stdin = "optional"
timeout = "30s"
"#,
    );
    let start = Instant::now();
    let out = h
        .cmd()
        .env("OCCAM_FAKE_SLEEP", "20")
        .args(["review", "--prompt", "x", "--timeout", "1s"])
        .output()
        .unwrap();
    let elapsed = start.elapsed();
    assert_eq!(out.status.code(), Some(6), "{}", stderr(&out));
    assert!(elapsed.as_secs() < 8, "took {elapsed:?}");
    assert!(stderr(&out).contains("timed out"));
}
