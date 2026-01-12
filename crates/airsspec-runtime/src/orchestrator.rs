//! Orchestrator for managing UOW lifecycle and agent execution.
//!
//! The orchestrator coordinates the execution of agents across the AI Development
//! Lifecycle (AI-DLC), managing state transitions, compliance gates, and agent
//! delegation.

// Layer 1: Standard library imports
use std::path::PathBuf;

// Layer 2: Third-party crate imports
// (none for this module)

// Layer 3: Internal module imports
use airsspec_core::error::StateError;
use airsspec_core::state::{ComplianceGate, Phase, StatePersistence, Transition, UowState};

/// Orchestrator for managing UOW lifecycle.
///
/// The orchestrator is responsible for:
/// - Loading and saving UOW state
/// - Managing phase transitions through compliance gates
/// - Coordinating agent execution (future enhancement)
///
/// # Examples
///
/// ```rust
/// use airsspec_core::state::{Phase, UowState};
/// use airsspec_runtime::orchestrator::Orchestrator;
/// use std::path::PathBuf;
///
/// // Create orchestrator with dependencies (requires concrete implementations)
/// // let orchestrator = Orchchestrator::new(
/// //     PathBuf::from("/workspace"),
/// //     MyPersistence,
/// //     MyComplianceGate,
/// // );
/// ```
pub struct Orchestrator<S: StatePersistence, C: ComplianceGate> {
    /// Path to the workspace root.
    #[allow(dead_code)]
    workspace_path: PathBuf,

    /// State persistence backend.
    state_persistence: S,

    /// Compliance gate for phase transitions.
    compliance_gate: C,
}

impl<S: StatePersistence, C: ComplianceGate> Orchestrator<S, C> {
    /// Creates a new `Orchestrator` with the given dependencies.
    ///
    /// # Arguments
    ///
    /// * `workspace_path` - Path to the workspace root
    /// * `state_persistence` - State persistence backend implementation
    /// * `compliance_gate` - Compliance gate implementation
    ///
    /// # Returns
    ///
    /// A new `Orchestrator` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_runtime::orchestrator::Orchestrator;
    /// use std::path::PathBuf;
    ///
    /// // let orchestrator = Orchestrator::new(
    /// //     PathBuf::from("/workspace"),
    /// //     MyPersistence,
    /// //     MyComplianceGate,
    /// // );
    /// ```
    #[must_use]
    pub const fn new(workspace_path: PathBuf, state_persistence: S, compliance_gate: C) -> Self {
        Self {
            workspace_path,
            state_persistence,
            compliance_gate,
        }
    }

