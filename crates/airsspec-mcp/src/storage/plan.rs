//! # Filesystem Plan Storage
//!
//! Implements the [`PlanStorage`] trait for local filesystem operations.
//!
//! This provider reads and writes plan YAML files in the `.airsspec/specs/`
//! directory, handling serialization via `serde_yaml`.
//!
//! ## File Naming Convention
//!
//! Plan files are named `{spec-id}.plan.yaml` (e.g., `1737734400-user-auth.plan.yaml`).
//! The `.plan.yaml` suffix distinguishes plan files from spec files (`{spec-id}.yaml`)
//! in the same directory.
//!
//! ## Examples
//!
//! ```no_run
//! use airsspec_core::plan::{Plan, PlanStep, PlanStorage};
//! use airsspec_core::spec::SpecId;
//! use airsspec_mcp::FileSystemPlanStorage;
//!
//! let storage = FileSystemPlanStorage::new("/path/to/.airsspec/specs");
//!
//! // Save a plan
//! let spec_id = SpecId::new(1_737_734_400, "user-auth");
//! let steps = vec![PlanStep::new(0, "Setup", "Initial setup")];
//! let plan = Plan::new(spec_id.clone(), "Incremental approach", steps);
//!
//! // storage.save_plan(&plan); // async
//! // let loaded = storage.load_plan(&spec_id); // async
//! ```

// Layer 1: Standard library
use std::fs;
use std::path::{Path, PathBuf};

// Layer 3: Internal crates/modules
use airsspec_core::plan::{Plan, PlanError, PlanStorage};
use airsspec_core::spec::SpecId;

/// Filesystem-backed plan storage.
///
/// Implements [`PlanStorage`] by reading and writing YAML files in a
/// directory on the local filesystem. Each plan is stored as a separate
/// `{spec-id}.plan.yaml` file.
///
/// # File Format
///
/// Plans are serialized as YAML using `serde_yaml`. The file content
/// contains the full `Plan` structure including spec ID, approach, and steps.
///
/// # Thread Safety
///
/// This type is `Send + Sync` (it holds only a `PathBuf`). However,
/// concurrent writes to the same plan file are not coordinated --
/// callers must ensure appropriate synchronization if needed.
#[derive(Debug, Clone)]
pub struct FileSystemPlanStorage {
    specs_dir: PathBuf,
}

impl FileSystemPlanStorage {
    /// Creates a new filesystem plan storage rooted at the given directory.
    ///
    /// The directory should already exist (typically created by
    /// [`super::FileSystemWorkspaceProvider`]).
    ///
    /// # Arguments
    ///
    /// * `specs_dir` - Path to the directory where plan YAML files are stored
    #[must_use]
    pub fn new(specs_dir: impl Into<PathBuf>) -> Self {
        Self {
            specs_dir: specs_dir.into(),
        }
    }

    /// Returns the path to the specs directory.
    #[must_use]
    pub fn specs_dir(&self) -> &Path {
        &self.specs_dir
    }

    /// Builds the file path for a plan with the given spec ID.
    fn plan_path(&self, spec_id: &SpecId) -> PathBuf {
        self.specs_dir
            .join(format!("{}.plan.yaml", spec_id.as_str()))
    }
}

impl PlanStorage for FileSystemPlanStorage {
    fn load_plan(&self, spec_id: &SpecId) -> impl Future<Output = Result<Plan, PlanError>> + Send {
        let path = self.plan_path(spec_id);
        let id_str = spec_id.as_str().to_string();

        let result = match fs::read_to_string(&path) {
            Ok(content) => serde_yaml::from_str::<Plan>(&content).map_err(|err| {
                PlanError::InvalidFormat(format!(
                    "failed to parse plan YAML '{}': {err}",
                    path.display()
                ))
            }),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(PlanError::NotFound(id_str))
            }
            Err(err) => Err(PlanError::Io(format!(
                "failed to read plan file '{}': {err}",
                path.display()
            ))),
        };

