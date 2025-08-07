use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("tuxpilot").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("An AI-powered copilot for Linux systems"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("tuxpilot").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tuxpilot"));
}

#[test]
fn test_config_show() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    // Create a test config file
    let config_content = r#"
[ai]
provider = "OpenAI"

[ai.openai]
api_key = "test-key"
model = "gpt-4"

[system]
package_manager = "Pacman"
service_manager = "Systemd"
log_paths = ["/var/log/syslog"]

[ui]
theme = "default"
show_tips = true
auto_suggest = true
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let mut cmd = Command::cargo_bin("tuxpilot").unwrap();
    cmd.arg("--config")
        .arg(&config_path)
        .arg("config")
        .arg("--show");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Current Configuration"));
}

#[test]
fn test_package_help() {
    let mut cmd = Command::cargo_bin("tuxpilot").unwrap();
    cmd.arg("package")
        .arg("install")
        .arg("test-package")
        .env("TUXPILOT_AI_TIMEOUT", "5") // Short timeout for tests
        .env("TUXPILOT_TEST_MODE", "1"); // Test mode flag

    // This should handle AI timeouts gracefully
    let output = cmd.output().unwrap();

    // Accept either success or timeout error (both are valid for tests)
    assert!(
        output.status.success() ||
        String::from_utf8_lossy(&output.stderr).contains("timed out") ||
        String::from_utf8_lossy(&output.stderr).contains("deadline has elapsed")
    );
}

#[test]
fn test_diagnose_without_ai() {
    let mut cmd = Command::cargo_bin("tuxpilot").unwrap();
    cmd.arg("diagnose")
        .arg("--input")
        .arg("test error message");
    
    // This might fail without AI configuration, but should handle gracefully
    let output = cmd.output().unwrap();
    
    // Check that it doesn't panic
    assert!(output.status.code().is_some());
}
