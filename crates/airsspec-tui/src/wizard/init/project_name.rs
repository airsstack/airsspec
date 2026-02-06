//! # Project Name Step
//!
//! The first step of the init wizard, collecting the required project name.
//!
//! Validates that the input is non-empty before allowing forward navigation.
//! Uses a [`TextInput`] widget with a placeholder and max length constraint.

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

/// Project name input step for the init wizard.
///
/// Collects a required project name from the user. The step is only
/// valid when the input contains at least one character.
#[derive(Debug)]
pub struct ProjectNameStep {
    input: TextInput,
}

impl ProjectNameStep {
    /// Creates a new project name step with default configuration.
    #[must_use]
    pub fn new() -> Self {
        let mut input = TextInput::new().placeholder("my-project").max_length(100);
        input.set_focused(true);
        Self { input }
    }

    /// Returns the current project name value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.input.get_value()
    }
}

impl Default for ProjectNameStep {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(
    clippy::unnecessary_literal_bound,
    reason = "trait returns &str; implementations return static literals"
)]
impl WizardStep for ProjectNameStep {
    fn title(&self) -> &str {
        "Project Name"
    }

    fn help_text(&self) -> &str {
        "Enter a name for your project"
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

        // Label with required indicator
        let label = Line::from(vec![
            Span::styled("Project Name", styles::title()),
            Span::styled(" *", styles::error()),
        ]);
        Paragraph::new(label).render(chunks[0], buf);

        // Input field with block border
        self.input.render_with_block(chunks[1], buf, "");

        // Help text
        let help = Paragraph::new(Line::from(Span::styled(self.help_text(), styles::muted())));
        help.render(chunks[2], buf);
    }

    fn handle_key(&mut self, key: KeyEvent) -> StepResult {
        match key.code {
            KeyCode::Enter => {
                if self.is_valid() {
                    StepResult::Next
                } else {
                    StepResult::Continue
                }
            }
            KeyCode::Esc => StepResult::Cancel,
            _ => {
                self.input.handle_key(key);
                StepResult::Continue
            }
        }
    }

    fn is_valid(&self) -> bool {
        !self.input.is_empty()
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
    fn test_is_valid_empty() {
        let step = ProjectNameStep::new();
        assert!(!step.is_valid());
    }

    #[test]
    fn test_is_valid_with_value() {
        let mut step = ProjectNameStep::new();
        step.handle_key(key_event(KeyCode::Char('a')));
        assert!(step.is_valid());
        assert_eq!(step.value(), "a");
    }
}
