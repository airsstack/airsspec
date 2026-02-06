//! # Widgets
//!
//! Reusable TUI input components for interactive forms and wizards.
//!
//! ## Available Widgets
//!
//! - [`TextInput`] -- Single-line text input with cursor management,
//!   placeholder text, max length enforcement, and horizontal scrolling.

mod text_input;

pub use text_input::TextInput;
