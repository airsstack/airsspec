//! State persistence and compliance gate traits.
//!
//! This module defines traits for managing UOW state persistence and enforcing
//! compliance gate rules for phase transitions.

// Layer 1: Standard library imports
use std::fmt;

// Layer 2: Third-party crate imports
use async_trait::async_trait;

// Layer 3: Internal module imports
use crate::error::StateError;
use crate::state::types::{Phase, Transition, UowState};

// Placeholder types for artifact references.
// These will be fully defined in the artifact module (BOLT-002).
// TODO: Replace these placeholder types with artifact::types::ArtifactRef
//       and artifact::types::ArtifactType when BOLT-002 is implemented.

/// Reference to an artifact in the UOW.
///
/// This is a placeholder type that will be replaced by the full artifact
/// reference type defined in `artifact::types::ArtifactType`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRef {
    /// Artifact type identifier.
    pub artifact_type: String,
    /// UOW ID that owns the artifact.
    pub uow_id: String,
}

/// Type of artifact in the UOW.
///
/// This is a placeholder type that will be replaced by the full artifact type
/// enum defined in `artifact::types::ArtifactType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactType {
    /// Requirements document.
    Requirements,
    /// Domain Architecture Analysis.
    Daa,
    /// Architecture Decision Record.
    Adr,
    /// Request for Change.
    Rfc,
}

impl fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requirements => write!(f, "requirements"),
            Self::Daa => write!(f, "daa"),
            Self::Adr => write!(f, "adr"),
            Self::Rfc => write!(f, "rfc"),
        }
    }
}

/// Trait for persisting and loading UOW state.
///
/// This trait abstracts the storage backend for UOW state, allowing different
/// implementations (file system, database, etc.) without changing the core
/// logic.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::error::StateError;
/// use airsspec_core::state::{StatePersistence, Transition, UowState};
///
/// struct FileBackend;
///
/// #[async_trait::async_trait]
/// impl StatePersistence for FileBackend {
///     async fn load(&self, uow_id: &str) -> Result<UowState, StateError> {
///         // Implementation
///         todo!()
///     }
///
///     async fn save(&self, state: &UowState) -> Result<(), StateError> {
///         // Implementation
///         todo!()
///     }
///
///     async fn record_transition(&self, uow_id: &str, transition: Transition) -> Result<(), StateError> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait StatePersistence: Send + Sync {
    /// Loads the current state for a UOW.
    ///
    /// # Arguments
    ///
    /// * `uow_id` - The unique identifier of the UOW
    ///
    /// # Returns
    ///
    /// The current `UowState` for the given UOW, or a `StateError` if the state
    /// cannot be loaded.
    async fn load(&self, uow_id: &str) -> Result<UowState, StateError>;

    /// Saves the current state for a UOW.
    ///
    /// # Arguments
    ///
    /// * `state` - The state to save
    ///
    /// # Returns
    ///
    /// `Ok(())` if the state was saved successfully, or a `StateError` if the
    /// save operation failed.
    async fn save(&self, state: &UowState) -> Result<(), StateError>;

    /// Records a phase transition for a UOW.
    ///
    /// # Arguments
    ///
    /// * `uow_id` - The unique identifier of the UOW
    /// * `transition` - The transition to record
    ///
    /// # Returns
    ///
    /// `Ok(())` if the transition was recorded successfully, or a `StateError`
    /// if the operation failed.
    async fn record_transition(
        &self,
        uow_id: &str,
        transition: Transition,
    ) -> Result<(), StateError>;
}