        async move { result }
    }

    fn save_plan(&self, plan: &Plan) -> impl Future<Output = Result<(), PlanError>> + Send {
        let path = self.plan_path(plan.spec_id());

        let result = serde_yaml::to_string(plan)
            .map_err(|err| {
                PlanError::InvalidFormat(format!(
                    "failed to serialize plan to YAML '{}': {err}",
                    path.display()
                ))
            })
            .and_then(|yaml| {
                fs::write(&path, yaml).map_err(|err| {
                    PlanError::Io(format!(
                        "failed to write plan file '{}': {err}",
                        path.display()
                    ))
                })
            });

        async move { result }
    }

    fn delete_plan(&self, spec_id: &SpecId) -> impl Future<Output = Result<(), PlanError>> + Send {
        let path = self.plan_path(spec_id);
        let id_str = spec_id.as_str().to_string();

        let result = match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(PlanError::NotFound(id_str))
            }
            Err(err) => Err(PlanError::Io(format!(
                "failed to delete plan file '{}': {err}",
                path.display()
            ))),
        };

        async move { result }
    }

    fn list_plans(&self) -> impl Future<Output = Result<Vec<SpecId>, PlanError>> + Send {
        let specs_dir = self.specs_dir.clone();

        let result = match fs::read_dir(&specs_dir) {
            Ok(entries) => {
                let ids: Vec<SpecId> = entries
                    .filter_map(Result::ok)
                    .filter_map(|entry| {
                        let file_name = entry.file_name().to_string_lossy().to_string();

                        // Must end with .plan.yaml
                        if !file_name.ends_with(".plan.yaml") {
                            return None;
                        }

                        // Strip .plan.yaml suffix to get the spec ID string
                        let stem = file_name.strip_suffix(".plan.yaml")?;

                        // Parse as SpecId, skip if invalid
                        SpecId::parse(stem).ok()
                    })
                    .collect();
                Ok(ids)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(err) => Err(PlanError::Io(format!(
                "failed to read specs directory '{}': {err}",
                specs_dir.display()
            ))),
        };

        async move { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    use airsspec_core::plan::PlanStep;
    use tempfile::TempDir;

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
            Poll::Pending => panic!("block_on: unexpected Pending from immediately-ready future"),
        }
    }

    /// Creates a test plan with the given spec ID components.
    fn test_plan(timestamp: i64, slug: &str) -> Plan {
        let spec_id = SpecId::new(timestamp, slug);
        Plan::new(
            spec_id,
            format!("Approach for {slug}"),
            vec![
                PlanStep::new(0, "Step 1", "First step"),
                PlanStep::new(1, "Step 2", "Second step"),
            ],
        )
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());
        let plan = test_plan(1_737_734_400, "user-auth");

        // Save
        block_on(storage.save_plan(&plan)).unwrap();

        // Verify file exists on disk
        let expected_path = temp.path().join("1737734400-user-auth.plan.yaml");
        assert!(expected_path.is_file());

        // Load back
        let loaded = block_on(storage.load_plan(plan.spec_id())).unwrap();
        assert_eq!(loaded.spec_id(), plan.spec_id());
        assert_eq!(loaded.approach(), plan.approach());
        assert_eq!(loaded.step_count(), plan.step_count());
        assert_eq!(loaded.steps()[0].title(), plan.steps()[0].title());
        assert_eq!(loaded.steps()[1].title(), plan.steps()[1].title());
    }

    #[test]
    fn test_load_not_found() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());
        let spec_id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.load_plan(&spec_id));
        assert!(result.is_err());
        match result.unwrap_err() {
            PlanError::NotFound(msg) => {
                assert!(
                    msg.contains("nonexistent"),
                    "error should contain the ID, got: {msg}"
                );
            }
            other => panic!("expected NotFound, got: {other:?}"),
        }
    }

    #[test]
    fn test_save_overwrites_existing() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());

        let spec_id = SpecId::new(1_737_734_400, "overwrite-test");

        // Save version 1
        let plan_v1 = Plan::new(
            spec_id.clone(),
            "Original approach",
            vec![PlanStep::new(0, "Original step", "Original")],
        );
        block_on(storage.save_plan(&plan_v1)).unwrap();

        // Save version 2
        let plan_v2 = Plan::new(
            spec_id.clone(),
            "Updated approach",
            vec![
                PlanStep::new(0, "Updated step 1", "Updated"),
                PlanStep::new(1, "New step 2", "Added"),
            ],
        );
        block_on(storage.save_plan(&plan_v2)).unwrap();

        // Load and verify version 2
        let loaded = block_on(storage.load_plan(&spec_id)).unwrap();
        assert_eq!(loaded.approach(), "Updated approach");
        assert_eq!(loaded.step_count(), 2);
    }

    #[test]
    fn test_list_plans_returns_correct_ids() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());

        // Save 3 plans
        for i in 0..3 {
            let plan = test_plan(1_737_734_400 + i, &format!("plan-{i}"));
            block_on(storage.save_plan(&plan)).unwrap();
        }

        let ids = block_on(storage.list_plans()).unwrap();
        assert_eq!(ids.len(), 3);

        // Verify all expected IDs are present (order not guaranteed)
        let id_strings: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        for i in 0..3 {
            let expected = format!("{}-plan-{i}", 1_737_734_400 + i);
            assert!(
                id_strings.contains(&expected),
                "expected {expected} in {id_strings:?}"
            );
        }
    }

    #[test]
    fn test_list_plans_excludes_spec_files() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());

        // Save a real plan
        let plan = test_plan(1_737_734_400, "with-spec");
        block_on(storage.save_plan(&plan)).unwrap();

        // Manually create a .yaml (spec) file in the same directory
        let spec_path = temp.path().join("1737734400-with-spec.yaml");
        fs::write(&spec_path, "id: 1737734400-with-spec\ntitle: test\n").unwrap();

        let ids = block_on(storage.list_plans()).unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].as_str(), "1737734400-with-spec");
    }

    #[test]
    fn test_delete_plan() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());

        let plan = test_plan(1_737_734_400, "to-delete");
        block_on(storage.save_plan(&plan)).unwrap();

        // Verify file exists
        let file_path = temp.path().join("1737734400-to-delete.plan.yaml");
        assert!(file_path.is_file());

        // Delete
        block_on(storage.delete_plan(plan.spec_id())).unwrap();

        // Verify file is gone
        assert!(!file_path.exists());

        // Verify load returns NotFound
        let result = block_on(storage.load_plan(plan.spec_id()));
        assert!(matches!(result, Err(PlanError::NotFound(_))));
    }

    #[test]
    fn test_delete_not_found() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());
        let spec_id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.delete_plan(&spec_id));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PlanError::NotFound(_)));
    }

    #[test]
    fn test_load_malformed_yaml() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemPlanStorage::new(temp.path());
        let spec_id = SpecId::new(1_737_734_400, "malformed");

        // Write invalid YAML content to the expected file
        let file_path = temp.path().join("1737734400-malformed.plan.yaml");
        fs::write(&file_path, "this is not valid yaml: [[[").unwrap();

        let result = block_on(storage.load_plan(&spec_id));
        assert!(result.is_err());
        match result.unwrap_err() {
            PlanError::InvalidFormat(msg) => {
                assert!(
                    msg.contains("failed to parse plan YAML"),
                    "error should describe parse failure, got: {msg}"
                );
            }
            other => panic!("expected InvalidFormat, got: {other:?}"),
        }
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FileSystemPlanStorage>();
    }
}
