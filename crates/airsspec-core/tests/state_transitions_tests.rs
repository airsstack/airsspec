//! Integration tests for state machine transitions.
//!
//! This is a standalone integration test crate that tests
//! the state module's public API and cross-module interactions.

use airsspec_core::shared::LifecycleState;
use airsspec_core::spec::SpecId;
use airsspec_core::state::{BuildProgress, StateError, StateMachine, WorkflowState};

#[test]
fn test_state_machine_valid_workflow() {
    let sm = StateMachine::new();

    // Happy path: Draft -> Active -> Done -> Archived
    assert!(sm.can_transition(LifecycleState::Draft, LifecycleState::Active));
    assert!(sm.can_transition(LifecycleState::Active, LifecycleState::Done));
    assert!(sm.can_transition(LifecycleState::Done, LifecycleState::Archived));
}

#[test]
fn test_state_machine_blocked_workflow() {
    let sm = StateMachine::new();

    // Blocked flow: Active -> Blocked -> Active -> Done
    assert!(sm.can_transition(LifecycleState::Active, LifecycleState::Blocked));
    assert!(sm.can_transition(LifecycleState::Blocked, LifecycleState::Active));
}

#[test]
fn test_state_machine_cancelled_workflow() {
    let sm = StateMachine::new();

    // Cancelled flow: Active -> Cancelled -> Archived
    assert!(sm.can_transition(LifecycleState::Active, LifecycleState::Cancelled));
    assert!(sm.can_transition(LifecycleState::Cancelled, LifecycleState::Archived));
}

#[test]
fn test_state_machine_invalid_transitions() {
    let sm = StateMachine::new();

    // Cannot skip states
    assert!(!sm.can_transition(LifecycleState::Draft, LifecycleState::Done));
    assert!(!sm.can_transition(LifecycleState::Draft, LifecycleState::Archived));

    // Cannot go backwards from terminal states
    assert!(!sm.can_transition(LifecycleState::Archived, LifecycleState::Active));
    assert!(!sm.can_transition(LifecycleState::Done, LifecycleState::Active));
}

#[test]
fn test_state_machine_transition_method() {
    let sm = StateMachine::new();

    // Valid transition returns new state
    let result = sm.transition(LifecycleState::Draft, LifecycleState::Active);
    assert_eq!(result, Ok(LifecycleState::Active));

    // Invalid transition returns error
    let result = sm.transition(LifecycleState::Draft, LifecycleState::Done);
    assert!(matches!(result, Err(StateError::InvalidTransition { .. })));
}

#[test]
fn test_workflow_state_integration() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");
    let mut workflow = WorkflowState::new(spec_id.clone());

    // Initial state
    assert_eq!(workflow.lifecycle(), LifecycleState::Draft);
    assert_eq!(workflow.spec_id(), &spec_id);

    // Progress through workflow
    workflow.set_lifecycle(LifecycleState::Active);
    assert_eq!(workflow.lifecycle(), LifecycleState::Active);

    workflow.set_lifecycle(LifecycleState::Done);
    assert_eq!(workflow.lifecycle(), LifecycleState::Done);
}

#[test]
fn test_build_progress_tracking() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");
    let mut workflow = WorkflowState::new(spec_id);

    // Update build progress using the builder pattern
    let progress = BuildProgress::new(4)
        .with_completed(2)
        .with_current_step("Step 2");
    workflow.set_build_progress(progress);

    assert_eq!(workflow.build_progress().percentage(), 50);
    assert_eq!(workflow.build_progress().current_step(), Some("Step 2"));
}

#[test]
fn test_valid_transitions_list() {
    let sm = StateMachine::new();

    let from_draft = sm.valid_transitions(LifecycleState::Draft);
    assert_eq!(from_draft, vec![LifecycleState::Active]);

    let from_active = sm.valid_transitions(LifecycleState::Active);
    assert_eq!(from_active.len(), 3); // Done, Blocked, Cancelled

    let from_archived = sm.valid_transitions(LifecycleState::Archived);
    assert!(from_archived.is_empty());
}

