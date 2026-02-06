//! Storage trait for plan persistence.
//!
//! This module defines the storage abstraction for plans. Per project guidelines,
//! we use generics for static dispatch (NO `dyn` trait objects).

use crate::spec::SpecId;

use super::error::PlanError;
use super::types::Plan;

/// Trait for persisting and retrieving plans.
///
/// Implementations handle the actual I/O operations for storing and loading plans.
/// Plans are keyed by their associated spec ID (one plan per spec).
///
/// # Thread Safety
///
/// All implementations must be `Send + Sync` to support concurrent access.
///
/// # Async Design
///
/// Methods return `impl Future` to support async implementations without
/// requiring tokio as a dependency in the core crate.
///
/// # Examples
///
/// ```ignore
/// // In airsspec-mcp crate:
/// use airsspec_core::plan::{PlanStorage, Plan, PlanError};
/// use airsspec_core::spec::SpecId;
///
/// struct FileStorage { /* ... */ }
///
/// impl PlanStorage for FileStorage {
///     fn load_plan(&self, spec_id: &SpecId) -> impl Future<Output = Result<Plan, PlanError>> + Send {
///         async move {
///             // Load from file system
///             todo!()
///         }
///     }
///     // ... other methods
/// }
/// ```
pub trait PlanStorage: Send + Sync {
    /// Loads a plan by its associated spec ID.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The spec ID the plan belongs to
    ///
    /// # Errors
    ///
    /// Returns `PlanError::NotFound` if no plan exists for the spec.
    /// May return other `PlanError` variants for I/O or parsing failures.
    fn load_plan(&self, spec_id: &SpecId) -> impl Future<Output = Result<Plan, PlanError>> + Send;

    /// Saves a plan to storage.
    ///
    /// If a plan for the same spec already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `plan` - The plan to save
    ///
    /// # Errors
    ///
    /// May return `PlanError` for I/O failures.
    fn save_plan(&self, plan: &Plan) -> impl Future<Output = Result<(), PlanError>> + Send;

    /// Deletes a plan from storage.
    ///
    /// # Arguments
    ///
    /// * `spec_id` - The spec ID whose plan should be deleted
    ///
    /// # Errors
    ///
    /// Returns `PlanError::NotFound` if no plan exists for the spec.
    /// May return other `PlanError` variants for I/O failures.
    fn delete_plan(&self, spec_id: &SpecId) -> impl Future<Output = Result<(), PlanError>> + Send;

    /// Lists all spec IDs that have associated plans.
    ///
    /// # Errors
    ///
    /// May return `PlanError` for I/O failures.
    fn list_plans(&self) -> impl Future<Output = Result<Vec<SpecId>, PlanError>> + Send;

    /// Checks if a plan exists for the given spec.
    ///
    /// Default implementation loads the plan and checks for success.
    /// Implementations may override for efficiency.
    fn exists(&self, spec_id: &SpecId) -> impl Future<Output = bool> + Send
    where
        Self: Sized,
    {
        async move { self.load_plan(spec_id).await.is_ok() }
    }
}

/// Extension trait for `PlanStorage` providing additional convenience methods.
///
/// This trait is automatically implemented for all types implementing `PlanStorage`.
pub trait PlanStorageExt: PlanStorage {
    /// Loads multiple plans by their spec IDs.
    ///
    /// Returns a vector of results, one for each requested spec ID.
    fn load_plans<'a, I>(
        &'a self,
        spec_ids: I,
    ) -> impl Future<Output = Vec<Result<Plan, PlanError>>> + Send + 'a
    where
        I: IntoIterator<Item = &'a SpecId> + Send + 'a,
        I::IntoIter: Send,
        Self: Sized,
    {
        async move {
            let mut results = Vec::new();
            for spec_id in spec_ids {
                results.push(self.load_plan(spec_id).await);
            }
            results
        }
    }

    /// Loads all plans from storage.
    ///
    /// # Errors
    ///
    /// Returns an error if listing plans fails. Individual load failures
    /// are collected in the returned vector.
    fn load_all(
        &self,
    ) -> impl Future<Output = Result<Vec<Result<Plan, PlanError>>, PlanError>> + Send
    where
        Self: Sized,
    {
        async move {
            let spec_ids = self.list_plans().await?;
            let mut results = Vec::with_capacity(spec_ids.len());
            for spec_id in &spec_ids {
                results.push(self.load_plan(spec_id).await);
            }
            Ok(results)
        }
    }
}

