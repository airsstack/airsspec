//! # Init Wizard
//!
//! Steps and result type for the project initialization wizard.
//!
//! The init wizard collects project configuration through a multi-step
//! interactive form:
//!
//! 1. [`ProjectNameStep`] -- Required project name input
//! 2. [`ProjectDescriptionStep`] -- Optional project description input
//! 3. [`ConfirmationStep`] -- Summary and confirmation
//!
//! On completion, the wizard produces an [`InitWizardResult`] containing
//! the collected values.

mod confirmation;
mod project_description;
mod project_name;

pub use confirmation::ConfirmationStep;
pub use project_description::ProjectDescriptionStep;
pub use project_name::ProjectNameStep;

/// Result of a completed init wizard containing the user's input.
///
/// Returned by [`super::run_init_wizard`] when the user confirms
/// the wizard (as opposed to cancelling, which returns `None`).
#[derive(Debug, Clone)]
pub struct InitWizardResult {
    /// The project name entered by the user.
    pub project_name: String,
    /// The project description entered by the user (may be empty).
    pub project_description: String,
}
