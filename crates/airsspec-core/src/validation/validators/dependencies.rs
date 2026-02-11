//! Dependency validator.
//!
//! Validates cross-spec dependency relationships in the workspace:
//! - Broken references (dependency target does not exist)
//! - Self-references (spec depends on itself)
//! - Circular dependencies (A -> B -> A or longer cycles)
//!
//! Uses the [`ValidatableSpec`] trait abstraction for DIP compliance.

use std::collections::{HashMap, HashSet};

use crate::validation::context::ValidationContext;
use crate::validation::issue::ValidationIssue;
use crate::validation::report::ValidationReport;
use crate::validation::traits::ValidatableSpec;
use crate::validation::validator::Validator;

/// Validates cross-spec dependencies in the workspace.
///
/// Checks for:
/// - **Broken references**: A spec depends on a spec ID that doesn't exist
/// - **Self-references**: A spec depends on itself
/// - **Circular dependencies**: Cycles in the dependency graph (A -> B -> A)
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use airsspec_core::validation::{
///     Validator, ValidationContextBuilder, DependencyValidator,
///     ValidatableSpec, ValidationReport,
/// };
///
/// // DependencyValidator requires S: ValidatableSpec, so use a typed context
/// struct MockSpec { id: String, deps: Vec<String> }
/// impl ValidatableSpec for MockSpec {
///     fn id_str(&self) -> &str { &self.id }
///     fn dependency_ids(&self) -> Vec<&str> {
///         self.deps.iter().map(|s| s.as_str()).collect()
///     }
///     fn validate_content(&self) -> ValidationReport { ValidationReport::new() }
/// }
///
/// let context = ValidationContextBuilder::new()
///     .workspace_path(PathBuf::from("/project"))
///     .specs(vec![MockSpec { id: "1000000-my-spec".into(), deps: vec![] }])
///     .build();
///
/// let validator = DependencyValidator;
/// let report = validator.validate(&context);
/// assert!(report.is_valid());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DependencyValidator;

impl<S, P> Validator<ValidationContext<S, P>> for DependencyValidator
where
    S: ValidatableSpec,
{
    fn name(&self) -> &'static str {
        "dependencies"
    }

    fn validate(&self, context: &ValidationContext<S, P>) -> ValidationReport {
        let mut report = ValidationReport::new();
        let specs = context.specs();

        if specs.is_empty() {
            return report;
        }

        // Build set of known spec IDs
        let known_ids: HashSet<&str> = specs.iter().map(ValidatableSpec::id_str).collect();

        // Check each spec's dependencies
        for spec in specs {
            let spec_id = spec.id_str();
            let dep_ids = spec.dependency_ids();

            for dep_id in &dep_ids {
                // Self-reference check
                if *dep_id == spec_id {
                    report.add_issue(
                        ValidationIssue::error(format!("Spec '{spec_id}' depends on itself"))
                            .with_field(format!("[{spec_id}] dependencies")),
                    );
                    continue;
                }

                // Broken reference check
                if !known_ids.contains(dep_id) {
                    report.add_issue(
                        ValidationIssue::error(format!(
                            "Spec '{spec_id}' depends on non-existent spec '{dep_id}'"
                        ))
                        .with_field(format!("[{spec_id}] dependencies")),
                    );
                }
            }
        }

        // Circular dependency detection using DFS
        detect_cycles(specs, &mut report);

        report
    }
}

