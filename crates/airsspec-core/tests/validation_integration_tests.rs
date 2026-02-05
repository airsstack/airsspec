//! Integration tests for validation framework.
//!
//! This is a standalone integration test crate that tests
//! the validation module's public API and cross-module interactions.

use airsspec_core::plan::{validate_plan, PlanBuilder, PlanStep};
use airsspec_core::spec::{validate_spec, Category, SpecBuilder, SpecId};
use airsspec_core::validation::{
    ValidationIssue, ValidationReport, ValidationSeverity, Validator, ValidatorExt,
};

#[test]
fn test_spec_validation_complete_spec() {
    let spec = SpecBuilder::new()
        .title("Complete Feature")
        .description("A fully specified feature")
        .category(Category::Feature)
        .content("# Specification\n\n## Overview\n\nThis feature does X.")
        .build()
        .expect("valid spec");

    let report = validate_spec(&spec);

    assert!(report.is_valid());
    assert!(report.is_empty() || report.warning_count() == 0);
}

#[test]
fn test_plan_validation_complete_plan() {
    let spec_id = SpecId::new(1_737_734_400, "test-feature");

    let plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Implement step by step")
        .step(PlanStep::new(0, "Analysis", "Analyze requirements"))
        .step(PlanStep::new(1, "Implementation", "Write the code"))
        .step(PlanStep::new(2, "Testing", "Write tests"))
        .build()
        .expect("valid plan");

    let report = validate_plan(&plan);

    assert!(report.is_valid());
}

#[test]
fn test_validation_report_merging() {
    let mut main_report = ValidationReport::new();

    // Simulate multiple validators
    let mut spec_report = ValidationReport::new();
    spec_report.add_warning("Consider adding more detail");

    let mut plan_report = ValidationReport::new();
    plan_report.add_info("Plan looks good");

    main_report.merge(spec_report);
    main_report.merge(plan_report);

    assert!(main_report.is_valid()); // Warnings/info don't fail
    assert_eq!(main_report.issue_count(), 2);
}

#[test]
fn test_validation_error_tracking() {
    let mut report = ValidationReport::new();

    report.add_error("Missing required field: title");
    report.add_error("Invalid state transition");
    report.add_warning("Empty description");

    assert!(!report.is_valid());
    assert_eq!(report.error_count(), 2);
    assert_eq!(report.warning_count(), 1);
}

#[test]
fn test_custom_validator_integration() {
    // Define a custom validator
    struct TitleLengthValidator {
        max_length: usize,
    }

    impl Validator<String> for TitleLengthValidator {
        fn name(&self) -> &'static str {
            "title-length"
        }

        fn validate(&self, target: &String) -> ValidationReport {
            let mut report = ValidationReport::new();
            if target.len() > self.max_length {
                report.add_error(format!("Title exceeds {} characters", self.max_length));
            }
            report
        }
    }

    // Use the validator
    let validator = TitleLengthValidator { max_length: 50 };

    let short_title = "Short Title".to_string();
    let report = validator.validate(&short_title);
    assert!(report.is_valid());

    let long_title = "A".repeat(100);
    let report = validator.validate(&long_title);
    assert!(!report.is_valid());
}

#[test]
fn test_validator_extension_trait() {
    struct AlwaysWarning;

    impl Validator<String> for AlwaysWarning {
        fn name(&self) -> &'static str {
            "always-warning"
        }

        fn validate(&self, _: &String) -> ValidationReport {
            let mut report = ValidationReport::new();
            report.add_warning("This is a warning");
            report
        }
    }

    let validators = vec![AlwaysWarning, AlwaysWarning, AlwaysWarning];
    let report = validators.validate_all(&"test".to_string());

    assert!(report.is_valid()); // Warnings don't fail
    assert_eq!(report.warning_count(), 3);
}

#[test]
fn test_validation_issue_with_field() {
    let issue = ValidationIssue::error("Field is required").with_field("title");

    assert_eq!(issue.severity(), ValidationSeverity::Error);
    assert_eq!(issue.field(), Some("title"));
}

