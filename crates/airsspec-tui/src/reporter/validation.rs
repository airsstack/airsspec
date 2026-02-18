//! Validation report rendering for terminal output.
//!
//! Implements styled, colored output of [`ValidationReport`] contents
//! using crossterm style commands written to any [`std::io::Write`] destination.

// Layer 1: Standard library
use std::io::{self, Write};

// Layer 2: External crates
use crossterm::style::{Attribute, ResetColor, SetAttribute, SetForegroundColor};
use ratatui::style::Color;

// Layer 3: Internal crates/modules
use airsspec_core::validation::{ValidationIssue, ValidationReport, ValidationSeverity};

use crate::theme::colors;

/// Renders a validation report as styled terminal output.
///
/// Writes a colored, structured summary of validation results to the
/// provided writer. Issues are grouped by severity (errors first, then
/// warnings, then info) with a summary status line at the end.
///
/// # Output Format
///
/// For a report with errors:
/// ```text
/// ERRORS (2)
///   [structure] Missing directory: specs/
///   Broken dependency: auth -> nonexistent
///
/// WARNINGS (1)
///   [content] Empty spec content
///
/// Status: FAILED | Errors: 2 | Warnings: 1 | Info: 0
/// ```
///
/// For a clean report:
/// ```text
/// No issues found. Workspace is valid.
/// ```
///
/// # Errors
///
/// Returns an error if writing to the provided writer fails.
pub fn render_validation_report(
    report: &ValidationReport,
    writer: &mut impl Write,
) -> io::Result<()> {
    if report.is_empty() {
        write_empty_report(writer)?;
        return Ok(());
    }

    let errors = report.errors();
    let warnings = report.warnings();
    let info_issues = collect_info_issues(report);

    if !errors.is_empty() {
        write_section(writer, "ERRORS", errors.len(), colors::ERROR, &errors)?;
    }

    if !warnings.is_empty() {
        write_section(
            writer,
            "WARNINGS",
            warnings.len(),
            colors::WARNING,
            &warnings,
        )?;
    }

    if !info_issues.is_empty() {
        write_section(
            writer,
            "INFO",
            info_issues.len(),
            colors::PRIMARY,
            &info_issues,
        )?;
    }

    write_status_line(writer, report)?;

    Ok(())
}

/// Writes the "no issues" message for empty reports.
fn write_empty_report(writer: &mut impl Write) -> io::Result<()> {
    write!(writer, "{}", SetForegroundColor(colors::SUCCESS.into()))?;
    write!(writer, "{}", SetAttribute(Attribute::Bold))?;
    write!(writer, "No issues found.")?;
    write!(writer, "{}", SetAttribute(Attribute::NoBold))?;
    writeln!(writer, " Workspace is valid.")?;
    write!(writer, "{ResetColor}")?;
    Ok(())
}

/// Writes a section header and its issues.
fn write_section(
    writer: &mut impl Write,
    header: &str,
    count: usize,
    color: Color,
    issues: &[&ValidationIssue],
) -> io::Result<()> {
    // Header line: "ERRORS (2)"
    write!(writer, "{}", SetForegroundColor(color.into()))?;
    write!(writer, "{}", SetAttribute(Attribute::Bold))?;
    writeln!(writer, "{header} ({count})")?;
    write!(writer, "{}", SetAttribute(Attribute::NoBold))?;

    // Each issue (color already set, only bold was removed)
    for issue in issues {
        write_issue(writer, issue)?;
    }

    // Blank line after section
    write!(writer, "{ResetColor}")?;
    writeln!(writer)?;

    Ok(())
}

/// Writes a single issue line.
fn write_issue(writer: &mut impl Write, issue: &ValidationIssue) -> io::Result<()> {
    if let Some(field) = issue.field() {
        writeln!(writer, "  [{field}] {}", issue.message())?;
    } else {
        writeln!(writer, "  {}", issue.message())?;
    }
    Ok(())
}

/// Writes the status summary line.
fn write_status_line(writer: &mut impl Write, report: &ValidationReport) -> io::Result<()> {
    let info_count = report.issue_count() - report.error_count() - report.warning_count();

    // Status label
    if report.is_valid() {
        write!(writer, "{}", SetForegroundColor(colors::SUCCESS.into()))?;
        write!(writer, "{}", SetAttribute(Attribute::Bold))?;
        write!(writer, "Status: PASSED")?;
    } else {
        write!(writer, "{}", SetForegroundColor(colors::ERROR.into()))?;
        write!(writer, "{}", SetAttribute(Attribute::Bold))?;
        write!(writer, "Status: FAILED")?;
    }
    write!(writer, "{}", SetAttribute(Attribute::Reset))?;

    // Counts
    write!(writer, "{}", SetForegroundColor(colors::MUTED.into()))?;
    writeln!(
        writer,
        " | Errors: {} | Warnings: {} | Info: {}",
        report.error_count(),
        report.warning_count(),
        info_count,
    )?;
    write!(writer, "{ResetColor}")?;

    Ok(())
}

