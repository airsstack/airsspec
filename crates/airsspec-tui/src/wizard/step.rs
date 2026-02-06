//! # Wizard Step
//!
//! Defines the [`WizardStep`] trait contract and [`StepResult`] enum for
//! step-to-runner communication in the wizard framework.
//!
//! ## Architecture
//!
//! The `WizardStep` trait mirrors the Elm Architecture (TEA) Component pattern
//! with `render` (view), `handle_key` (update), and `is_valid` (validation).
//! The trait exists to define a clear contract for wizard step implementations.
//!
//! **Important:** The wizard runner dispatches to steps via concrete index
//! matching, not `dyn WizardStep` trait objects. The trait is for contract
//! clarity, not dynamic dispatch.

// Layer 2: External crates
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

/// Result of handling a key event in a wizard step.
///
/// Communicates the step's intention back to the wizard runner,
/// analogous to a "message" in the Elm Architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
    /// Key was handled but no navigation change is needed.
    Continue,
    /// Step is complete; advance to the next step (or finish if last).
    Next,
    /// Navigate back to the previous step.
    Previous,
    /// Cancel the entire wizard.
    Cancel,
}

/// Contract for a wizard step that can render itself, handle keyboard
/// input, and report its validation state.
///
/// Each step owns its input state and renders it in immediate mode
/// (rebuilding the UI from state each frame).
pub trait WizardStep {
    /// Returns the display title for this step.
    fn title(&self) -> &str;

    /// Returns contextual help text shown in the step content area.
    fn help_text(&self) -> &str;

    /// Renders the step content into the given buffer area.
    ///
    /// This is called each frame by the wizard runner. The implementation
    /// should rebuild the entire UI from current state (immediate mode).
    fn render(&self, area: Rect, buf: &mut Buffer);

    /// Handles a keyboard event and returns a [`StepResult`] indicating
    /// what the runner should do next.
    fn handle_key(&mut self, key: KeyEvent) -> StepResult;

    /// Returns whether the step's current input is valid.
    ///
    /// The runner checks this before allowing forward navigation.
    fn is_valid(&self) -> bool;
}
