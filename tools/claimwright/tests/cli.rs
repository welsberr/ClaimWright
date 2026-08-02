use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_claimwright")
}

fn temporary_root() -> std::path::PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("claimwright-cli-test-{suffix}"));
    fs::create_dir(&root).expect("temporary test directory");
    root
}

#[test]
fn check_command_preserves_current_success_output() {
    let manifest_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_root
        .parent()
        .and_then(|path| path.parent())
        .expect("repository root");
    let output = Command::new(binary())
        .args(["check", root.to_str().expect("repository path")])
        .output()
        .expect("run claimwright");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "ClaimWright check passed: policy substrate is present.\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn check_command_preserves_missing_file_failure() {
    let root = temporary_root();
    let output = Command::new(binary())
        .args(["check", root.to_str().expect("temporary path")])
        .output()
        .expect("run claimwright");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("error: missing required file: MOU.md")
    );
    fs::remove_dir(&root).expect("remove temporary test directory");
}

#[test]
fn invalid_arguments_preserve_usage_exit_code() {
    let output = Command::new(binary())
        .arg("publication")
        .output()
        .expect("run claimwright");

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "usage: claimwright check <repo-root>\n"
    );
}