/// Collects info-level issues from the report.
///
/// `ValidationReport` does not have a dedicated `infos()` method,
/// so we filter `issues()` by severity.
fn collect_info_issues(report: &ValidationReport) -> Vec<&ValidationIssue> {
    report
        .issues()
        .iter()
        .filter(|i| i.severity() == ValidationSeverity::Info)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: render report to string for assertions.
    fn render_to_string(report: &ValidationReport) -> String {
        let mut buf = Vec::new();
        render_validation_report(report, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn test_empty_report_shows_no_issues() {
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
        assert!(
            !output.contains("Status:"),
            "Should not contain status line"
        );
    }

    #[test]
    fn test_errors_only() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("Missing specs directory"));
        report.add_issue(ValidationIssue::error("Invalid config format"));

        let output = render_to_string(&report);

        assert!(
            output.contains("ERRORS (2)"),
            "Should show ERRORS header with count"
        );
        assert!(
            output.contains("Missing specs directory"),
            "Should contain first error message"
        );
        assert!(
            output.contains("Invalid config format"),
            "Should contain second error message"
        );
        assert!(
            output.contains("Status: FAILED"),
            "Should show FAILED status"
        );
        assert!(output.contains("Errors: 2"), "Should show error count");
        assert!(
            !output.contains("WARNINGS"),
            "Should not contain WARNINGS section"
        );
    }

    #[test]
    fn test_warnings_only_shows_passed() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::warning("Empty description"));

        let output = render_to_string(&report);

        assert!(
            output.contains("WARNINGS (1)"),
            "Should show WARNINGS header with count"
        );
        assert!(
            output.contains("Status: PASSED"),
            "Should show PASSED (warnings don't fail)"
        );
        assert!(output.contains("Warnings: 1"), "Should show warning count");
        assert!(
            !output.contains("ERRORS"),
            "Should not contain ERRORS section"
        );
    }

    #[test]
    fn test_info_only_shows_passed() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::info("Consider adding metadata"));

        let output = render_to_string(&report);

        assert!(
            output.contains("INFO (1)"),
            "Should show INFO header with count"
        );
        assert!(
            output.contains("Status: PASSED"),
            "Should show PASSED status"
        );
        assert!(output.contains("Info: 1"), "Should show info count");
    }

    #[test]
    fn test_mixed_errors_and_warnings() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("Error one"));
        report.add_issue(ValidationIssue::error("Error two"));
        report.add_issue(ValidationIssue::warning("Warning one"));
        report.add_issue(ValidationIssue::info("Info one"));

        let output = render_to_string(&report);

        // Verify section ordering
        let errors_pos = output
            .find("ERRORS (2)")
            .expect("Should have ERRORS section");
        let warnings_pos = output
            .find("WARNINGS (1)")
            .expect("Should have WARNINGS section");
        let info_pos = output.find("INFO (1)").expect("Should have INFO section");
        assert!(
            errors_pos < warnings_pos,
            "ERRORS should appear before WARNINGS"
        );
        assert!(
            warnings_pos < info_pos,
            "WARNINGS should appear before INFO"
        );

        assert!(
            output.contains("Status: FAILED"),
            "Should show FAILED status"
        );
        assert!(
            output.contains("Errors: 2 | Warnings: 1 | Info: 1"),
            "Should show all counts"
        );
    }

    #[test]
    fn test_issue_with_field_shows_brackets() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("Missing value").with_field("spec.title"));

        let output = render_to_string(&report);

        assert!(
            output.contains("[spec.title]"),
            "Should show field in brackets"
        );
        assert!(output.contains("Missing value"), "Should show the message");
    }

    #[test]
    fn test_issue_without_field_no_brackets() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("General error message"));

        let output = render_to_string(&report);

        // The issue line should show the message without a field bracket prefix.
        // ANSI escape sequences contain '[' so we check for the specific pattern
        // of a bracketed field label preceding the message text.
        assert!(
            !output.contains("[General"),
            "Should not have brackets around the message"
        );
        assert!(
            output.contains("  General error message"),
            "Should contain the error message with indent but no field brackets"
        );
    }

    #[test]
    fn test_write_error_propagated() {
        /// A writer that always fails.
        struct FailWriter;

        impl Write for FailWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "test error"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "test error"))
            }
        }

        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("An error"));

        let result = render_validation_report(&report, &mut FailWriter);
        assert!(result.is_err(), "Should propagate write errors");
    }

    #[test]
    fn test_errors_and_info_skips_warnings() {
        let mut report = ValidationReport::new();
        report.add_issue(ValidationIssue::error("An error"));
        report.add_issue(ValidationIssue::info("Some info"));

        let output = render_to_string(&report);

        assert!(output.contains("ERRORS (1)"), "Should have ERRORS section");
        assert!(output.contains("INFO (1)"), "Should have INFO section");
        assert!(
            !output.contains("WARNINGS"),
            "Should not have WARNINGS section"
        );
        assert!(
            output.contains("Status: FAILED"),
            "Should show FAILED status"
        );
        assert!(output.contains("Warnings: 0"), "Should show zero warnings");
    }
}
