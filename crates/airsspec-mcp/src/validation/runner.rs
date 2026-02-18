//! Composite validator runner for workspace validation.
//!
//! Contains the [`validate_workspace`] function that orchestrates end-to-end
//! workspace validation: loads specs and plans from the filesystem, builds
//! a typed [`ValidationContext`](airsspec_core::validation::ValidationContext),
//! runs all workspace validators, and returns a merged
//! [`ValidationReport`](airsspec_core::validation::ValidationReport).

// Layer 1: Standard library
use std::fmt;
use std::path::Path;

// Layer 3: Internal crates/modules
use airsspec_core::plan::PlanStorageExt as _;
use airsspec_core::spec::SpecStorageExt as _;
use airsspec_core::validation::{
    DependencyValidator, DirectoryStructureValidator, SpecContentValidator,
    StateTransitionValidator, ValidationContextBuilder, ValidationReport, Validator,
};

use crate::storage::{FileSystemPlanStorage, FileSystemSpecStorage};

/// Validates an `AirsSpec` workspace at the given path.
///
/// This is the main entry point for workspace validation. It:
///
/// 1. Runs directory structure validation
/// 2. Loads all specs and plans from the filesystem
/// 3. Reports any load failures as validation errors
/// 4. Runs content, dependency, and state validators on loaded data
/// 5. Returns a merged `ValidationReport` with all issues
///
/// # Permissive Validation (ADR-005)
///
/// This function never panics or returns an error. ALL issues -- including
/// I/O failures -- are reported as entries in the `ValidationReport`.
/// Validation continues even when some data fails to load.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use airsspec_mcp::validate_workspace;
///
/// # async fn example() {
/// let report = validate_workspace(Path::new("/my/project")).await;
/// if report.is_valid() {
///     println!("Workspace is valid!");
/// } else {
///     for issue in report.issues() {
///         println!("{issue}");
///     }
/// }
/// # }
/// ```
#[must_use]
pub async fn validate_workspace(workspace_path: &Path) -> ValidationReport {
    let mut report = ValidationReport::new();
    let workspace_path_buf = workspace_path.to_path_buf();

    // Phase 1: Validate directory structure (works with any context type)
    let structure_context = ValidationContextBuilder::new()
        .workspace_path(workspace_path_buf.clone())
        .build();
    report.merge(DirectoryStructureValidator.validate(&structure_context));

    // Phase 2: Load specs and plans from filesystem
    let specs_dir = workspace_path.join(".airsspec").join("specs");
    let spec_storage = FileSystemSpecStorage::new(&specs_dir);
    let plan_storage = FileSystemPlanStorage::new(&specs_dir);

    let specs = collect_loaded(&spec_storage.load_all().await, "spec", &mut report);
    let plans = collect_loaded(&plan_storage.load_all().await, "plan", &mut report);

    // Phase 3: Build typed context and run remaining validators
    let context = ValidationContextBuilder::new()
        .workspace_path(workspace_path_buf)
        .specs(specs)
        .plans(plans)
        .build();

    report.merge(SpecContentValidator.validate(&context));
    report.merge(DependencyValidator.validate(&context));
    report.merge(StateTransitionValidator.validate(&context));

    report
}

