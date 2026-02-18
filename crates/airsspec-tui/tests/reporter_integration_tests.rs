//! Integration tests for the TUI validation reporter.
//!
//! These tests verify cross-crate integration between `airsspec_core`
//! (which provides `ValidationReport` and `ValidationIssue`) and
//! `airsspec_tui` (which provides `render_validation_report`).

use airsspec_core::validation::{ValidationIssue, ValidationReport};
use airsspec_tui::render_validation_report;

/// Helper: render report to string for assertions.
fn render_to_string(report: &ValidationReport) -> String {
    let mut buf = Vec::new();
    render_validation_report(report, &mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

#[test]
fn test_report_with_all_severity_levels() {
    let mut report = ValidationReport::new();
    report.add_issue(ValidationIssue::error("Missing specs directory").with_field("structure"));
    report.add_issue(ValidationIssue::error("Invalid config format"));
    report.add_issue(ValidationIssue::warning("Empty description").with_field("content"));
    report.add_issue(ValidationIssue::info("Consider adding metadata"));

    let output = render_to_string(&report);

    // Verify all three sections present
    assert!(output.contains("ERRORS (2)"), "Should have ERRORS section");
    assert!(
        output.contains("WARNINGS (1)"),
        "Should have WARNINGS section"
    );
    assert!(output.contains("INFO (1)"), "Should have INFO section");

    // Verify issue content
    assert!(
        output.contains("Missing specs directory"),
        "Should contain error message"
    );
    assert!(
        output.contains("[structure]"),
        "Should contain field in brackets"
    );
    assert!(
        output.contains("Invalid config format"),
        "Should contain fieldless error"
    );
    assert!(
        output.contains("Empty description"),
        "Should contain warning message"
    );
    assert!(output.contains("[content]"), "Should contain warning field");
    assert!(
        output.contains("Consider adding metadata"),
        "Should contain info message"
    );

    // Verify status
    assert!(
        output.contains("FAILED"),
        "Should show FAILED status (has errors)"
    );
    assert!(output.contains("Errors: 2"), "Should show error count");
    assert!(output.contains("Warnings: 1"), "Should show warning count");
    assert!(output.contains("Info: 1"), "Should show info count");

    // Verify section ordering (errors before warnings before info)
    let errors_pos = output.find("ERRORS").unwrap();
    let warnings_pos = output.find("WARNINGS").unwrap();
    let info_pos = output.find("INFO").unwrap();
    assert!(
        errors_pos < warnings_pos,
        "ERRORS should appear before WARNINGS"
    );
    assert!(
        warnings_pos < info_pos,
        "WARNINGS should appear before INFO"
    );
}

#[test]
fn test_empty_report_rendering() {
    let report = ValidationReport::new();
    let output = render_to_string(&report);

    assert!(
        output.contains("No issues found"),
        "Should show no-issues message"
    );
    assert!(
        output.contains("Workspace is valid"),
        "Should confirm workspace validity"
    );
    assert!(
        !output.contains("ERRORS"),
        "Should not contain ERRORS section"
    );
    assert!(
        !output.contains("WARNINGS"),
        "Should not contain WARNINGS section"
    );
    assert!(!output.contains("INFO"), "Should not contain INFO section");
    assert!(
        !output.contains("Status:"),
        "Should not contain status line"
    );
}

#[test]
fn test_report_built_from_issues_collection() {
    let issues = vec![
        ValidationIssue::warning("Spec has no description").with_field("spec.description"),
        ValidationIssue::warning("Plan has no steps").with_field("plan.steps"),
        ValidationIssue::info("Workspace initialized recently"),
    ];
    let report = ValidationReport::from_issues(issues);
    let output = render_to_string(&report);

    // Warnings-only report should pass
    assert!(output.contains("PASSED"), "Should show PASSED (no errors)");
    assert!(output.contains("WARNINGS (2)"), "Should have 2 warnings");
    assert!(output.contains("INFO (1)"), "Should have 1 info");
    assert!(!output.contains("ERRORS"), "Should not have ERRORS section");

    // Verify field paths preserved through cross-crate boundary
    assert!(
        output.contains("[spec.description]"),
        "Should preserve spec field path"
    );
    assert!(
        output.contains("[plan.steps]"),
        "Should preserve plan field path"
    );

    // Verify counts
    assert!(output.contains("Errors: 0"), "Should show zero errors");
    assert!(output.contains("Warnings: 2"), "Should show warning count");
    assert!(output.contains("Info: 1"), "Should show info count");
}
