//! [`ValidatableSpec`] trait implementation for [`Spec`].
//!
//! This bridges the domain type (`Spec`) with the validation framework's
//! abstraction (`ValidatableSpec`), following the Dependency Inversion
//! Principle: the domain "detail" depends on the framework "abstraction".

use crate::validation::{ValidatableSpec, ValidationReport};

use super::types::Spec;
use super::validator::validate_spec;

impl ValidatableSpec for Spec {
    fn id_str(&self) -> &str {
        self.id().as_str()
    }

    fn dependency_ids(&self) -> Vec<&str> {
        self.dependencies()
            .iter()
            .map(|d| d.spec_id.as_str())
            .collect()
    }

    fn validate_content(&self) -> ValidationReport {
        validate_spec(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Dependency, SpecId, SpecMetadata};

    #[test]
    fn test_spec_id_str() {
        let spec = Spec::new(
            SpecId::new(1_737_734_400, "test-spec"),
            SpecMetadata::new("Test", "Desc"),
            "content",
        );
        assert_eq!(spec.id_str(), "1737734400-test-spec");
    }

    #[test]
    fn test_spec_dependency_ids_empty() {
        let spec = Spec::new(
            SpecId::new(1_737_734_400, "no-deps"),
            SpecMetadata::new("No Deps", "Desc"),
            "content",
        );
        assert!(spec.dependency_ids().is_empty());
    }

    #[test]
    fn test_spec_dependency_ids_populated() {
        let mut metadata = SpecMetadata::new("With Deps", "Desc");
        metadata.add_dependency(Dependency::blocked_by(SpecId::new(1_000_000, "dep-a")));
        metadata.add_dependency(Dependency::related_to(SpecId::new(1_000_001, "dep-b")));
        let spec = Spec::new(SpecId::new(1_737_734_400, "with-deps"), metadata, "content");

        let deps = spec.dependency_ids();
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0], "1000000-dep-a");
        assert_eq!(deps[1], "1000001-dep-b");
    }

    #[test]
    fn test_spec_validate_content_valid() {
        let spec = Spec::new(
            SpecId::new(1_737_734_400, "valid"),
            SpecMetadata::new("Valid Spec", "A good description"),
            "# Content\n\nSome content here.",
        );
        let report = spec.validate_content();
        assert!(report.is_valid());
        assert!(report.is_empty());
    }

    #[test]
    fn test_spec_validate_content_with_self_reference() {
        let id = SpecId::new(1_737_734_400, "self-ref");
        let mut metadata = SpecMetadata::new("Self Ref", "Description");
        metadata.add_dependency(Dependency::blocked_by(id.clone()));
        let spec = Spec::new(id, metadata, "Content");

        let report = spec.validate_content();
        assert!(!report.is_valid());
    }
}
