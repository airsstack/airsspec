//! Storage trait for specification persistence.
//!
//! This module defines the storage abstraction for specs. Per project guidelines,
//! we use generics for static dispatch (NO `dyn` trait objects).
//!
//! The trait is designed to support async implementations without requiring
//! tokio in the core crate.

use super::error::SpecError;
use super::id::SpecId;
use super::types::Spec;

/// Trait for persisting and retrieving specifications.
///
/// Implementations of this trait handle the actual I/O operations for
/// storing and loading specs. The trait uses associated types and generics
/// for static dispatch, avoiding `dyn` trait objects per project guidelines.
///
/// # Thread Safety
///
/// All implementations must be `Send + Sync` to support concurrent access
/// in async contexts.
///
/// # Async Design
///
/// Methods return `impl Future` to support async implementations without
/// requiring tokio as a dependency in the core crate. Implementations
/// in `airsspec-mcp` will use tokio for actual async I/O.
///
/// # Examples
///
/// ```ignore
/// // In airsspec-mcp crate:
/// use airsspec_core::spec::{SpecStorage, Spec, SpecId, SpecError};
///
/// struct FileStorage { /* ... */ }
///
/// impl SpecStorage for FileStorage {
///     fn load_spec(&self, id: &SpecId) -> impl Future<Output = Result<Spec, SpecError>> + Send {
///         async move {
///             // Load from file system
///             todo!()
///         }
///     }
///     // ... other methods
/// }
/// ```
pub trait SpecStorage: Send + Sync {
    /// Loads a specification by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the spec to load
    ///
    /// # Errors
    ///
    /// Returns `SpecError::NotFound` if the spec does not exist.
    /// May return other `SpecError` variants for I/O or parsing failures.
    fn load_spec(&self, id: &SpecId) -> impl Future<Output = Result<Spec, SpecError>> + Send;

    /// Saves a specification to storage.
    ///
    /// If a spec with the same ID already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `spec` - The specification to save
    ///
    /// # Errors
    ///
    /// May return `SpecError` for I/O failures.
    fn save_spec(&self, spec: &Spec) -> impl Future<Output = Result<(), SpecError>> + Send;

    /// Lists all specification IDs in storage.
    ///
    /// Returns a vector of all spec IDs, optionally filtered. The default
    /// implementation returns all specs without filtering.
    ///
    /// # Errors
    ///
    /// May return `SpecError` for I/O failures.
    fn list_specs(&self) -> impl Future<Output = Result<Vec<SpecId>, SpecError>> + Send;

    /// Deletes a specification from storage.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the spec to delete
    ///
    /// # Errors
    ///
    /// Returns `SpecError::NotFound` if the spec does not exist.
    /// May return other `SpecError` variants for I/O failures.
    fn delete_spec(&self, id: &SpecId) -> impl Future<Output = Result<(), SpecError>> + Send;

    /// Checks if a specification exists in storage.
    ///
    /// Default implementation loads the spec and checks for success.
    /// Implementations may override for efficiency.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier to check
    fn exists(&self, id: &SpecId) -> impl Future<Output = bool> + Send
    where
        Self: Sized,
    {
        async move { self.load_spec(id).await.is_ok() }
    }
}

/// Extension trait for `SpecStorage` providing additional convenience methods.
///
/// This trait is automatically implemented for all types implementing `SpecStorage`.
pub trait SpecStorageExt: SpecStorage {
    /// Loads multiple specifications by their IDs.
    ///
    /// Returns a vector of results, one for each requested ID.
    /// Failed loads are represented as `Err` in the result vector.
    ///
    /// # Arguments
    ///
    /// * `ids` - Iterator of spec IDs to load
    fn load_specs<'a, I>(
        &'a self,
        ids: I,
    ) -> impl Future<Output = Vec<Result<Spec, SpecError>>> + Send + 'a
    where
        I: IntoIterator<Item = &'a SpecId> + Send + 'a,
        I::IntoIter: Send,
        Self: Sized,
    {
        async move {
            let mut results = Vec::new();
            for id in ids {
                results.push(self.load_spec(id).await);
            }
            results
        }
    }

    /// Loads all specifications from storage.
    ///
    /// Convenience method that combines `list_specs` and `load_specs`.
    ///
    /// # Errors
    ///
    /// Returns an error if listing specs fails. Individual spec load failures
    /// are collected in the returned vector.
    fn load_all(
        &self,
    ) -> impl Future<Output = Result<Vec<Result<Spec, SpecError>>, SpecError>> + Send
    where
        Self: Sized,
    {
        async move {
            let ids = self.list_specs().await?;
            let mut results = Vec::with_capacity(ids.len());
            for id in &ids {
                results.push(self.load_spec(id).await);
            }
            Ok(results)
        }
    }
}