/// Collects successfully loaded items from a two-level `Result`, reporting
/// failures as validation errors.
///
/// The outer `Result` represents listing failures (e.g., directory not found).
/// The inner `Result` per item represents individual load failures (e.g.,
/// malformed YAML). Both are reported as error-level validation issues.
fn collect_loaded<T: Clone, E1: fmt::Display, E2: fmt::Display>(
    results: &Result<Vec<Result<T, E2>>, E1>,
    label: &str,
    report: &mut ValidationReport,
) -> Vec<T> {
    let items = match results {
        Ok(items) => items,
        Err(err) => {
            report.add_error(format!("Failed to list {label}s: {err}"));
            return Vec::new();
        }
    };

    let mut loaded = Vec::with_capacity(items.len());
    for result in items {
        match result {
            Ok(item) => loaded.push(item.clone()),
            Err(err) => {
                report.add_error(format!("Failed to load {label}: {err}"));
            }
        }
    }
    loaded
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    use airsspec_core::plan::{Plan, PlanStep, PlanStorage};
    use airsspec_core::spec::{Dependency, Spec, SpecId, SpecMetadata, SpecStorage};

    use super::*;

    /// Simple single-threaded executor for testing immediately-ready futures.
    fn block_on<F: Future>(f: F) -> F::Output {
        struct NoopWaker;
        impl Wake for NoopWaker {
            fn wake(self: Arc<Self>) {}
        }
        let waker = Waker::from(Arc::new(NoopWaker));
        let mut cx = Context::from_waker(&waker);
        let mut f = pin!(f);
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(result) => result,
            Poll::Pending => panic!("block_on: unexpected Pending"),
        }
    }

    /// Creates a valid workspace directory structure.
    fn create_workspace(dir: &std::path::Path) {
        fs::create_dir_all(dir.join(".airsspec/specs")).unwrap();
        fs::create_dir_all(dir.join(".airsspec/logs")).unwrap();
        fs::write(
            dir.join(".airsspec/config.toml"),
            "[project]\nname = \"test\"\ndescription = \"test project\"\n",
        )
        .unwrap();
    }

    /// Creates and saves a test spec to the workspace.
    fn save_test_spec(specs_dir: &std::path::Path, timestamp: i64, slug: &str) {
        let storage = FileSystemSpecStorage::new(specs_dir);
        let id = SpecId::new(timestamp, slug);
        let metadata =
            SpecMetadata::new(format!("Test: {slug}"), format!("Description for {slug}"));
        let spec = Spec::new(id, metadata, format!("# {slug}\n\nContent."));
        block_on(storage.save_spec(&spec)).unwrap();
    }

    /// Creates and saves a test spec with dependencies to the workspace.
    fn save_test_spec_with_deps(
        specs_dir: &std::path::Path,
        timestamp: i64,
        slug: &str,
        deps: Vec<SpecId>,
    ) {
        let storage = FileSystemSpecStorage::new(specs_dir);
        let id = SpecId::new(timestamp, slug);
        let mut metadata =
            SpecMetadata::new(format!("Test: {slug}"), format!("Description for {slug}"));
        for dep_id in deps {
            metadata.add_dependency(Dependency::blocked_by(dep_id));
        }
        let spec = Spec::new(id, metadata, format!("# {slug}\n\nContent."));
        block_on(storage.save_spec(&spec)).unwrap();
    }

    /// Creates and saves a test plan to the workspace.
    fn save_test_plan(
        specs_dir: &std::path::Path,
        timestamp: i64,
        slug: &str,
        steps: Vec<PlanStep>,
    ) {
        let storage = FileSystemPlanStorage::new(specs_dir);
        let spec_id = SpecId::new(timestamp, slug);
        let plan = Plan::new(spec_id, "Test approach", steps);
        block_on(storage.save_plan(&plan)).unwrap();
    }

    #[test]
    fn test_valid_workspace_passes() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        let specs_dir = ws.join(".airsspec/specs");
        save_test_spec(&specs_dir, 1_000_000, "my-feature");
        save_test_plan(
            &specs_dir,
            1_000_000,
            "my-feature",
            vec![PlanStep::new(0, "Step 1", "First step")],
        );

        let report = block_on(validate_workspace(ws));
        assert!(
            report.is_valid(),
            "expected valid report, got errors: {:?}",
            report.errors()
        );
        assert!(
            report.is_empty(),
            "expected empty report, got issues: {:?}",
            report.issues()
        );
    }

    #[test]
    fn test_empty_workspace_passes() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        let report = block_on(validate_workspace(ws));
        assert!(
            report.is_valid(),
            "expected valid report, got errors: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_nonexistent_path_reports_errors() {
        let report = block_on(validate_workspace(std::path::Path::new(
            "/nonexistent/path/ws",
        )));
        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains(".airsspec")),
            "expected error mentioning .airsspec, got: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_missing_specs_directory_reports_error() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();

        // Create .airsspec with config but no specs/ directory
        fs::create_dir_all(ws.join(".airsspec/logs")).unwrap();
        fs::write(
            ws.join(".airsspec/config.toml"),
            "[project]\nname = \"test\"\ndescription = \"test\"\n",
        )
        .unwrap();

        let report = block_on(validate_workspace(ws));
        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("specs")),
            "expected error mentioning specs directory, got: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_malformed_spec_yaml_reports_error() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        // Write a file with invalid YAML content but valid spec-ID filename
        let bad_file = ws.join(".airsspec/specs/1000000-bad.yaml");
        fs::write(&bad_file, "this is not valid yaml: [[[").unwrap();

        let report = block_on(validate_workspace(ws));
        assert!(
            !report.is_valid(),
            "expected invalid report due to malformed YAML"
        );
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("Failed to load spec")),
            "expected load error, got: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_broken_dependency_reports_error() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        let specs_dir = ws.join(".airsspec/specs");
        save_test_spec_with_deps(
            &specs_dir,
            1_000_000,
            "depends-on-missing",
            vec![SpecId::new(9_999_999, "nonexistent")],
        );

        let report = block_on(validate_workspace(ws));
        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("non-existent")),
            "expected broken dependency error, got: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_empty_plan_steps_reports_warning() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        let specs_dir = ws.join(".airsspec/specs");
        save_test_spec(&specs_dir, 1_000_000, "empty-plan-spec");
        save_test_plan(&specs_dir, 1_000_000, "empty-plan-spec", vec![]);

        let report = block_on(validate_workspace(ws));
        assert!(
            report.is_valid(),
            "expected valid report (warnings only), got errors: {:?}",
            report.errors()
        );
        assert!(
            report.warning_count() >= 1,
            "expected at least one warning about empty plan, got: {:?}",
            report.warnings()
        );
    }

    #[test]
    fn test_spec_without_plan_validates() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();
        create_workspace(ws);

        let specs_dir = ws.join(".airsspec/specs");
        save_test_spec(&specs_dir, 1_000_000, "no-plan-spec");

        let report = block_on(validate_workspace(ws));
        assert!(
            report.is_valid(),
            "expected valid report for spec without plan, got errors: {:?}",
            report.errors()
        );
    }

    #[test]
    fn test_multiple_validators_all_run() {
        let temp = tempfile::tempdir().unwrap();
        let ws = temp.path();

        // Create workspace without logs/ directory (structure warning)
        fs::create_dir_all(ws.join(".airsspec/specs")).unwrap();
        fs::write(
            ws.join(".airsspec/config.toml"),
            "[project]\nname = \"test\"\ndescription = \"test\"\n",
        )
        .unwrap();

        // Create a spec with empty description (content warning)
        let specs_dir = ws.join(".airsspec/specs");
        let storage = FileSystemSpecStorage::new(&specs_dir);
        let id = SpecId::new(1_000_000, "warn-spec");
        let metadata = SpecMetadata::new("Valid Title", "");
        let spec = Spec::new(id, metadata, "");
        block_on(storage.save_spec(&spec)).unwrap();

        let report = block_on(validate_workspace(ws));

        // Should be valid (only warnings, no errors)
        assert!(
            report.is_valid(),
            "expected valid report (warnings only), got errors: {:?}",
            report.errors()
        );

        // Should have warnings from multiple validators:
        // - DirectoryStructureValidator: missing logs/ directory
        // - SpecContentValidator: empty description and/or empty content
        assert!(
            report.warning_count() >= 2,
            "expected warnings from multiple validators, got {} warning(s): {:?}",
            report.warning_count(),
            report.warnings()
        );
    }
}
