//! Workflow state for specifications.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{LifecycleState, Phase};
use crate::spec::SpecId;

use super::progress::BuildProgress;

/// Complete workflow state for a specification.
///
/// Tracks the current lifecycle state, phase, and build progress.
/// This is persisted alongside the spec for workflow tracking.
///
/// # Examples
///
/// ```
/// use airsspec_core::state::WorkflowState;
/// use airsspec_core::spec::SpecId;
/// use airsspec_core::shared::{LifecycleState, Phase};
///
/// let spec_id = SpecId::new(1737734400, "user-auth");
/// let state = WorkflowState::new(spec_id);
///
/// assert_eq!(state.lifecycle(), LifecycleState::Draft);
/// assert_eq!(state.phase(), Phase::Spec);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    /// ID of the spec this state belongs to.
    spec_id: SpecId,
    /// Current lifecycle state.
    lifecycle: LifecycleState,
    /// Current workflow phase.
    phase: Phase,
    /// Build progress (relevant in Build phase).
    build_progress: BuildProgress,
    /// Last update timestamp.
    updated_at: DateTime<Utc>,
}

impl WorkflowState {
    /// Creates initial workflow state for a new spec.
    #[must_use]
    pub fn new(spec_id: SpecId) -> Self {
        Self {
            spec_id,
            lifecycle: LifecycleState::default(),
            phase: Phase::default(),
            build_progress: BuildProgress::default(),
            updated_at: Utc::now(),
        }
    }

    /// Returns the spec ID.
    #[must_use]
    pub fn spec_id(&self) -> &SpecId {
        &self.spec_id
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub fn lifecycle(&self) -> LifecycleState {
        self.lifecycle
    }

    /// Returns the current workflow phase.
    #[must_use]
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Returns the build progress.
    #[must_use]
    pub fn build_progress(&self) -> &BuildProgress {
        &self.build_progress
    }

    /// Returns the last update timestamp.
    #[must_use]
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Sets the lifecycle state and updates the timestamp.
    pub fn set_lifecycle(&mut self, lifecycle: LifecycleState) {
        self.lifecycle = lifecycle;
        self.updated_at = Utc::now();
    }

    /// Sets the workflow phase and updates the timestamp.
    pub fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.updated_at = Utc::now();
    }

    /// Sets the build progress and updates the timestamp.
    pub fn set_build_progress(&mut self, progress: BuildProgress) {
        self.build_progress = progress;
        self.updated_at = Utc::now();
    }

    /// Advances to the next phase if possible.
    ///
    /// Returns `true` if advanced, `false` if already at the Build phase.
    pub fn advance_phase(&mut self) -> bool {
        if let Some(next) = self.phase.next() {
            self.phase = next;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_spec_id() -> SpecId {
        SpecId::new(1_737_734_400, "test-spec")
    }

    #[test]
    fn test_new() {
        let spec_id = test_spec_id();
        let state = WorkflowState::new(spec_id.clone());

        assert_eq!(state.spec_id(), &spec_id);
        assert_eq!(state.lifecycle(), LifecycleState::Draft);
        assert_eq!(state.phase(), Phase::Spec);
        assert_eq!(state.build_progress().total_steps(), 0);
    }

    #[test]
    fn test_set_lifecycle() {
        let spec_id = test_spec_id();
        let mut state = WorkflowState::new(spec_id);
        let original_updated_at = state.updated_at();

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(1));

        state.set_lifecycle(LifecycleState::Active);
        assert_eq!(state.lifecycle(), LifecycleState::Active);
        assert!(state.updated_at() >= original_updated_at);
    }

    #[test]
    fn test_set_phase() {
        let spec_id = test_spec_id();
        let mut state = WorkflowState::new(spec_id);
        let original_updated_at = state.updated_at();

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(1));

        state.set_phase(Phase::Plan);
        assert_eq!(state.phase(), Phase::Plan);
        assert!(state.updated_at() >= original_updated_at);
    }

    #[test]
    fn test_advance_phase() {
        let spec_id = test_spec_id();
        let mut state = WorkflowState::new(spec_id);

        assert_eq!(state.phase(), Phase::Spec);

        // Advance Spec -> Plan
        assert!(state.advance_phase());
        assert_eq!(state.phase(), Phase::Plan);

        // Advance Plan -> Build
        assert!(state.advance_phase());
        assert_eq!(state.phase(), Phase::Build);

        // Cannot advance beyond Build
        assert!(!state.advance_phase());
        assert_eq!(state.phase(), Phase::Build);
    }

    #[test]
    fn test_set_build_progress() {
        let spec_id = test_spec_id();
        let mut state = WorkflowState::new(spec_id);

        let progress = BuildProgress::new(5).with_completed(2);
        state.set_build_progress(progress.clone());

        assert_eq!(state.build_progress().total_steps(), 5);
        assert_eq!(state.build_progress().completed_steps(), 2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let spec_id = test_spec_id();
        let mut state = WorkflowState::new(spec_id);
        state.set_lifecycle(LifecycleState::Active);
        state.set_phase(Phase::Build);
        state.set_build_progress(BuildProgress::new(10).with_completed(5));

        let json = serde_json::to_string(&state).unwrap();
        let parsed: WorkflowState = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.spec_id(), state.spec_id());
        assert_eq!(parsed.lifecycle(), state.lifecycle());
        assert_eq!(parsed.phase(), state.phase());
        assert_eq!(
            parsed.build_progress().total_steps(),
            state.build_progress().total_steps()
        );
    }
}