#[test]
fn test_full_validation_workflow() {
    // Create a spec
    let spec = SpecBuilder::new()
        .title("Full Workflow Test")
        .description("Testing the entire validation workflow")
        .category(Category::Feature)
        .content("Content goes here")
        .build()
        .expect("valid spec");

    // Validate the spec
    let spec_report = validate_spec(&spec);
    assert!(spec_report.is_valid());

    // Create a plan for the spec
    let plan = PlanBuilder::new()
        .spec_id(spec.id().clone())
        .approach("Incremental development")
        .step(PlanStep::new(0, "Setup", "Initial setup"))
        .step(PlanStep::new(1, "Build", "Implementation"))
        .build()
        .expect("valid plan");

    // Validate the plan
    let plan_report = validate_plan(&plan);
    assert!(plan_report.is_valid());

    // Merge reports for combined validation
    let mut combined = ValidationReport::new();
    combined.merge(spec_report);
    combined.merge(plan_report);

    assert!(combined.is_valid());
}

#[test]
fn test_validation_severity_levels() {
    let mut report = ValidationReport::new();

    report.add_error("This is an error");
    report.add_warning("This is a warning");
    report.add_info("This is informational");

    assert_eq!(report.error_count(), 1);
    assert_eq!(report.warning_count(), 1);
    // Note: info_count() is not in the API, but we can count via issues
    assert_eq!(report.issue_count(), 3);

    // Only errors make the report invalid
    assert!(!report.is_valid());
}

#[test]
fn test_empty_validation_report() {
    let report = ValidationReport::new();

    assert!(report.is_valid());
    assert!(report.is_empty());
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warning_count(), 0);
}

#[test]
fn test_validation_report_errors_accessor() {
    let mut report = ValidationReport::new();
    report.add_error("Error 1");
    report.add_error("Error 2");
    report.add_warning("Warning 1");

    let errors = report.errors();
    assert_eq!(errors.len(), 2);
}

#[test]
fn test_validation_report_warnings_accessor() {
    let mut report = ValidationReport::new();
    report.add_error("Error 1");
    report.add_warning("Warning 1");
    report.add_warning("Warning 2");

    let warnings = report.warnings();
    assert_eq!(warnings.len(), 2);
}

#[test]
fn test_chained_validators() {
    struct NonEmptyValidator;
    struct MaxLengthValidator(usize);

    impl Validator<String> for NonEmptyValidator {
        fn name(&self) -> &'static str {
            "non-empty"
        }

        fn validate(&self, target: &String) -> ValidationReport {
            let mut report = ValidationReport::new();
            if target.is_empty() {
                report.add_error("Value cannot be empty");
            }
            report
        }
    }

    impl Validator<String> for MaxLengthValidator {
        fn name(&self) -> &'static str {
            "max-length"
        }

        fn validate(&self, target: &String) -> ValidationReport {
            let mut report = ValidationReport::new();
            if target.len() > self.0 {
                report.add_error(format!("Value exceeds {} characters", self.0));
            }
            report
        }
    }

    // Test valid input
    let validators: Vec<Box<dyn Validator<String>>> =
        vec![Box::new(NonEmptyValidator), Box::new(MaxLengthValidator(10))];

    let mut report = ValidationReport::new();
    for v in &validators {
        report.merge(v.validate(&"hello".to_string()));
    }
    assert!(report.is_valid());

    // Test empty input
    let mut report = ValidationReport::new();
    for v in &validators {
        report.merge(v.validate(&String::new()));
    }
    assert!(!report.is_valid());
    assert_eq!(report.error_count(), 1);

    // Test too long input
    let mut report = ValidationReport::new();
    for v in &validators {
        report.merge(v.validate(&"a".repeat(20)));
    }
    assert!(!report.is_valid());
    assert_eq!(report.error_count(), 1);
}

#[test]
fn test_validation_issue_severity() {
    let error = ValidationIssue::error("Error message");
    assert_eq!(error.severity(), ValidationSeverity::Error);

    let warning = ValidationIssue::warning("Warning message");
    assert_eq!(warning.severity(), ValidationSeverity::Warning);

    let info = ValidationIssue::info("Info message");
    assert_eq!(info.severity(), ValidationSeverity::Info);
}

#[test]
fn test_validation_merge_all() {
    let mut main_report = ValidationReport::new();

    let mut r1 = ValidationReport::new();
    r1.add_error("E1");

    let mut r2 = ValidationReport::new();
    r2.add_warning("W1");

    let mut r3 = ValidationReport::new();
    r3.add_info("I1");

    main_report.merge_all([r1, r2, r3]);
    assert_eq!(main_report.issue_count(), 3);
}