// Blanket implementation for all SpecStorage types
impl<T: SpecStorage> SpecStorageExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::SpecMetadata;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory storage implementation for testing.
    struct InMemoryStorage {
        specs: Mutex<HashMap<String, Spec>>,
    }

    impl InMemoryStorage {
        fn new() -> Self {
            Self {
                specs: Mutex::new(HashMap::new()),
            }
        }
    }

    impl SpecStorage for InMemoryStorage {
        fn load_spec(&self, id: &SpecId) -> impl Future<Output = Result<Spec, SpecError>> + Send {
            let id_str = id.as_str().to_string();
            let specs = self.specs.lock().unwrap();
            let result = specs
                .get(&id_str)
                .cloned()
                .ok_or(SpecError::NotFound(id_str));
            async move { result }
        }

        fn save_spec(&self, spec: &Spec) -> impl Future<Output = Result<(), SpecError>> + Send {
            let mut specs = self.specs.lock().unwrap();
            specs.insert(spec.id().as_str().to_string(), spec.clone());
            async { Ok(()) }
        }

        fn list_specs(&self) -> impl Future<Output = Result<Vec<SpecId>, SpecError>> + Send {
            let specs = self.specs.lock().unwrap();
            let ids: Vec<SpecId> = specs.keys().filter_map(|k| SpecId::parse(k).ok()).collect();
            async move { Ok(ids) }
        }

        fn delete_spec(&self, id: &SpecId) -> impl Future<Output = Result<(), SpecError>> + Send {
            let id_str = id.as_str().to_string();
            let mut specs = self.specs.lock().unwrap();
            let result = if specs.remove(&id_str).is_some() {
                Ok(())
            } else {
                Err(SpecError::NotFound(id_str))
            };
            async move { result }
        }
    }

    // Helper to run async tests synchronously
    fn block_on<F: Future>(f: F) -> F::Output {
        // Simple single-threaded executor for testing
        use std::pin::pin;
        use std::sync::Arc;
        use std::task::{Context, Poll, Wake, Waker};

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

    #[test]
    fn test_in_memory_storage_save_and_load() {
        let storage = InMemoryStorage::new();
        let id = SpecId::new(1_737_734_400, "test-spec");
        let metadata = SpecMetadata::new("Test Spec", "A test specification");
        let spec = Spec::new(id.clone(), metadata, "Content");

        // Save
        let save_result = block_on(storage.save_spec(&spec));
        assert!(save_result.is_ok());

        // Load
        let loaded = block_on(storage.load_spec(&id)).unwrap();
        assert_eq!(loaded.id(), spec.id());
        assert_eq!(loaded.title(), spec.title());
    }

    #[test]
    fn test_in_memory_storage_not_found() {
        let storage = InMemoryStorage::new();
        let id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.load_spec(&id));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SpecError::NotFound(_)));
    }

    #[test]
    fn test_in_memory_storage_list() {
        let storage = InMemoryStorage::new();

        // Add some specs
        for i in 0..3 {
            let id = SpecId::new(1_737_734_400 + i, &format!("spec-{i}"));
            let metadata = SpecMetadata::new(format!("Spec {i}"), "Description");
            let spec = Spec::new(id, metadata, "Content");
            block_on(storage.save_spec(&spec)).unwrap();
        }

        let ids = block_on(storage.list_specs()).unwrap();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn test_in_memory_storage_delete() {
        let storage = InMemoryStorage::new();
        let id = SpecId::new(1_737_734_400, "to-delete");
        let metadata = SpecMetadata::new("To Delete", "Description");
        let spec = Spec::new(id.clone(), metadata, "Content");

        // Save and verify
        block_on(storage.save_spec(&spec)).unwrap();
        assert!(block_on(storage.exists(&id)));

        // Delete
        block_on(storage.delete_spec(&id)).unwrap();
        assert!(!block_on(storage.exists(&id)));

        // Delete again should fail
        let result = block_on(storage.delete_spec(&id));
        assert!(matches!(result.unwrap_err(), SpecError::NotFound(_)));
    }

    #[test]
    fn test_in_memory_storage_exists() {
        let storage = InMemoryStorage::new();
        let id = SpecId::new(1_737_734_400, "exists-test");
        let metadata = SpecMetadata::new("Exists Test", "Description");
        let spec = Spec::new(id.clone(), metadata, "Content");

        assert!(!block_on(storage.exists(&id)));
        block_on(storage.save_spec(&spec)).unwrap();
        assert!(block_on(storage.exists(&id)));
    }

    #[test]
    fn test_storage_ext_load_specs() {
        let storage = InMemoryStorage::new();

        // Add specs
        let id1 = SpecId::new(1_737_734_400, "spec-1");
        let id2 = SpecId::new(1_737_734_401, "spec-2");
        let id3 = SpecId::new(1_737_734_402, "nonexistent");

        let spec1 = Spec::new(id1.clone(), SpecMetadata::new("Spec 1", ""), "");
        let spec2 = Spec::new(id2.clone(), SpecMetadata::new("Spec 2", ""), "");

        block_on(storage.save_spec(&spec1)).unwrap();
        block_on(storage.save_spec(&spec2)).unwrap();

        // Load multiple
        let ids = vec![id1, id2, id3];
        let results = block_on(storage.load_specs(&ids));

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_err());
    }

    #[test]
    fn test_storage_ext_load_all() {
        let storage = InMemoryStorage::new();

        // Add specs
        for i in 0..2 {
            let id = SpecId::new(1_737_734_400 + i, &format!("spec-{i}"));
            let spec = Spec::new(id, SpecMetadata::new(format!("Spec {i}"), ""), "");
            block_on(storage.save_spec(&spec)).unwrap();
        }

        let results = block_on(storage.load_all()).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(Result::is_ok));
    }

    #[test]
    fn test_storage_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<InMemoryStorage>();
    }
}