/// Trait for enforcing compliance gate rules for phase transitions.
///
/// This trait defines the interface for checking whether phase transitions are
/// allowed based on the presence and approval status of required artifacts.
///
/// # Examples
///
/// ```rust
/// use airsspec_core::error::StateError;
/// use airsspec_core::state::{ArtifactRef, ArtifactType, ComplianceGate, Phase, UowState};
///
/// struct ComplianceRules;
///
/// impl ComplianceGate for ComplianceRules {
///     fn can_transition(&self, from: Phase, to: Phase, artifacts: &[ArtifactRef]) -> bool {
///         // Check if transition is allowed
///         todo!()
///     }
///
///     fn required_artifacts(&self, phase: Phase) -> Vec<ArtifactType> {
///         // Return required artifacts for phase
///         todo!()
///     }
///
///     fn validate_gate(&self, state: &UowState) -> Result<(), StateError> {
///         // Validate gate conditions
///         todo!()
///     }
/// }
/// ```
pub trait ComplianceGate: Send + Sync {
    /// Checks if a phase transition is allowed.
    ///
    /// # Arguments
    ///
    /// * `from` - The source phase
    /// * `to` - The destination phase
    /// * `artifacts` - Slice of artifacts available in the current state
    ///
    /// # Returns
    ///
    /// `true` if the transition is allowed, `false` otherwise.
    fn can_transition(&self, from: Phase, to: Phase, artifacts: &[ArtifactRef]) -> bool;

    /// Returns the list of artifact types required for a phase.
    ///
    /// # Arguments
    ///
    /// * `phase` - The phase to get required artifacts for
    ///
    /// # Returns
    ///
    /// A vector of `ArtifactType` values that must exist and be approved before
    /// entering the given phase.
    fn required_artifacts(&self, phase: Phase) -> Vec<ArtifactType>;

    /// Validates that all gate conditions are met for the current state.
    ///
    /// # Arguments
    ///
    /// * `state` - The UOW state to validate
    ///
    /// # Returns
    ///
    /// `Ok(())` if all gate conditions are satisfied, or a `StateError` if any
    /// condition is not met.
    ///
    /// # Errors
    ///
    /// Returns a `StateError` if any gate condition is not met.
    fn validate_gate(&self, state: &UowState) -> Result<(), StateError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock implementation of `StatePersistence` for testing.
    struct MockPersistence;

    #[async_trait]
    impl StatePersistence for MockPersistence {
        async fn load(&self, uow_id: &str) -> Result<UowState, StateError> {
            Ok(UowState::new(uow_id, Phase::Idle))
        }

        async fn save(&self, _state: &UowState) -> Result<(), StateError> {
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
    struct MockComplianceGate;

    impl ComplianceGate for MockComplianceGate {
        fn can_transition(&self, _from: Phase, _to: Phase, _artifacts: &[ArtifactRef]) -> bool {
            true
        }

        fn required_artifacts(&self, _phase: Phase) -> Vec<ArtifactType> {
            vec![]
        }

        fn validate_gate(&self, _state: &UowState) -> Result<(), StateError> {
            Ok(())
        }
    }

    #[test]
    fn test_artifact_type_display() {
        assert_eq!(ArtifactType::Requirements.to_string(), "requirements");
        assert_eq!(ArtifactType::Daa.to_string(), "daa");
        assert_eq!(ArtifactType::Adr.to_string(), "adr");
        assert_eq!(ArtifactType::Rfc.to_string(), "rfc");
    }

    #[tokio::test]
    #[allow(clippy::unwrap_used)]
    async fn test_state_persistence_load() {
        let persistence = MockPersistence;
        let state = persistence.load("UOW-001").await.unwrap();
        assert_eq!(state.id, "UOW-001");
        assert_eq!(state.phase, Phase::Idle);
    }

    #[tokio::test]
    async fn test_state_persistence_save() {
        let persistence = MockPersistence;
        let state = UowState::new("UOW-001", Phase::Planning);
        let result = persistence.save(&state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_state_persistence_record_transition() {
        let persistence = MockPersistence;
        let transition = Transition::new(Phase::Planning, Phase::Construction);
        let result = persistence.record_transition("UOW-001", transition).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_compliance_gate_can_transition() {
        let gate = MockComplianceGate;
        let artifacts = vec![];
        assert!(gate.can_transition(Phase::Planning, Phase::Construction, &artifacts));
    }

    #[test]
    fn test_compliance_gate_required_artifacts() {
        let gate = MockComplianceGate;
        let artifacts = gate.required_artifacts(Phase::Planning);
        assert!(artifacts.is_empty());
    }

    #[test]
    fn test_compliance_gate_validate_gate() {
        let gate = MockComplianceGate;
        let state = UowState::new("UOW-001", Phase::Planning);
        let result = gate.validate_gate(&state);
        assert!(result.is_ok());
    }
}
