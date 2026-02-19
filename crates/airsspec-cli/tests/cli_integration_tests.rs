//! # CLI Integration Tests
//!
//! End-to-end tests that invoke the compiled `airsspec` binary and verify
//! its behavior from the user's perspective. These tests exercise:
//!
//! - Help and version output
//! - Command execution with correct exit codes
//! - Error handling for invalid input
//!
//! Each test spawns a new process via `std::process::Command` to ensure
//! the binary behaves correctly as a standalone executable.

// Layer 1: Standard library
use std::fs;
use std::path::Path;
use std::process::Command;

/// Create a `Command` pointing to the compiled `airsspec` binary.
///
/// Uses the `CARGO_BIN_EXE_airsspec` environment variable set by Cargo
/// during test compilation to locate the binary.
fn airsspec_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_airsspec"))
}

#[test]
fn test_help_output() {
    let output = airsspec_cmd()
        .arg("--help")
        .output()
        .expect("failed to execute airsspec --help");

    assert!(
        output.status.success(),
        "airsspec --help should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("lightweight"),
        "help output should contain 'lightweight'"
    );
    assert!(
        stdout.contains("init"),
        "help output should list 'init' command"
    );
    assert!(
        stdout.contains("mcp"),
        "help output should list 'mcp' command"
    );
    assert!(
        stdout.contains("validate"),
        "help output should list 'validate' command"
    );
}

#[test]
fn test_version_output() {
    let output = airsspec_cmd()
        .arg("--version")
        .output()
        .expect("failed to execute airsspec --version");

    assert!(
        output.status.success(),
        "airsspec --version should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("0.1.0"),
        "version output should contain the version number"
    );
}

#[test]
fn test_init_command_no_tty() {
    // `airsspec init` requires an interactive terminal for the TUI wizard.
    // When run without a TTY (as in CI / test subprocess), the wizard fails
    // to initialize the terminal, producing a non-zero exit code.
    let output = airsspec_cmd()
        .arg("init")
        .output()
        .expect("failed to execute airsspec init");

    assert!(
        !output.status.success(),
        "airsspec init without a TTY should exit with non-zero code"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("TUI wizard failed"),
        "error output should mention TUI wizard failure, got: {stderr}"
    );
}

#[test]
fn test_mcp_command() {
    let output = airsspec_cmd()
        .arg("mcp")
        .output()
        .expect("failed to execute airsspec mcp");

    assert!(
        output.status.success(),
        "airsspec mcp should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("MCP Server"),
        "mcp output should contain 'MCP Server'"
    );
}

#[test]
fn test_mcp_debug_flag() {
    let output = airsspec_cmd()
        .args(["mcp", "--debug"])
        .output()
        .expect("failed to execute airsspec mcp --debug");

    assert!(
        output.status.success(),
        "airsspec mcp --debug should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Debug mode: true"),
        "mcp --debug output should show debug mode as true"
    );
}

/// Creates a minimal valid `.airsspec/` workspace structure in `dir`.
fn create_valid_workspace(dir: &Path) {
    let airsspec = dir.join(".airsspec");
    fs::create_dir_all(airsspec.join("specs")).unwrap();
    fs::create_dir_all(airsspec.join("logs")).unwrap();
    fs::write(
        airsspec.join("config.toml"),
        "[project]\nname = \"test\"\ndescription = \"test project\"\n",
    )
    .unwrap();
}

/// Creates a valid spec YAML file in the specs directory.
///
/// Writes raw YAML matching the `serde_yaml`-serialized `Spec` structure.
/// This avoids depending on `airsspec-mcp` types in subprocess integration
/// tests. The file is named following the `{spec-id}.yaml` convention.
fn create_test_spec_yaml(specs_dir: &Path) {
    let spec_yaml = "\
id: 1000000-test-feature
metadata:
  title: Test Feature
  description: A test feature for validation
  category: feature
  dependencies: []
  created_at: '2026-01-01T00:00:00Z'
  updated_at: '2026-01-01T00:00:00Z'
content: '# Test Feature\n\nThis is test content.'
";
    fs::write(specs_dir.join("1000000-test-feature.yaml"), spec_yaml).unwrap();
}

#[test]
fn test_validate_valid_workspace() {
    let temp = tempfile::tempdir().unwrap();
    create_valid_workspace(temp.path());
    create_test_spec_yaml(&temp.path().join(".airsspec/specs"));

    let output = airsspec_cmd()
        .arg("validate")
        .current_dir(temp.path())
        .output()
        .expect("failed to execute airsspec validate");

    assert!(
        output.status.success(),
        "airsspec validate in valid workspace should exit with code 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // With a spec present and no issues, the reporter shows "No issues found"
    // (empty report) or the status line with "PASSED"
    assert!(
        stdout.contains("No issues found") || stdout.contains("PASSED"),
        "output should indicate validation passed, got: {stdout}",
    );
}

#[test]
fn test_validate_no_workspace() {
    let temp = tempfile::tempdir().unwrap();

    let output = airsspec_cmd()
        .arg("validate")
        .current_dir(temp.path())
        .output()
        .expect("failed to execute airsspec validate");

    assert!(
        !output.status.success(),
        "airsspec validate outside workspace should exit with non-zero code",
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("validation failed"),
        "stderr should mention validation failure, got: {stderr}",
    );
}

#[test]
fn test_validate_empty_workspace() {
    let temp = tempfile::tempdir().unwrap();
    create_valid_workspace(temp.path());
    // No specs added -- workspace is valid but empty

    let output = airsspec_cmd()
        .arg("validate")
        .current_dir(temp.path())
        .output()
        .expect("failed to execute airsspec validate");

    assert!(
        output.status.success(),
        "airsspec validate in empty workspace should exit with code 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No issues found"),
        "empty workspace should show 'No issues found', got: {stdout}",
    );
}

#[test]
fn test_unknown_command_fails() {
    let output = airsspec_cmd()
        .arg("unknown")
        .output()
        .expect("failed to execute airsspec unknown");

    assert!(
        !output.status.success(),
        "airsspec unknown should exit with non-zero code"
    );
}

#[test]
fn test_no_command_fails() {
    let output = airsspec_cmd()
        .output()
        .expect("failed to execute airsspec with no args");

    assert!(
        !output.status.success(),
        "airsspec with no subcommand should exit with non-zero code"
    );
}

#[test]
fn test_init_help() {
    let output = airsspec_cmd()
        .args(["init", "--help"])
        .output()
        .expect("failed to execute airsspec init --help");

    assert!(
        output.status.success(),
        "airsspec init --help should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Initialize"),
        "init --help should contain 'Initialize'"
    );
}

#[test]
fn test_mcp_help() {
    let output = airsspec_cmd()
        .args(["mcp", "--help"])
        .output()
        .expect("failed to execute airsspec mcp --help");

    assert!(
        output.status.success(),
        "airsspec mcp --help should exit with code 0"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--debug"),
        "mcp --help should describe the --debug flag"
    );
}
