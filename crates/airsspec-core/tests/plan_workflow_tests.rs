//! Integration tests for plan workflow.
//!
//! This is a standalone integration test crate that tests
//! the plan module's public API and cross-module interactions.

use airsspec_core::plan::{
    Complexity, Plan, PlanBuilder, PlanStep, StepBuilder, StepStatus, validate_plan,
};
use airsspec_core::spec::SpecId;

#[test]
fn test_plan_creation_with_steps() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let step1 = PlanStep::new(0, "Setup database", "Create schema and migrations");
    let step2 = PlanStep::new(1, "Implement API", "Create REST endpoints");

    let plan = PlanBuilder::new()
        .spec_id(spec_id.clone())
        .approach("Incremental implementation with TDD")
        .step(step1)
        .step(step2)
        .build()
        .expect("valid plan");

    assert_eq!(plan.spec_id(), &spec_id);
    assert_eq!(plan.step_count(), 2);
    assert_eq!(plan.approach(), "Incremental implementation with TDD");
}

#[test]
fn test_plan_step_with_builder() {
    let step = StepBuilder::new()
        .index(0)
        .title("Implementation")
        .description("Detailed description")
        .complexity(Complexity::Complex)
        .build()
        .expect("valid step");

    assert_eq!(step.index(), 0);
    assert_eq!(step.title(), "Implementation");
    assert_eq!(step.complexity(), Complexity::Complex);
    assert_eq!(step.status(), StepStatus::Pending);
}

#[test]
fn test_plan_validation_passes() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Simple approach")
        .step(PlanStep::new(0, "Step 1", "Do thing"))
        .build()
        .expect("valid plan");

    let report = validate_plan(&plan);
    assert!(
        report.is_valid(),
        "Expected valid plan, got errors: {:?}",
        report.errors()
    );
}

#[test]
fn test_plan_step_completion() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .step(PlanStep::new(1, "Step 2", "Description"))
        .build()
        .expect("valid plan");

    // Complete first step
    plan.complete_step(0, Some("Notes".to_string()))
        .expect("should complete");

    assert_eq!(plan.steps()[0].status(), StepStatus::Completed);
    assert_eq!(plan.steps()[1].status(), StepStatus::Pending);
}

#[test]
fn test_plan_completion_percentage() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .step(PlanStep::new(1, "Step 2", "Description"))
        .step(PlanStep::new(2, "Step 3", "Description"))
        .step(PlanStep::new(3, "Step 4", "Description"))
        .build()
        .expect("valid plan");

    assert_eq!(plan.completion_percentage(), 0);

    plan.complete_step(0, None).unwrap();
    assert_eq!(plan.completion_percentage(), 25);

    plan.complete_step(1, None).unwrap();
    assert_eq!(plan.completion_percentage(), 50);
}

#[test]
fn test_plan_requires_at_least_one_step() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    // PlanBuilder requires at least one step
    let result = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_step_complexity_levels() {
    let complexities = [
        Complexity::Trivial,
        Complexity::Simple,
        Complexity::Medium,
        Complexity::Complex,
    ];

    for complexity in complexities {
        let step = StepBuilder::new()
            .index(0)
            .title("Test")
            .description("Test")
            .complexity(complexity)
            .build()
            .expect("valid step");

        assert_eq!(step.complexity(), complexity);
    }
}

#[test]
fn test_plan_with_many_steps() {
    let spec_id = SpecId::new(1_737_734_400, "large-plan");

    let mut builder = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Large plan with many steps");

    for i in 0..10 {
        builder = builder.step(PlanStep::new(i, format!("Step {}", i + 1), "Description"));
    }

    let plan = builder.build().expect("valid plan");
    assert_eq!(plan.step_count(), 10);
}

#[test]
fn test_plan_step_status_change() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .step(PlanStep::new(1, "Step 2", "Description"))
        .build()
        .expect("valid plan");

    // Set first step to skipped via step_mut
    plan.step_mut(0).unwrap().set_status(StepStatus::Skipped);
    assert_eq!(plan.steps()[0].status(), StepStatus::Skipped);
    assert_eq!(plan.steps()[1].status(), StepStatus::Pending);
}

#[test]
fn test_plan_serde_roundtrip() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Test approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .build()
        .expect("valid plan");

    let json = serde_json::to_string(&plan).expect("serialize");
    let parsed: Plan = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(plan.approach(), parsed.approach());
    assert_eq!(plan.step_count(), parsed.step_count());
}

#[test]
fn test_plan_step_in_progress() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .build()
        .expect("valid plan");

    // Set step to in progress via step_mut
    plan.step_mut(0).unwrap().set_status(StepStatus::InProgress);
    assert_eq!(plan.steps()[0].status(), StepStatus::InProgress);
}

#[test]
fn test_plan_all_steps_completed() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .step(PlanStep::new(1, "Step 2", "Description"))
        .build()
        .expect("valid plan");

    plan.complete_step(0, None).unwrap();
    plan.complete_step(1, None).unwrap();

    assert_eq!(plan.completion_percentage(), 100);
    assert!(plan.is_completed());
}

#[test]
fn test_plan_current_step() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .step(PlanStep::new(1, "Step 2", "Description"))
        .build()
        .expect("valid plan");

    assert_eq!(plan.current_step_index(), Some(0));
    assert_eq!(plan.current_step().unwrap().title(), "Step 1");

    plan.complete_step(0, None).unwrap();
    assert_eq!(plan.current_step_index(), Some(1));
    assert_eq!(plan.current_step().unwrap().title(), "Step 2");
}

#[test]
fn test_plan_is_blocked() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");

    let mut plan = PlanBuilder::new()
        .spec_id(spec_id)
        .approach("Approach")
        .step(PlanStep::new(0, "Step 1", "Description"))
        .build()
        .expect("valid plan");

    assert!(!plan.is_blocked());

    plan.step_mut(0).unwrap().set_status(StepStatus::Blocked);
    assert!(plan.is_blocked());
}