// Blanket implementation for all PlanStorage types
impl<T: PlanStorage> PlanStorageExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::step::PlanStep;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory storage implementation for testing.
    struct InMemoryPlanStorage {
        plans: Mutex<HashMap<String, Plan>>,
    }

    impl InMemoryPlanStorage {
        fn new() -> Self {
            Self {
                plans: Mutex::new(HashMap::new()),
            }
        }
    }

    impl PlanStorage for InMemoryPlanStorage {
        fn load_plan(
            &self,
            spec_id: &SpecId,
        ) -> impl Future<Output = Result<Plan, PlanError>> + Send {
            let key = spec_id.as_str().to_string();
            let plans = self.plans.lock().unwrap();
            let result = plans.get(&key).cloned().ok_or(PlanError::NotFound(key));
            async move { result }
        }

        fn save_plan(&self, plan: &Plan) -> impl Future<Output = Result<(), PlanError>> + Send {
            let mut plans = self.plans.lock().unwrap();
            plans.insert(plan.spec_id().as_str().to_string(), plan.clone());
            async { Ok(()) }
        }

        fn delete_plan(
            &self,
            spec_id: &SpecId,
        ) -> impl Future<Output = Result<(), PlanError>> + Send {
            let key = spec_id.as_str().to_string();
            let mut plans = self.plans.lock().unwrap();
            let result = if plans.remove(&key).is_some() {
                Ok(())
            } else {
                Err(PlanError::NotFound(key))
            };
            async move { result }
        }

        fn list_plans(&self) -> impl Future<Output = Result<Vec<SpecId>, PlanError>> + Send {
            let plans = self.plans.lock().unwrap();
            let ids: Vec<SpecId> = plans.keys().filter_map(|k| SpecId::parse(k).ok()).collect();
            async move { Ok(ids) }
        }
    }

    // Helper to run async tests synchronously
    fn block_on<F: Future>(f: F) -> F::Output {
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

        loop {
            match f.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {}
            }
        }
    }

    fn test_spec_id() -> SpecId {
        SpecId::new(1_737_734_400, "test-spec")
    }

    fn test_plan(spec_id: SpecId) -> Plan {
        Plan::new(
            spec_id,
            "Test approach",
            vec![PlanStep::new(0, "Step 1", "First step")],
        )
    }

    #[test]
    fn test_save_and_load() {
        let storage = InMemoryPlanStorage::new();
        let spec_id = test_spec_id();
        let plan = test_plan(spec_id.clone());

        // Save
        let save_result = block_on(storage.save_plan(&plan));
        assert!(save_result.is_ok());

        // Load
        let loaded = block_on(storage.load_plan(&spec_id)).unwrap();
        assert_eq!(loaded.spec_id(), plan.spec_id());
        assert_eq!(loaded.approach(), plan.approach());
    }

    #[test]
    fn test_not_found() {
        let storage = InMemoryPlanStorage::new();
        let spec_id = SpecId::new(1_737_734_400, "nonexistent");

        let result = block_on(storage.load_plan(&spec_id));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PlanError::NotFound(_)));
    }

    #[test]
    fn test_delete() {
        let storage = InMemoryPlanStorage::new();
        let spec_id = test_spec_id();
        let plan = test_plan(spec_id.clone());

        // Save and verify
        block_on(storage.save_plan(&plan)).unwrap();
        assert!(block_on(storage.exists(&spec_id)));

        // Delete
        block_on(storage.delete_plan(&spec_id)).unwrap();
        assert!(!block_on(storage.exists(&spec_id)));

        // Delete again should fail
        let result = block_on(storage.delete_plan(&spec_id));
        assert!(matches!(result.unwrap_err(), PlanError::NotFound(_)));
    }

    #[test]
    fn test_list_plans() {
        let storage = InMemoryPlanStorage::new();

        // Add some plans
        for i in 0..3 {
            let spec_id = SpecId::new(1_737_734_400 + i, &format!("spec-{i}"));
            let plan = test_plan(spec_id);
            block_on(storage.save_plan(&plan)).unwrap();
        }

        let ids = block_on(storage.list_plans()).unwrap();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn test_exists() {
        let storage = InMemoryPlanStorage::new();
        let spec_id = test_spec_id();
        let plan = test_plan(spec_id.clone());

        assert!(!block_on(storage.exists(&spec_id)));
        block_on(storage.save_plan(&plan)).unwrap();
        assert!(block_on(storage.exists(&spec_id)));
    }

    #[test]
    fn test_storage_ext_load_plans() {
        let storage = InMemoryPlanStorage::new();

        let id_1 = SpecId::new(1_737_734_400, "spec-1");
        let id_2 = SpecId::new(1_737_734_401, "spec-2");
        let id_missing = SpecId::new(1_737_734_402, "nonexistent");

        block_on(storage.save_plan(&test_plan(id_1.clone()))).unwrap();
        block_on(storage.save_plan(&test_plan(id_2.clone()))).unwrap();

        let ids = vec![id_1, id_2, id_missing];
        let results = block_on(storage.load_plans(&ids));

        assert_eq!(results.len(), 3);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());
        assert!(results[2].is_err());
    }

    #[test]
    fn test_storage_ext_load_all() {
        let storage = InMemoryPlanStorage::new();

        for i in 0..2 {
            let spec_id = SpecId::new(1_737_734_400 + i, &format!("spec-{i}"));
            block_on(storage.save_plan(&test_plan(spec_id))).unwrap();
        }

        let results = block_on(storage.load_all()).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(Result::is_ok));
    }

    #[test]
    fn test_storage_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<InMemoryPlanStorage>();
    }
}
