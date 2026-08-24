mod common;
use common::*;
use std::fs;

#[test]
fn command_override_points_at_custom_executable() {
    let h = Harness::new();
    let custom = h.bin_dir.join("my-codex");
    fs::copy(h.bin_dir.join("codex"), &custom).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&custom, fs::Permissions::from_mode(0o755)).unwrap();
    }
    h.write_occam_toml(&format!(
        r#"
default_driver = "codex"
[drivers.codex]
command = "{}"
[task.review]
instructions = "hi"
stdin = "optional"
"#,
        custom.display()
    ));
    let out = h.cmd().args(["review", "--prompt", "x"]).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(h.log_text().contains("my-codex"));
}

#[test]
fn unknown_fallback_id_fails_validation() {
    let h = Harness::new();
    h.write_occam_toml("fallback = [\"nope\"]\n");
    let out = h.cmd().arg("drivers").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn fallback_list_is_accepted() {
    let h = Harness::new();
    h.write_occam_toml("fallback = [\"codex\", \"claude\"]\n");
    let out = h.cmd().arg("drivers").output().unwrap();
    assert!(out.status.success(), "{}", stderr(&out));
}

#[test]
fn drivers_array_is_rejected_with_fallback_hint() {
    let h = Harness::new();
    h.write_occam_toml("drivers = [\"codex\", \"claude\"]\n");
    let out = h.cmd().arg("drivers").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = stderr(&out);
    assert!(err.contains("fallback"), "{err}");
}