#[test]
fn test_lifecycle_state_properties() {
    // Terminal states
    assert!(LifecycleState::Done.is_terminal());
    assert!(LifecycleState::Cancelled.is_terminal());
    assert!(LifecycleState::Archived.is_terminal());

    // Non-terminal states
    assert!(!LifecycleState::Draft.is_terminal());
    assert!(!LifecycleState::Active.is_terminal());
    assert!(!LifecycleState::Blocked.is_terminal());
}

#[test]
fn test_build_progress_bounds() {
    // Test boundary values for percentage
    let progress_zero = BuildProgress::new(10).with_completed(0);
    assert_eq!(progress_zero.percentage(), 0);

    let progress_hundred = BuildProgress::new(10).with_completed(10);
    assert_eq!(progress_hundred.percentage(), 100);

    // Test with current step
    let progress_with_step = BuildProgress::new(4)
        .with_completed(3)
        .with_current_step("Final step");
    assert_eq!(progress_with_step.percentage(), 75);
    assert_eq!(progress_with_step.current_step(), Some("Final step"));
}

#[test]
fn test_workflow_state_clone() {
    let spec_id = SpecId::new(1_737_734_400, "test-spec");
    let workflow = WorkflowState::new(spec_id);
    let cloned = workflow.clone();

    assert_eq!(workflow.spec_id(), cloned.spec_id());
    assert_eq!(workflow.lifecycle(), cloned.lifecycle());
}

#[test]
fn test_state_machine_self_transitions() {
    let sm = StateMachine::new();

    // Self-transitions should not be allowed (staying in same state)
    assert!(!sm.can_transition(LifecycleState::Draft, LifecycleState::Draft));
    assert!(!sm.can_transition(LifecycleState::Active, LifecycleState::Active));
}

#[test]
fn test_complete_workflow_cycle() {
    let sm = StateMachine::new();
    let mut current = LifecycleState::Draft;

    // Draft -> Active
    current = sm.transition(current, LifecycleState::Active).unwrap();
    assert_eq!(current, LifecycleState::Active);

    // Active -> Done
    current = sm.transition(current, LifecycleState::Done).unwrap();
    assert_eq!(current, LifecycleState::Done);

    // Done -> Archived
    current = sm.transition(current, LifecycleState::Archived).unwrap();
    assert_eq!(current, LifecycleState::Archived);

    // Archived is terminal - no more transitions
    assert!(sm.valid_transitions(current).is_empty());
}

#[test]
fn test_blocked_recovery_workflow() {
    let sm = StateMachine::new();
    let mut current = LifecycleState::Draft;

    // Draft -> Active
    current = sm.transition(current, LifecycleState::Active).unwrap();

    // Active -> Blocked
    current = sm.transition(current, LifecycleState::Blocked).unwrap();
    assert_eq!(current, LifecycleState::Blocked);

    // Blocked -> Active (recovery)
    current = sm.transition(current, LifecycleState::Active).unwrap();
    assert_eq!(current, LifecycleState::Active);

    // Can now proceed to Done
    current = sm.transition(current, LifecycleState::Done).unwrap();
    assert_eq!(current, LifecycleState::Done);
}

#[test]
fn test_build_progress_incremental() {
    let mut progress = BuildProgress::new(5);
    assert_eq!(progress.percentage(), 0);

    progress.complete_step();
    assert_eq!(progress.percentage(), 20);

    progress.complete_step();
    assert_eq!(progress.percentage(), 40);

    progress.complete_step();
    progress.complete_step();
    progress.complete_step();
    assert_eq!(progress.percentage(), 100);
    assert!(progress.is_complete());
}

#[test]
fn test_build_progress_with_notes() {
    let progress = BuildProgress::new(3)
        .with_completed(1)
        .with_notes("In progress");

    assert_eq!(progress.notes(), Some("In progress"));
}
