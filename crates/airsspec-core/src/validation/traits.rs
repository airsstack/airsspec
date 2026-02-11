//! Trait abstractions for validation data sources.
//!
//! These traits follow the Dependency Inversion Principle (DIP): the validation
//! framework defines the abstractions it needs, and domain types implement them.
//! This ensures the validation module (framework) never depends on concrete
//! domain types like `Spec` or `Plan`.
//!
//! ## Design
//!
//! - Traits are defined here (in the validation module) because the framework
//!   owns the abstractions it depends on.
//! - Domain types (`Spec`, `Plan`) implement these traits in their own modules.
//! - Validators use trait bounds, not concrete types.
//! - No `dyn` trait objects -- all usage is through generics (static dispatch).

use super::report::ValidationReport;

/// A spec-like type that can be validated and queried by workspace validators.
///
/// This trait abstracts the data that workspace-level validators need from
/// specifications. Domain types (e.g., `Spec`) implement this trait so
/// the validation framework can operate without depending on concrete types.
///
/// # Implementors
///
/// - [`crate::spec::Spec`] -- The primary implementation
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::{ValidatableSpec, ValidationReport};
///
/// // In tests, you can create mock implementations:
/// struct MockSpec {
///     id: String,
///     deps: Vec<String>,
/// }
///
/// impl ValidatableSpec for MockSpec {
///     fn id_str(&self) -> &str { &self.id }
///     fn dependency_ids(&self) -> Vec<&str> {
///         self.deps.iter().map(|s| s.as_str()).collect()
///     }
///     fn validate_content(&self) -> ValidationReport {
///         ValidationReport::new() // Always valid mock
///     }
/// }
/// ```
pub trait ValidatableSpec: Send + Sync {
    /// Returns the unique identifier of this spec as a string slice.
    ///
    /// Used by validators for error attribution and dependency resolution.
    fn id_str(&self) -> &str;

    /// Returns the IDs of specs this spec depends on.
    ///
    /// Used by `DependencyValidator` to check for broken references,
    /// self-references, and circular dependencies.
    fn dependency_ids(&self) -> Vec<&str>;

    /// Validates this spec's content and returns a validation report.
    ///
    /// Used by `SpecContentValidator` to run per-spec validation without
    /// the validation module needing to know about concrete spec validation
    /// logic.
    fn validate_content(&self) -> ValidationReport;
}

/// A plan-like type that can be queried by workspace validators.
///
/// This trait abstracts the data that workspace-level validators need from
/// implementation plans. Domain types (e.g., `Plan`) implement this trait
/// so the validation framework can operate without depending on concrete types.
///
/// # Implementors
///
/// - [`crate::plan::Plan`] -- The primary implementation
///
/// # Examples
///
/// ```
/// use airsspec_core::validation::ValidatablePlan;
///
/// struct MockPlan {
///     spec_id: String,
///     steps: usize,
///     completed: bool,
/// }
///
/// impl ValidatablePlan for MockPlan {
///     fn spec_id_str(&self) -> &str { &self.spec_id }
///     fn step_count(&self) -> usize { self.steps }
///     fn is_completed(&self) -> bool { self.completed }
/// }
/// ```
pub trait ValidatablePlan: Send + Sync {
    /// Returns the spec ID this plan belongs to, as a string slice.
    ///
    /// Used by `StateTransitionValidator` to match plans to their specs.
    fn spec_id_str(&self) -> &str;

    /// Returns the number of steps in the plan.
    ///
    /// Used by `StateTransitionValidator` to check for empty plans.
    fn step_count(&self) -> usize;

    /// Returns true if all steps are completed.
    ///
    /// Used by `StateTransitionValidator` to verify plan completion.
    fn is_completed(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing the traits themselves

    struct TestSpec {
        id: String,
        deps: Vec<String>,
        valid: bool,
    }

    impl TestSpec {
        fn new(id: &str) -> Self {
            Self {
                id: id.to_string(),
                deps: Vec::new(),
                valid: true,
            }
        }

        fn with_deps(mut self, deps: Vec<&str>) -> Self {
            self.deps = deps.into_iter().map(String::from).collect();
            self
        }

        fn invalid(mut self) -> Self {
            self.valid = false;
            self
        }
    }

    impl ValidatableSpec for TestSpec {
        fn id_str(&self) -> &str {
            &self.id
        }

        fn dependency_ids(&self) -> Vec<&str> {
            self.deps.iter().map(String::as_str).collect()
        }

        fn validate_content(&self) -> ValidationReport {
            let mut report = ValidationReport::new();
            if !self.valid {
                report.add_error("Invalid content");
            }
            report
        }
    }

    struct TestPlan {
        spec_id: String,
        steps: usize,
        completed: bool,
    }

    impl TestPlan {
        fn new(spec_id: &str, steps: usize) -> Self {
            Self {
                spec_id: spec_id.to_string(),
                steps,
                completed: false,
            }
        }

        fn completed(mut self) -> Self {
            self.completed = true;
            self
        }
    }

    impl ValidatablePlan for TestPlan {
        fn spec_id_str(&self) -> &str {
            &self.spec_id
        }

        fn step_count(&self) -> usize {
            self.steps
        }

        fn is_completed(&self) -> bool {
            self.completed
        }
    }

    #[test]
    fn test_validatable_spec_id() {
        let spec = TestSpec::new("1000000-my-spec");
        assert_eq!(spec.id_str(), "1000000-my-spec");
    }

    #[test]
    fn test_validatable_spec_dependency_ids() {
        let spec =
            TestSpec::new("1000000-spec-a").with_deps(vec!["1000001-spec-b", "1000002-spec-c"]);
        let deps = spec.dependency_ids();
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0], "1000001-spec-b");
        assert_eq!(deps[1], "1000002-spec-c");
    }

    #[test]
    fn test_validatable_spec_no_deps() {
        let spec = TestSpec::new("1000000-no-deps");
        assert!(spec.dependency_ids().is_empty());
    }

    #[test]
    fn test_validatable_spec_validate_content_valid() {
        let spec = TestSpec::new("1000000-valid");
        let report = spec.validate_content();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validatable_spec_validate_content_invalid() {
        let spec = TestSpec::new("1000000-invalid").invalid();
        let report = spec.validate_content();
        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
    }

    #[test]
    fn test_validatable_plan_spec_id() {
        let plan = TestPlan::new("1000000-my-spec", 3);
        assert_eq!(plan.spec_id_str(), "1000000-my-spec");
    }

    #[test]
    fn test_validatable_plan_step_count() {
        let plan = TestPlan::new("1000000-spec", 5);
        assert_eq!(plan.step_count(), 5);
    }

    #[test]
    fn test_validatable_plan_not_completed() {
        let plan = TestPlan::new("1000000-spec", 3);
        assert!(!plan.is_completed());
    }

    #[test]
    fn test_validatable_plan_completed() {
        let plan = TestPlan::new("1000000-spec", 3).completed();
        assert!(plan.is_completed());
    }

    #[test]
    fn test_validatable_plan_empty_steps() {
        let plan = TestPlan::new("1000000-spec", 0);
        assert_eq!(plan.step_count(), 0);
    }
}