    /// Loads the state for a UOW by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the UOW
    ///
    /// # Returns
    ///
    /// The current `UowState` for the given UOW, or a `StateError` if the state
    /// cannot be loaded.
    ///
    /// # Errors
    ///
    /// Returns a `StateError::Persistence` if the state cannot be loaded from
    /// the persistence backend.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_runtime::orchestrator::Orchestrator;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // let orchestrator = Orchestrator::new(...);
    /// // let state = orchestrator.load_uow("UOW-001").await?;
    /// // assert_eq!(state.id, "UOW-001");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_uow(&self, id: &str) -> Result<UowState, StateError> {
        self.state_persistence.load(id).await
    }

    /// Transitions a UOW to a new phase.
    ///
    /// This method validates that the transition is allowed by the compliance
    /// gate, updates the state, and records the transition.
    ///
    /// # Arguments
    ///
    /// * `uow_id` - The unique identifier of the UOW
    /// * `to` - The destination phase
    ///
    /// # Returns
    ///
    /// `Ok(())` if the transition was successful, or a `StateError` if:
    /// - The transition is invalid
    /// - Gate conditions are not met
    /// - State persistence fails
    ///
    /// # Errors
    ///
    /// - `StateError::InvalidTransition` - Transition from current phase to `to` is not allowed
    /// - `StateError::GateNotMet` - Required artifacts are missing or unapproved
    /// - `StateError::Persistence` - Failed to save state or record transition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::state::Phase;
    /// use airsspec_runtime::orchestrator::Orchestrator;
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // let orchestrator = Orchestrator::new(...);
    /// // orchestrator.transition("UOW-001", Phase::Planning).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn transition(&self, uow_id: &str, to: Phase) -> Result<(), StateError> {
        // Load current state
        let mut state = self.load_uow(uow_id).await?;

        // Validate gate conditions for current state
        self.compliance_gate.validate_gate(&state)?;

        // Create transition record
        let from = state.phase;
        let transition = Transition::new(from, to);

        // Update state phase
        state.update_phase(to);

        // Save updated state
        self.state_persistence.save(&state).await?;

        // Record transition
        self.state_persistence
            .record_transition(uow_id, transition)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use airsspec_core::state::traits::{ArtifactRef, ArtifactType};

    /// Mock implementation of `StatePersistence` for testing.
    struct MockPersistence {
        state: std::sync::Mutex<Option<UowState>>,
    }

    impl MockPersistence {
        fn new() -> Self {
            Self {
                state: std::sync::Mutex::new(None),
            }
        }

        fn set_state(&self, state: UowState) {
            *self.state.lock().unwrap() = Some(state);
        }
    }

    #[async_trait::async_trait]
    impl StatePersistence for MockPersistence {
        async fn load(&self, uow_id: &str) -> Result<UowState, StateError> {
            let guard = self.state.lock().unwrap();
            match guard.as_ref() {
                Some(state) if state.id == uow_id => Ok(state.clone()),
                Some(_) => Err(StateError::Persistence(format!(
                    "UOW ID mismatch: expected {uow_id}"
                ))),
                None => Err(StateError::Persistence("No state set".to_string())),
            }
        }

        async fn save(&self, state: &UowState) -> Result<(), StateError> {
            *self.state.lock().unwrap() = Some(state.clone());
            Ok(())
        }

        async fn record_transition(
            &self,
            _uow_id: &str,
            _transition: Transition,
        ) -> Result<(), StateError> {
            Ok(())
        }
    }

    /// Mock implementation of `ComplianceGate` for testing.
    struct MockComplianceGate {
        allow_all: bool,
    }

    impl MockComplianceGate {
        fn new(allow_all: bool) -> Self {
            Self { allow_all }
        }
    }

    impl ComplianceGate for MockComplianceGate {
        fn can_transition(&self, _from: Phase, _to: Phase, _artifacts: &[ArtifactRef]) -> bool {
            self.allow_all
        }

        fn required_artifacts(&self, _phase: Phase) -> Vec<ArtifactType> {
            vec![]
        }

        fn validate_gate(&self, _state: &UowState) -> Result<(), StateError> {
            if self.allow_all {
                Ok(())
            } else {
                Err(StateError::GateNotMet("Gate not met".to_string()))
            }
        }
    }

    #[test]
    fn test_orchestrator_new() {
        let workspace_path = PathBuf::from("/workspace");
        let persistence = MockPersistence::new();
        let gate = MockComplianceGate::new(true);

        let orchestrator: Orchestrator<MockPersistence, MockComplianceGate> =
            Orchestrator::new(workspace_path, persistence, gate);

        assert_eq!(orchestrator.workspace_path, PathBuf::from("/workspace"));
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_load_uow() {
        let persistence = MockPersistence::new();
        let gate = MockComplianceGate::new(true);
        let orchestrator: Orchestrator<MockPersistence, MockComplianceGate> =
            Orchestrator::new(PathBuf::from("/workspace"), persistence, gate);

        // Mock persistence doesn't have state set yet
        let result = orchestrator.load_uow("UOW-001").await;
        assert!(result.is_err());

        // Set up mock state
        let mock_persistence = MockPersistence::new();
        let state = UowState::new("UOW-001", Phase::Idle);
        mock_persistence.set_state(state);

        let orchestrator2: Orchestrator<MockPersistence, MockComplianceGate> = Orchestrator::new(
            PathBuf::from("/workspace"),
            mock_persistence,
            MockComplianceGate::new(true),
        );

        let loaded_state = orchestrator2.load_uow("UOW-001").await.unwrap();
        assert_eq!(loaded_state.id, "UOW-001");
        assert_eq!(loaded_state.phase, Phase::Idle);
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_transition() {
        let mock_persistence = MockPersistence::new();
        let state = UowState::new("UOW-001", Phase::Idle);
        mock_persistence.set_state(state);

        let orchestrator: Orchestrator<MockPersistence, MockComplianceGate> = Orchestrator::new(
            PathBuf::from("/workspace"),
            mock_persistence,
            MockComplianceGate::new(true),
        );

        let result = orchestrator.transition("UOW-001", Phase::Research).await;
        assert!(result.is_ok());

        // Verify state was updated
        let loaded_state = orchestrator.load_uow("UOW-001").await.unwrap();
        assert_eq!(loaded_state.phase, Phase::Research);
    }

    #[tokio::test]
    async fn test_transition_gate_not_met() {
        let mock_persistence = MockPersistence::new();
        let state = UowState::new("UOW-001", Phase::Idle);
        mock_persistence.set_state(state);

        let orchestrator: Orchestrator<MockPersistence, MockComplianceGate> = Orchestrator::new(
            PathBuf::from("/workspace"),
            mock_persistence,
            MockComplianceGate::new(false), // Gate blocks all
        );

        let result = orchestrator.transition("UOW-001", Phase::Research).await;
        assert!(matches!(result, Err(StateError::GateNotMet(_))));
    }
}
