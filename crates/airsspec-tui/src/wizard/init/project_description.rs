//! # Project Description Step
//!
//! The second step of the init wizard, collecting an optional project description.
//!
//! Always valid (description is optional). Pressing Backspace on an empty input
//! navigates back to the previous step.

// Layer 2: External crates
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

// Layer 3: Internal crates/modules
use crate::theme::styles;
use crate::widgets::TextInput;
use crate::wizard::step::{StepResult, WizardStep};

/// Project description input step for the init wizard.
///
/// Collects an optional project description. This step is always valid
/// since the description field is not required. Pressing Backspace on
/// an empty input returns [`StepResult::Previous`] to go back.
#[derive(Debug)]
pub struct ProjectDescriptionStep {
    input: TextInput,
}

impl ProjectDescriptionStep {
    /// Creates a new project description step with default configuration.
    #[must_use]
    pub fn new() -> Self {
        let mut input = TextInput::new()
            .placeholder("A brief description of your project")
            .max_length(500);
        input.set_focused(true);
        Self { input }
    }

    /// Returns the current description value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.input.get_value()
    }
}

impl Default for ProjectDescriptionStep {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(
    clippy::unnecessary_literal_bound,
    reason = "trait returns &str; implementations return static literals"
)]
impl WizardStep for ProjectDescriptionStep {
    fn title(&self) -> &str {
        "Project Description"
    }

    fn help_text(&self) -> &str {
        "Describe what your project does (optional)"
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Label
                Constraint::Length(3), // Input field (with block border)
                Constraint::Length(1), // Help text
                Constraint::Min(0),    // Remaining space
            ])
            .split(area);

        // Label without required indicator (optional field)
        let label = Line::from(Span::styled("Project Description", styles::title()));
        Paragraph::new(label).render(chunks[0], buf);

        // Input field with block border
        self.input.render_with_block(chunks[1], buf, "");

        // Help text
        let help = Paragraph::new(Line::from(Span::styled(self.help_text(), styles::muted())));
        help.render(chunks[2], buf);
    }

    fn handle_key(&mut self, key: KeyEvent) -> StepResult {
        match key.code {
            KeyCode::Enter => StepResult::Next,
            KeyCode::Esc => StepResult::Cancel,
            KeyCode::Backspace if self.input.is_empty() => StepResult::Previous,
            _ => {
                self.input.handle_key(key);
                StepResult::Continue
            }
        }
    }

    fn is_valid(&self) -> bool {
        // Description is optional, always valid
        true
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyEventKind, KeyEventState, KeyModifiers};

    use super::*;

    /// Helper to create a key press event.
    fn key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_is_valid_always_true() {
        let step = ProjectDescriptionStep::new();
        // Empty description is valid (optional field)
        assert!(step.is_valid());
    }

    #[test]
    fn test_backspace_on_empty_returns_previous() {
        let mut step = ProjectDescriptionStep::new();
        assert!(step.input.is_empty());

        let result = step.handle_key(key_event(KeyCode::Backspace));
        assert_eq!(result, StepResult::Previous);
    }
}
