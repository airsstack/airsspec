//! # Filesystem Spec Storage
//!
//! Implements the [`SpecStorage`] trait for local filesystem operations.
//!
//! This provider reads and writes spec YAML files in the `.airsspec/specs/`
//! directory, handling serialization via `serde_yaml`.
//!
//! ## File Naming Convention
//!
//! Spec files are named `{spec-id}.yaml` (e.g., `1737734400-user-auth.yaml`).
//! Plan files (`{spec-id}.plan.yaml`) in the same directory are excluded
//! from spec listing operations.
//!
//! ## Examples
//!
//! ```no_run
//! use std::path::Path;
//!
//! use airsspec_core::spec::{Spec, SpecId, SpecMetadata, SpecStorage};
//! use airsspec_mcp::FileSystemSpecStorage;
//!
//! let storage = FileSystemSpecStorage::new("/path/to/.airsspec/specs");
//!
//! // Save a spec
//! let id = SpecId::new(1_737_734_400, "user-auth");
//! let metadata = SpecMetadata::new("User Auth", "Implement OAuth2");
//! let spec = Spec::new(id.clone(), metadata, "# User Auth\n\nDetails...");
//!
//! // storage.save_spec(&spec); // async
//! // let loaded = storage.load_spec(&id); // async
//! ```

// Layer 1: Standard library
use std::fs;
use std::path::{Path, PathBuf};

// Layer 3: Internal crates/modules
use airsspec_core::spec::{Spec, SpecError, SpecId, SpecStorage};

/// Filesystem-backed spec storage.
///
/// Implements [`SpecStorage`] by reading and writing YAML files in a
/// directory on the local filesystem. Each spec is stored as a separate
/// `{spec-id}.yaml` file.
///
/// # File Format
///
/// Specs are serialized as YAML using `serde_yaml`. The file content
/// contains the full `Spec` structure including ID, metadata, and content.
///
/// # Thread Safety
///
/// This type is `Send + Sync` (it holds only a `PathBuf`). However,
/// concurrent writes to the same spec file are not coordinated --
/// callers must ensure appropriate synchronization if needed.
#[derive(Debug, Clone)]
pub struct FileSystemSpecStorage {
    specs_dir: PathBuf,
}

impl FileSystemSpecStorage {
    /// Creates a new filesystem spec storage rooted at the given directory.
    ///
    /// The directory should already exist (typically created by
    /// [`super::FileSystemWorkspaceProvider`]).
    ///
    /// # Arguments
    ///
    /// * `specs_dir` - Path to the directory where spec YAML files are stored
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

    /// Builds the file path for a spec with the given ID.
    fn spec_path(&self, id: &SpecId) -> PathBuf {
        self.specs_dir.join(format!("{}.yaml", id.as_str()))
    }
}

impl SpecStorage for FileSystemSpecStorage {
    fn load_spec(&self, id: &SpecId) -> impl Future<Output = Result<Spec, SpecError>> + Send {
        let path = self.spec_path(id);
        let id_str = id.as_str().to_string();

        let result = match fs::read_to_string(&path) {
            Ok(content) => serde_yaml::from_str::<Spec>(&content).map_err(|err| {
                SpecError::InvalidFormat(format!(
                    "failed to parse spec YAML '{}': {err}",
                    path.display()
                ))
            }),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(SpecError::NotFound(id_str))
            }
            Err(err) => Err(SpecError::Io(format!(
                "failed to read spec file '{}': {err}",
                path.display()
            ))),
        };

