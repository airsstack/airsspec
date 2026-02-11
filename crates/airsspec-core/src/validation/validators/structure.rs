//! Directory structure validator.
//!
//! Validates that the workspace has the required directory structure:
//! `.airsspec/` root, `specs/` subdirectory, `logs/` subdirectory,
//! and `config.toml` configuration file.

use crate::validation::context::ValidationContext;
use crate::validation::issue::ValidationIssue;
use crate::validation::report::ValidationReport;
use crate::validation::validator::Validator;

/// Validates the workspace directory structure.
///
/// Checks that the following exist under the workspace path:
/// - `.airsspec/` directory (error if missing)
/// - `specs/` subdirectory within `.airsspec/` (error if missing)
/// - `logs/` subdirectory within `.airsspec/` (warning if missing)
/// - `config.toml` within `.airsspec/` (error if missing)
///
/// This validator does not need access to specs or plans, so it works
/// with any `ValidationContext<S, P>` regardless of type parameters.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use airsspec_core::validation::{
///     Validator, ValidationContext, DirectoryStructureValidator,
/// };
///
/// let context = ValidationContext::new(PathBuf::from("/my/project"));
/// let validator = DirectoryStructureValidator;
/// let report = validator.validate(&context);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DirectoryStructureValidator;

impl<S, P> Validator<ValidationContext<S, P>> for DirectoryStructureValidator {
    fn name(&self) -> &'static str {
        "directory-structure"
    }

    fn validate(&self, context: &ValidationContext<S, P>) -> ValidationReport {
        let mut report = ValidationReport::new();
        let ws = context.workspace_path();

        // Check .airsspec/ directory
        let airsspec_dir = ws.join(".airsspec");
        if !airsspec_dir.is_dir() {
            report.add_issue(
                ValidationIssue::error(format!(
                    "Missing required directory: {}",
                    airsspec_dir.display()
                ))
                .with_field(".airsspec"),
            );
            // If .airsspec/ is missing, no point checking subdirectories
            return report;
        }

        // Check specs/ subdirectory
        let specs_dir = airsspec_dir.join("specs");
        if !specs_dir.is_dir() {
            report.add_issue(
                ValidationIssue::error(format!(
                    "Missing required directory: {}",
                    specs_dir.display()
                ))
                .with_field(".airsspec/specs"),
            );
        }

        // Check logs/ subdirectory (warning only - non-critical)
        let logs_dir = airsspec_dir.join("logs");
        if !logs_dir.is_dir() {
            report.add_issue(
                ValidationIssue::warning(format!(
                    "Missing optional directory: {}",
                    logs_dir.display()
                ))
                .with_field(".airsspec/logs"),
            );
        }

        // Check config.toml
        let config_file = airsspec_dir.join("config.toml");
        if !config_file.is_file() {
            report.add_issue(
                ValidationIssue::error(format!("Missing required file: {}", config_file.display()))
                    .with_field(".airsspec/config.toml"),
            );
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// Helper: create a `ValidationContext` for a temp directory path.
    fn context_for(path: &std::path::Path) -> ValidationContext {
        ValidationContext::new(path.to_path_buf())
    }

    #[test]
    fn test_valid_workspace_structure() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();

        // Create required structure
        std::fs::create_dir_all(ws.join(".airsspec/specs")).unwrap();
        std::fs::create_dir_all(ws.join(".airsspec/logs")).unwrap();
        std::fs::write(
            ws.join(".airsspec/config.toml"),
            "[project]\nname = \"test\"\ndescription = \"test\"\n",
        )
        .unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(ws));

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_missing_airsspec_directory() {
        let tmp = tempfile::tempdir().unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(tmp.path()));

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
        assert!(report.errors()[0].message().contains(".airsspec"));
    }

    #[test]
    fn test_missing_specs_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();

        std::fs::create_dir_all(ws.join(".airsspec/logs")).unwrap();
        std::fs::write(
            ws.join(".airsspec/config.toml"),
            "[project]\nname = \"t\"\ndescription = \"t\"\n",
        )
        .unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(ws));

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.field() == Some(".airsspec/specs"))
        );
    }

    #[test]
    fn test_missing_logs_directory_is_warning() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();

        std::fs::create_dir_all(ws.join(".airsspec/specs")).unwrap();
        std::fs::write(
            ws.join(".airsspec/config.toml"),
            "[project]\nname = \"t\"\ndescription = \"t\"\n",
        )
        .unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(ws));

        assert!(report.is_valid()); // Warnings don't invalidate
        assert_eq!(report.warning_count(), 1);
        assert!(report.warnings()[0].field() == Some(".airsspec/logs"));
    }

    #[test]
    fn test_missing_config_file() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();

        std::fs::create_dir_all(ws.join(".airsspec/specs")).unwrap();
        std::fs::create_dir_all(ws.join(".airsspec/logs")).unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(ws));

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.field() == Some(".airsspec/config.toml"))
        );
    }

    #[test]
    fn test_multiple_missing_items_reported_together() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();

        // Only create .airsspec/ but nothing inside
        std::fs::create_dir_all(ws.join(".airsspec")).unwrap();

        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context_for(ws));

        assert!(!report.is_valid());
        // Should report: missing specs/, missing config.toml (errors), missing logs/ (warning)
        assert_eq!(report.error_count(), 2);
        assert_eq!(report.warning_count(), 1);
    }

    #[test]
    fn test_validator_name() {
        let validator = DirectoryStructureValidator;
        assert_eq!(
            Validator::<ValidationContext>::name(&validator),
            "directory-structure"
        );
    }

    #[test]
    fn test_nonexistent_workspace_path() {
        let context =
            ValidationContext::new(PathBuf::from("/nonexistent/path/that/does/not/exist"));
        let validator = DirectoryStructureValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1); // Missing .airsspec/ (early return)
    }
}
