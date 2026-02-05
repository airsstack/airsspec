//! Integration tests for spec lifecycle.
//!
//! This is a standalone integration test crate that tests
//! the spec module's public API and cross-module interactions.

use airsspec_core::spec::{
    Category, Dependency, DependencyKind, Spec, SpecBuilder, SpecId, validate_spec,
};
use airsspec_core::utils::{id, slug};

#[test]
fn test_spec_creation_with_builder() {
    let spec = SpecBuilder::new()
        .title("User Authentication")
        .description("Implement OAuth2 login flow")
        .category(Category::Feature)
        .content("# Specification\n\nImplement OAuth2...")
        .build()
        .expect("valid spec");

    assert_eq!(spec.title(), "User Authentication");
    assert_eq!(spec.description(), "Implement OAuth2 login flow");
    assert_eq!(spec.category(), Category::Feature);
    assert!(!spec.content().is_empty());
}

#[test]
fn test_spec_id_generated_from_title() {
    let spec = SpecBuilder::new()
        .title("User Auth")
        .description("Authentication feature")
        .build()
        .expect("valid spec");

    // SpecBuilder generates ID from title
    assert_eq!(spec.id().slug(), "user-auth");
    assert!(spec.id().timestamp() > 0);
}

#[test]
fn test_spec_validation_passes() {
    let spec = SpecBuilder::new()
        .title("Valid Spec")
        .description("A valid specification")
        .category(Category::Feature)
        .content("Some content")
        .build()
        .expect("valid spec");

    let report = validate_spec(&spec);
    assert!(
        report.is_valid(),
        "Expected valid spec, got errors: {:?}",
        report.errors()
    );
}

#[test]
fn test_spec_with_dependencies() {
    let dep_id = SpecId::new(1_737_734_400, "auth-system");
    let dep = Dependency::new(dep_id.clone(), DependencyKind::BlockedBy);

    let spec = SpecBuilder::new()
        .title("Payment System")
        .description("Depends on auth")
        .dependency(dep)
        .build()
        .expect("valid spec");

    assert_eq!(spec.dependencies().len(), 1);
    assert_eq!(spec.dependencies()[0].spec_id, dep_id);
}

#[test]
fn test_slug_generation_integration() {
    let title = "My Complex Feature #123";
    let slug_str = slug::generate_default(title);

    // Slug should be usable in SpecId
    let spec_id = SpecId::new(1_737_734_400, &slug_str);
    assert_eq!(spec_id.slug(), "my-complex-feature-123");
}

#[test]
fn test_spec_metadata_accessible() {
    let spec = SpecBuilder::new()
        .title("Test")
        .description("Test")
        .build()
        .expect("valid spec");

    // Metadata is accessible via the spec
    assert_eq!(spec.metadata().title(), "Test");
    assert_eq!(spec.metadata().description(), "Test");
}

#[test]
fn test_spec_with_all_categories() {
    let categories = [
        Category::Feature,
        Category::Enhancement,
        Category::BugFix,
        Category::Refactor,
        Category::Documentation,
        Category::Infrastructure,
    ];

    for category in categories {
        let spec = SpecBuilder::new()
            .title("Test")
            .description("Test")
            .category(category)
            .build()
            .expect("valid spec");

        assert_eq!(spec.category(), category);
    }
}

#[test]
fn test_spec_with_multiple_dependencies() {
    let dep1 = Dependency::blocked_by(SpecId::new(1_000_000, "dep-one"));
    let dep2 = Dependency::related_to(SpecId::new(2_000_000, "dep-two"));
    let dep3 = Dependency::new(SpecId::new(3_000_000, "dep-three"), DependencyKind::ChildOf);

    let spec = SpecBuilder::new()
        .title("Complex Spec")
        .description("Has multiple dependencies")
        .dependency(dep1)
        .dependency(dep2)
        .dependency(dep3)
        .build()
        .expect("valid spec");

    assert_eq!(spec.dependencies().len(), 3);
}

#[test]
fn test_spec_id_roundtrip() {
    let original_id = id::generate_spec_id_with_timestamp(1_737_734_400, "Test Feature");
    let id_string = original_id.as_str();
    let parsed_id = SpecId::parse(id_string).expect("should parse");

    assert_eq!(original_id, parsed_id);
}

#[test]
fn test_spec_serde_roundtrip() {
    let spec = SpecBuilder::new()
        .title("Test Spec")
        .description("For serialization test")
        .category(Category::Feature)
        .content("Content here")
        .build()
        .expect("valid spec");

    let json = serde_json::to_string(&spec).expect("serialize");
    let parsed: Spec = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(spec.title(), parsed.title());
    assert_eq!(spec.description(), parsed.description());
    assert_eq!(spec.category(), parsed.category());
}

#[test]
fn test_dependency_kinds() {
    let kinds = [
        DependencyKind::BlockedBy,
        DependencyKind::RelatedTo,
        DependencyKind::ChildOf,
        DependencyKind::ParentOf,
    ];

    for kind in kinds {
        let dep = Dependency::new(SpecId::new(1_000_000, "test"), kind);
        assert_eq!(dep.kind, kind);
    }
}

#[test]
fn test_spec_id_components() {
    let spec_id = SpecId::new(1_737_734_400, "test-feature");
    assert_eq!(spec_id.timestamp(), 1_737_734_400);
    assert_eq!(spec_id.slug(), "test-feature");
    assert_eq!(spec_id.as_str(), "1737734400-test-feature");
}

#[test]
fn test_spec_id_parsing() {
    let parsed = SpecId::parse("1737734400-my-spec").expect("should parse");
    assert_eq!(parsed.timestamp(), 1_737_734_400);
    assert_eq!(parsed.slug(), "my-spec");
}

#[test]
fn test_spec_id_with_hyphenated_slug() {
    let spec_id = SpecId::new(1_737_734_400, "multi-word-slug");
    assert_eq!(spec_id.slug(), "multi-word-slug");

    let parsed = SpecId::parse("1737734400-multi-word-slug").expect("should parse");
    assert_eq!(parsed.slug(), "multi-word-slug");
}

#[test]
fn test_utils_id_generation() {
    let spec_id = id::generate_spec_id("My Feature");
    assert!(!spec_id.slug().is_empty());
    assert!(spec_id.timestamp() > 0);

    // Generated slug should be normalized
    assert_eq!(spec_id.slug(), "my-feature");
}