/// Detects circular dependencies using DFS with a visited/in-stack approach.
fn detect_cycles<S: ValidatableSpec>(specs: &[S], report: &mut ValidationReport) {
    // Build adjacency list
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for spec in specs {
        adjacency.insert(spec.id_str(), spec.dependency_ids());
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut in_stack: HashSet<&str> = HashSet::new();
    let mut reported_cycles: HashSet<String> = HashSet::new();

    for spec in specs {
        let id = spec.id_str();
        if !visited.contains(id) {
            dfs_detect_cycle(
                id,
                &adjacency,
                &mut visited,
                &mut in_stack,
                &mut reported_cycles,
                report,
            );
        }
    }
}

/// DFS helper for cycle detection.
fn dfs_detect_cycle<'a>(
    node: &'a str,
    adjacency: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    in_stack: &mut HashSet<&'a str>,
    reported_cycles: &mut HashSet<String>,
    report: &mut ValidationReport,
) {
    visited.insert(node);
    in_stack.insert(node);

    if let Some(neighbors) = adjacency.get(node) {
        for &neighbor in neighbors {
            if !visited.contains(neighbor) {
                // Only recurse if the neighbor is a known spec
                if adjacency.contains_key(neighbor) {
                    dfs_detect_cycle(
                        neighbor,
                        adjacency,
                        visited,
                        in_stack,
                        reported_cycles,
                        report,
                    );
                }
            } else if in_stack.contains(neighbor) {
                // Found a cycle -- create a normalized key to avoid duplicate reports
                let mut cycle_key = [node, neighbor];
                cycle_key.sort_unstable();
                let key = cycle_key.join(",");

                if reported_cycles.insert(key) {
                    report.add_issue(
                        ValidationIssue::error(format!(
                            "Circular dependency detected: '{node}' and '{neighbor}' form a cycle"
                        ))
                        .with_field(format!("[{node}] dependencies")),
                    );
                }
            }
        }
    }

    in_stack.remove(node);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::spec::{Dependency, Spec, SpecId, SpecMetadata};
    use crate::validation::ValidationContextBuilder;

    fn make_spec(timestamp: i64, slug: &str) -> Spec {
        Spec::new(
            SpecId::new(timestamp, slug),
            SpecMetadata::new(slug, "Description"),
            "Content",
        )
    }

    fn make_spec_with_deps(timestamp: i64, slug: &str, deps: Vec<SpecId>) -> Spec {
        let id = SpecId::new(timestamp, slug);
        let mut metadata = SpecMetadata::new(slug, "Description");
        for dep_id in deps {
            metadata.add_dependency(Dependency::blocked_by(dep_id));
        }
        Spec::new(id, metadata, "Content")
    }

    fn make_context(specs: Vec<Spec>) -> ValidationContext<Spec> {
        ValidationContextBuilder::new()
            .workspace_path(PathBuf::from("/project"))
            .specs(specs)
            .build()
    }

    #[test]
    fn test_no_specs_is_valid() {
        let context = make_context(vec![]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_specs_with_no_dependencies_valid() {
        let specs = vec![
            make_spec(1_000_000, "spec-a"),
            make_spec(1_000_001, "spec-b"),
        ];

        let context = make_context(specs);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_valid_dependency_passes() {
        let target = make_spec(1_000_000, "target");
        let source =
            make_spec_with_deps(1_000_001, "source", vec![SpecId::new(1_000_000, "target")]);

        let context = make_context(vec![target, source]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(report.is_valid());
    }

    #[test]
    fn test_broken_dependency_reports_error() {
        let source = make_spec_with_deps(
            1_000_001,
            "source",
            vec![SpecId::new(9_999_999, "nonexistent")],
        );

        let context = make_context(vec![source]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
        assert!(report.errors()[0].message().contains("non-existent"));
    }

    #[test]
    fn test_circular_dependency_detected() {
        let spec_a =
            make_spec_with_deps(1_000_000, "spec-a", vec![SpecId::new(1_000_001, "spec-b")]);
        let spec_b =
            make_spec_with_deps(1_000_001, "spec-b", vec![SpecId::new(1_000_000, "spec-a")]);

        let context = make_context(vec![spec_a, spec_b]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("Circular"))
        );
    }

    #[test]
    fn test_transitive_circular_dependency_detected() {
        let spec_a =
            make_spec_with_deps(1_000_000, "spec-a", vec![SpecId::new(1_000_001, "spec-b")]);
        let spec_b =
            make_spec_with_deps(1_000_001, "spec-b", vec![SpecId::new(1_000_002, "spec-c")]);
        let spec_c =
            make_spec_with_deps(1_000_002, "spec-c", vec![SpecId::new(1_000_000, "spec-a")]);

        let context = make_context(vec![spec_a, spec_b, spec_c]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("Circular"))
        );
    }

    #[test]
    fn test_self_reference_detected() {
        let id = SpecId::new(1_000_000, "self-ref");
        let spec = make_spec_with_deps(1_000_000, "self-ref", vec![id]);

        let context = make_context(vec![spec]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert!(
            report
                .errors()
                .iter()
                .any(|e| e.message().contains("itself"))
        );
    }

    #[test]
    fn test_multiple_broken_dependencies_reported() {
        let source = make_spec_with_deps(
            1_000_000,
            "source",
            vec![
                SpecId::new(9_999_998, "missing-a"),
                SpecId::new(9_999_999, "missing-b"),
            ],
        );

        let context = make_context(vec![source]);
        let validator = DependencyValidator;
        let report = validator.validate(&context);

        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 2);
    }

    #[test]
    fn test_validator_name() {
        let validator = DependencyValidator;
        assert_eq!(
            Validator::<ValidationContext<Spec>>::name(&validator),
            "dependencies"
        );
    }
}
