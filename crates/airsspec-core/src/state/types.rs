//! State types for UOW lifecycle management.
//!
//! This module defines the core types for tracking UOW state, phases, and transitions
//! throughout the AI Development Lifecycle (AI-DLC).

// Layer 1: Standard library imports
// (none)

// Layer 2: Third-party crate imports
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Layer 3: Internal module imports
// (none - this is a foundational module)

/// Phase of the AI Development Lifecycle (AI-DLC).
///
/// Represents the current phase of a Unit of Work (UOW). Phases progress sequentially
/// following the compliance gate rules defined in the `ComplianceGate` trait.
///
/// # Phase Progression
///
/// The AI-DLC follows these phases in order:
///
/// 1. **Idle** - Initial state, no active work
/// 2. **Research** - Knowledge gathering, requirements definition
/// 3. **Inception** - Domain architecture analysis
/// 4. **Design** - Architecture decision records (ADRs)
/// 5. **Planning** - Request for Change (RFC) and bolt planning
/// 6. **Construction** - Implementation and code generation
///
/// # Examples
///
/// ```rust
/// use airsspec_core::state::Phase;
///
/// let current_phase = Phase::Planning;
/// assert_eq!(current_phase, Phase::Planning);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    /// Initial state, no active work.
    Idle,

    /// Knowledge gathering and requirements definition.
    Research,

    /// Domain architecture analysis.
    Inception,

    /// Architecture decision records (ADRs).
    Design,

    /// Request for Change (RFC) and bolt planning.
    Planning,

    /// Implementation and code generation.
    Construction,
}

/// State of a Unit of Work (UOW).
///
/// Tracks the current state of a UOW including its ID, current phase, and timestamps.
///
/// # Fields
///
/// * `id` - Unique identifier for the UOW
/// * `phase` - Current phase in the AI-DLC
/// * `created_at` - Timestamp when the UOW was created
/// * `updated_at` - Timestamp of the last state update
///
/// # Examples
///
/// ```rust
/// use airsspec_core::state::{Phase, UowState};
/// use chrono::Utc;
///
/// let state = UowState {
///     id: "UOW-001".to_string(),
///     phase: Phase::Planning,
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UowState {
    /// Unique identifier for the UOW.
    pub id: String,

    /// Current phase in the AI-DLC.
    pub phase: Phase,

    /// Timestamp when the UOW was created.
    pub created_at: DateTime<Utc>,

    /// Timestamp of the last state update.
    pub updated_at: DateTime<Utc>,
}

impl UowState {
    /// Creates a new `UowState` with the given ID and phase.
    ///
    /// Both `created_at` and `updated_at` are set to the current time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::state::{Phase, UowState};
    ///
    /// let state = UowState::new("UOW-001", Phase::Idle);
    /// assert_eq!(state.id, "UOW-001");
    /// assert_eq!(state.phase, Phase::Idle);
    /// ```
    pub fn new(id: impl Into<String>, phase: Phase) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            phase,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the phase and sets `updated_at` to the current time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::state::{Phase, UowState};
    ///
    /// let mut state = UowState::new("UOW-001", Phase::Planning);
    /// state.update_phase(Phase::Construction);
    /// assert_eq!(state.phase, Phase::Construction);
    /// ```
    pub fn update_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.updated_at = Utc::now();
    }
}

/// Record of a phase transition.
///
/// Tracks the history of phase transitions for a UOW, including timestamps and optional reasons.
///
/// # Fields
///
/// * `from` - The source phase before the transition
/// * `to` - The destination phase after the transition
/// * `at` - Timestamp when the transition occurred
/// * `reason` - Optional reason for the transition
///
/// # Examples
///
/// ```rust
/// use airsspec_core::state::{Phase, Transition};
/// use chrono::Utc;
///
/// let transition = Transition {
///     from: Phase::Planning,
///     to: Phase::Construction,
///     at: Utc::now(),
///     reason: Some("All planning artifacts approved".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    /// The source phase before the transition.
    pub from: Phase,

    /// The destination phase after the transition.
    pub to: Phase,

    /// Timestamp when the transition occurred.
    pub at: DateTime<Utc>,

    /// Optional reason for the transition.
    pub reason: Option<String>,
}

impl Transition {
    /// Creates a new `Transition` from one phase to another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::state::{Phase, Transition};
    ///
    /// let transition = Transition::new(Phase::Planning, Phase::Construction);
    /// assert_eq!(transition.from, Phase::Planning);
    /// assert_eq!(transition.to, Phase::Construction);
    /// assert!(transition.reason.is_none());
    /// ```
    #[must_use]
    pub fn new(from: Phase, to: Phase) -> Self {
        Self {
            from,
            to,
            at: Utc::now(),
            reason: None,
        }
    }

    /// Creates a new `Transition` with a reason.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use airsspec_core::state::{Phase, Transition};
    ///
    /// let transition = Transition::with_reason(
    ///     Phase::Planning,
    ///     Phase::Construction,
    ///     "All artifacts approved"
    /// );
    /// assert_eq!(transition.reason, Some("All artifacts approved".to_string()));
    /// ```
    #[must_use]
    pub fn with_reason(from: Phase, to: Phase, reason: impl Into<String>) -> Self {
        Self {
            from,
            to,
            at: Utc::now(),
            reason: Some(reason.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_copy() {
        let phase1 = Phase::Planning;
        let phase2 = phase1; // Phase is Copy
        assert_eq!(phase1, phase2);
    }

    #[test]
    fn test_phase_partial_eq() {
        assert_eq!(Phase::Planning, Phase::Planning);
        assert_ne!(Phase::Planning, Phase::Construction);
    }

    #[test]
    fn test_uow_state_new() {
        let state = UowState::new("UOW-001", Phase::Idle);
        assert_eq!(state.id, "UOW-001");
        assert_eq!(state.phase, Phase::Idle);
        assert_eq!(state.created_at, state.updated_at);
    }

    #[test]
    fn test_uow_state_update_phase() {
        let mut state = UowState::new("UOW-001", Phase::Planning);
        let original_updated_at = state.updated_at;

        // Small delay to ensure timestamp changes
        std::thread::sleep(std::time::Duration::from_millis(10));
        state.update_phase(Phase::Construction);

        assert_eq!(state.phase, Phase::Construction);
        assert!(state.updated_at > original_updated_at);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_uow_state_serialize_deserialize() {
        let state = UowState::new("UOW-001", Phase::Planning);
        let serialized = serde_json::to_string(&state).unwrap();
        let deserialized: UowState = serde_json::from_str(&serialized).unwrap();

        assert_eq!(state.id, deserialized.id);
        assert_eq!(state.phase, deserialized.phase);
    }

    #[test]
    fn test_transition_new() {
        let transition = Transition::new(Phase::Planning, Phase::Construction);
        assert_eq!(transition.from, Phase::Planning);
        assert_eq!(transition.to, Phase::Construction);
        assert!(transition.reason.is_none());
    }

    #[test]
    fn test_transition_with_reason() {
        let transition = Transition::with_reason(
            Phase::Planning,
            Phase::Construction,
            "All artifacts approved",
        );
        assert_eq!(
            transition.reason,
            Some("All artifacts approved".to_string())
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_transition_serialize_deserialize() {
        let transition = Transition::new(Phase::Planning, Phase::Construction);
        let serialized = serde_json::to_string(&transition).unwrap();
        let deserialized: Transition = serde_json::from_str(&serialized).unwrap();

        assert_eq!(transition.from, deserialized.from);
        assert_eq!(transition.to, deserialized.to);
    }
}
