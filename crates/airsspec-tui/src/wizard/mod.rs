//! # Wizard
//!
//! Multi-step wizard framework for interactive terminal forms.
//!
//! ## Components
//!
//! - [`step`] -- `WizardStep` trait contract and `StepResult` message enum
//! - [`state`] -- `WizardState` navigation tracker
//! - [`init`] -- Init wizard steps and `InitWizardResult` type
//! - [`runner`] -- `run_init_wizard()` terminal lifecycle and event loop
//!
//! ## Architecture
//!
//! Follows a simplified Elm Architecture (TEA) pattern where `WizardState`
//! and step structs serve as the Model, `StepResult` as Messages, the
//! runner's match logic as Update, and step render methods as View.
//!
//! ## Usage
//!
//! ```ignore
//! use airsspec_tui::wizard::run_init_wizard;
//!
//! let result = run_init_wizard()?;
//! match result {
//!     Some(config) => println!("Creating project: {}", config.project_name),
//!     None => println!("Wizard cancelled"),
//! }
//! ```

pub mod init;
pub mod runner;
pub mod state;
pub mod step;

pub use init::InitWizardResult;
pub use runner::run_init_wizard;
pub use state::WizardState;
pub use step::{StepResult, WizardStep};