        async move { result }
    }

    fn save_spec(&self, spec: &Spec) -> impl Future<Output = Result<(), SpecError>> + Send {
        let path = self.spec_path(spec.id());

        let result = serde_yaml::to_string(spec)
            .map_err(|err| {
                SpecError::InvalidFormat(format!(
                    "failed to serialize spec to YAML '{}': {err}",
                    path.display()
                ))
            })
            .and_then(|yaml| {
                fs::write(&path, yaml).map_err(|err| {
                    SpecError::Io(format!(
                        "failed to write spec file '{}': {err}",
                        path.display()
                    ))
                })
            });

        async move { result }
    }

    fn list_specs(&self) -> impl Future<Output = Result<Vec<SpecId>, SpecError>> + Send {
        let specs_dir = self.specs_dir.clone();

        let result = match fs::read_dir(&specs_dir) {
            Ok(entries) => {
                let ids: Vec<SpecId> = entries
                    .filter_map(Result::ok)
                    .filter_map(|entry| {
                        let file_name = entry.file_name().to_string_lossy().to_string();

                        // Must end with .yaml but NOT .plan.yaml.
                        // We intentionally use case-sensitive matching because our file
                        // format specifies lowercase `.yaml` extensions exclusively.
                        #[expect(
                            clippy::case_sensitive_file_extension_comparisons,
                            reason = "compound extension .plan.yaml requires string matching"
                        )]
                        let is_spec_yaml =
                            file_name.ends_with(".yaml") && !file_name.ends_with(".plan.yaml");
                        if !is_spec_yaml {
                            return None;
                        }

                        // Strip .yaml extension to get the spec ID string
                        let stem = file_name.strip_suffix(".yaml")?;

                        // Parse as SpecId, skip if invalid
                        SpecId::parse(stem).ok()
                    })
                    .collect();
                Ok(ids)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(err) => Err(SpecError::Io(format!(
                "failed to read specs directory '{}': {err}",
                specs_dir.display()
            ))),
        };

        async move { result }
    }

    fn delete_spec(&self, id: &SpecId) -> impl Future<Output = Result<(), SpecError>> + Send {
        let path = self.spec_path(id);
        let id_str = id.as_str().to_string();

        let result = match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Err(SpecError::NotFound(id_str))
            }
            Err(err) => Err(SpecError::Io(format!(
                "failed to delete spec file '{}': {err}",
                path.display()
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

    use airsspec_core::spec::SpecMetadata;
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

    /// Creates a test spec with the given ID components.
    fn test_spec(timestamp: i64, slug: &str) -> Spec {
        let id = SpecId::new(timestamp, slug);
        let metadata = SpecMetadata::new(
            format!("Test Spec: {slug}"),
            format!("Description for {slug}"),
        );
        Spec::new(id, metadata, format!("# {slug}\n\nContent for {slug}."))
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());
        let spec = test_spec(1_737_734_400, "user-auth");

        // Save
        block_on(storage.save_spec(&spec)).unwrap();

        // Verify file exists on disk
        let expected_path = temp.path().join("1737734400-user-auth.yaml");
        assert!(expected_path.is_file());

        // Load back
        let loaded = block_on(storage.load_spec(spec.id())).unwrap();
        assert_eq!(loaded.id(), spec.id());
        assert_eq!(loaded.title(), spec.title());
        assert_eq!(loaded.description(), spec.description());
        assert_eq!(loaded.content(), spec.content());
        assert_eq!(loaded.category(), spec.category());
        assert_eq!(loaded.dependencies().len(), spec.dependencies().len());
    }

    #[test]
    fn test_load_not_found() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());
        let id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.load_spec(&id));
        assert!(result.is_err());
        match result.unwrap_err() {
            SpecError::NotFound(msg) => {
                assert!(msg.contains("nonexistent"), "error should contain the ID");
            }
            other => panic!("expected NotFound, got: {other:?}"),
        }
    }

    #[test]
    fn test_save_overwrites_existing() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());

        let id = SpecId::new(1_737_734_400, "overwrite-test");
        let metadata = SpecMetadata::new("Original Title", "Original description");
        let spec_v1 = Spec::new(id.clone(), metadata, "Original content");

        // Save version 1
        block_on(storage.save_spec(&spec_v1)).unwrap();

        // Save version 2 with different content
        let metadata_v2 = SpecMetadata::new("Updated Title", "Updated description");
        let spec_v2 = Spec::new(id.clone(), metadata_v2, "Updated content");
        block_on(storage.save_spec(&spec_v2)).unwrap();

        // Load and verify version 2
        let loaded = block_on(storage.load_spec(&id)).unwrap();
        assert_eq!(loaded.title(), "Updated Title");
        assert_eq!(loaded.content(), "Updated content");
    }

    #[test]
    fn test_list_specs_returns_correct_ids() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());

        // Save 3 specs
        for i in 0..3 {
            let spec = test_spec(1_737_734_400 + i, &format!("spec-{i}"));
            block_on(storage.save_spec(&spec)).unwrap();
        }

        let ids = block_on(storage.list_specs()).unwrap();
        assert_eq!(ids.len(), 3);

        // Verify all expected IDs are present (order not guaranteed)
        let id_strings: Vec<String> = ids.iter().map(|id| id.as_str().to_string()).collect();
        for i in 0..3 {
            let expected = format!("{}-spec-{i}", 1_737_734_400 + i);
            assert!(
                id_strings.contains(&expected),
                "expected {expected} in {id_strings:?}"
            );
        }
    }

    #[test]
    fn test_list_specs_excludes_plan_files() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());

        // Save a real spec
        let spec = test_spec(1_737_734_400, "with-plan");
        block_on(storage.save_spec(&spec)).unwrap();

        // Manually create a .plan.yaml file in the same directory
        let plan_path = temp.path().join("1737734400-with-plan.plan.yaml");
        fs::write(
            &plan_path,
            "spec_id: 1737734400-with-plan\napproach: test\n",
        )
        .unwrap();

        let ids = block_on(storage.list_specs()).unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].as_str(), "1737734400-with-plan");
    }

    #[test]
    fn test_list_specs_skips_unparseable_filenames() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());

        // Save a valid spec
        let spec = test_spec(1_737_734_400, "valid-spec");
        block_on(storage.save_spec(&spec)).unwrap();

        // Create files with invalid spec ID names
        fs::write(temp.path().join("not-a-spec.yaml"), "invalid").unwrap();
        fs::write(temp.path().join("README.yaml"), "also invalid").unwrap();

        let ids = block_on(storage.list_specs()).unwrap();
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].as_str(), "1737734400-valid-spec");
    }

    #[test]
    fn test_delete_spec() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());

        let spec = test_spec(1_737_734_400, "to-delete");
        block_on(storage.save_spec(&spec)).unwrap();

        // Verify file exists
        let file_path = temp.path().join("1737734400-to-delete.yaml");
        assert!(file_path.is_file());

        // Delete
        block_on(storage.delete_spec(spec.id())).unwrap();

        // Verify file is gone
        assert!(!file_path.exists());

        // Verify load returns NotFound
        let result = block_on(storage.load_spec(spec.id()));
        assert!(matches!(result, Err(SpecError::NotFound(_))));
    }

    #[test]
    fn test_delete_not_found() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());
        let id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.delete_spec(&id));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SpecError::NotFound(_)));
    }

    #[test]
    fn test_load_malformed_yaml() {
        let temp = TempDir::new().unwrap();
        let storage = FileSystemSpecStorage::new(temp.path());
        let id = SpecId::new(1_737_734_400, "malformed");

        // Write invalid YAML content to the expected file
        let file_path = temp.path().join("1737734400-malformed.yaml");
        fs::write(&file_path, "this is not valid yaml: [[[").unwrap();

        let result = block_on(storage.load_spec(&id));
        assert!(result.is_err());
        match result.unwrap_err() {
            SpecError::InvalidFormat(msg) => {
                assert!(
                    msg.contains("failed to parse spec YAML"),
                    "error should describe parse failure, got: {msg}"
                );
            }
            other => panic!("expected InvalidFormat, got: {other:?}"),
        }
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FileSystemSpecStorage>();
    }
}
