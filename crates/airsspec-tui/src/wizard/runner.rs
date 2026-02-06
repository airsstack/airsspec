//! # Wizard Runner
//!
//! The main entry point for executing the init wizard in the terminal.
//!
//! Manages the terminal lifecycle (raw mode, alternate screen), installs
//! a panic hook for safe terminal restoration, drives the event loop,
//! and returns the wizard result.
//!
//! ## Terminal Safety
//!
//! A panic hook is installed before the event loop to ensure the terminal
//! is restored to normal mode if a panic occurs. On clean exit (completion
//! or cancel), the original panic hook is restored so that subsequent panics
//! in the application use the default behavior.
//!
//! ## Architecture
//!
//! Follows a simplified Elm Architecture (TEA) pattern:
//! - **Model**: `WizardState` + concrete step structs
//! - **Messages**: `StepResult` enum variants
//! - **Update**: Runner's match on `StepResult` to modify state
//! - **View**: Step render methods + runner layout chrome

// Layer 1: Standard library
use std::io;

// Layer 2: External crates
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

// Layer 3: Internal crates/modules
use crate::theme::styles;
use crate::wizard::init::{
    ConfirmationStep, InitWizardResult, ProjectDescriptionStep, ProjectNameStep,
};
use crate::wizard::state::WizardState;
use crate::wizard::step::{StepResult, WizardStep};

/// Runs the init wizard in an interactive terminal session.
///
/// Enters raw mode and the alternate screen, presents a multi-step wizard
/// for collecting project configuration, and returns the result.
///
/// Returns `Ok(Some(result))` if the user completes the wizard, or
/// `Ok(None)` if the user cancels (Esc or Ctrl+C).
///
/// # Errors
///
/// Returns `io::Error` if terminal initialization, event reading,
/// or cleanup fails.
pub fn run_init_wizard() -> io::Result<Option<InitWizardResult>> {
    // --- Terminal Setup ---
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // --- Install panic hook for terminal restoration ---
    // This ensures the terminal is restored if a panic occurs during the
    // wizard, preventing the terminal from being left in raw/alternate mode.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // --- Initialize wizard state and steps ---
    let mut state = WizardState::new(3);
    let mut name_step = ProjectNameStep::new();
    let mut desc_step = ProjectDescriptionStep::new();
    let mut confirm_step = ConfirmationStep::new(String::new(), String::new());

    let result = run_event_loop(
        &mut terminal,
        &mut state,
        &mut name_step,
        &mut desc_step,
        &mut confirm_step,
    );

    // --- Teardown: restore terminal and panic hook on ALL exit paths ---
    // Remove the custom panic hook and restore default behavior.
    // We cannot restore the original_hook because it was moved into the
    // closure, so we take_hook (drops the custom one) and let Rust install
    // its default hook.
    let _ = std::panic::take_hook();

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

/// Drives the wizard event loop until completion or cancellation.
///
/// Separated from `run_init_wizard` to keep terminal lifecycle management
/// clean and ensure teardown always runs regardless of how this returns.
fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut WizardState,
    name_step: &mut ProjectNameStep,
    desc_step: &mut ProjectDescriptionStep,
    confirm_step: &mut ConfirmationStep,
) -> io::Result<Option<InitWizardResult>> {
    loop {
        // Update confirmation step with latest values when navigating to it
        if state.current() == 2 {
            confirm_step.update(name_step.value().to_owned(), desc_step.value().to_owned());
        }

        // --- Draw UI ---
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(0),    // Content
                    Constraint::Length(3), // Footer
                ])
                .split(frame.area());

            // Header: step counter
            let step_title = match state.current() {
                0 => name_step.title(),
                1 => desc_step.title(),
                _ => confirm_step.title(),
            };
            let header_text = format!(
                " AirsSpec Init - Step {}/{}: {} ",
                state.display_step(),
                state.total(),
                step_title,
            );
            let header = Paragraph::new(Line::from(Span::styled(&header_text, styles::title())))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(styles::title()),
                );
            frame.render_widget(header, chunks[0]);

            // Content: current step
            match state.current() {
                0 => name_step.render(chunks[1], frame.buffer_mut()),
                1 => desc_step.render(chunks[1], frame.buffer_mut()),
                _ => confirm_step.render(chunks[1], frame.buffer_mut()),
            }

            // Footer: key hints
            let mut hints = vec![Span::styled(" Enter", styles::key_hint()), Span::raw(": ")];

            if state.is_last() {
                hints.push(Span::raw("Create"));
            } else {
                hints.push(Span::raw("Next"));
            }

            hints.push(Span::raw("  "));

            if !state.is_first() {
                hints.push(Span::styled("Backspace", styles::key_hint()));
                hints.push(Span::raw(": Back  "));
            }

            hints.push(Span::styled("Esc", styles::key_hint()));
            hints.push(Span::raw(": Cancel"));

            let footer = Paragraph::new(Line::from(hints)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(styles::muted()),
            );
            frame.render_widget(footer, chunks[2]);
        })?;

        // --- Handle input events ---
        let ev = event::read()?;
        if let Event::Key(key) = ev {
            // Only process key press events to avoid double-handling
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // Global Ctrl+C handler
            if key.code == KeyCode::Char('c')
                && key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
            {
                return Ok(None);
            }

            // Dispatch to current step using index matching (no dyn dispatch)
            let step_result = match state.current() {
                0 => name_step.handle_key(key),
                1 => desc_step.handle_key(key),
                _ => confirm_step.handle_key(key),
            };

            // Process step result
            match step_result {
                StepResult::Continue => {}
                StepResult::Next => {
                    if state.is_last() {
                        // Wizard complete
                        return Ok(Some(InitWizardResult {
                            project_name: name_step.value().to_owned(),
                            project_description: desc_step.value().to_owned(),
                        }));
                    }
                    state.next();
                }
                StepResult::Previous => {
                    state.previous();
                }
                StepResult::Cancel => {
                    return Ok(None);
                }
            }
        }
    }
}
