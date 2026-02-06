//! # Confirmation Step
//!
//! The final step of the init wizard, displaying a summary of the collected
//! project information and allowing the user to confirm or go back.
//!
//! Shows the project name, description, and workspace directory that will
//! be created.

// Layer 2: External crates
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

// Layer 3: Internal crates/modules
use crate::theme::styles;
use crate::wizard::step::{StepResult, WizardStep};

/// Confirmation step displaying a summary of the wizard inputs.
///
/// Renders a bordered summary panel showing the project name, description,
/// and workspace directory. The user can confirm with Enter, go back with
/// Backspace, or cancel with Esc.
#[derive(Debug)]
pub struct ConfirmationStep {
    project_name: String,
    project_description: String,
}

impl ConfirmationStep {
    /// Creates a new confirmation step with the given project details.
    #[must_use]
    pub fn new(project_name: String, project_description: String) -> Self {
        Self {
            project_name,
            project_description,
        }
    }

    /// Updates the project details shown in the summary.
    ///
    /// Called by the runner when navigating to this step to reflect
    /// the latest values from the input steps.
    pub fn update(&mut self, name: String, description: String) {
        self.project_name = name;
        self.project_description = description;
    }
}

#[allow(
    clippy::unnecessary_literal_bound,
    reason = "trait returns &str; implementations return static literals"
)]
impl WizardStep for ConfirmationStep {
    fn title(&self) -> &str {
        "Confirm"
    }

    fn help_text(&self) -> &str {
        "Press Enter to create workspace, Esc to cancel, or Backspace to go back"
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let description_display = if self.project_description.is_empty() {
            "(none)"
        } else {
            &self.project_description
        };

        let summary_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Project Name:  ", styles::muted()),
                Span::styled(&self.project_name, styles::title()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Description:   ", styles::muted()),
                Span::raw(description_display),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Directory:     ", styles::muted()),
                Span::raw(".airsspec/"),
            ]),
            Line::from(""),
        ];

        let block = Block::default()
            .title(" Summary ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(styles::title());

        let paragraph = Paragraph::new(summary_lines).block(block);
        paragraph.render(area, buf);
    }

    fn handle_key(&mut self, key: KeyEvent) -> StepResult {
        match key.code {
            KeyCode::Enter => StepResult::Next,
            KeyCode::Esc => StepResult::Cancel,
            KeyCode::Backspace => StepResult::Previous,
            _ => StepResult::Continue,
        }
    }

    fn is_valid(&self) -> bool {
        !self.project_name.is_empty()
    }
}
