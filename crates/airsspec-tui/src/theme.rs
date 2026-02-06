//! # Theme
//!
//! Centralized color palette and reusable style definitions for consistent
//! visual styling across all TUI components.
//!
//! Uses Catppuccin-inspired RGB colors for modern terminal aesthetics with
//! precise cross-terminal rendering.
//!
//! ## Sub-modules
//!
//! - [`colors`] -- Named color constants using `Color::Rgb`
//! - [`styles`] -- Composable style functions returning `ratatui::style::Style`

// Layer 2: External crates
use ratatui::style::{Color, Modifier, Style};

/// Named color constants for the `AirsSpec` TUI theme.
///
/// All colors use `Color::Rgb(r, g, b)` for consistent rendering
/// across different terminal emulators.
pub mod colors {
    use super::Color;

    /// Primary accent color (blue).
    pub const PRIMARY: Color = Color::Rgb(137, 180, 250);

    /// Success indicator color (green).
    pub const SUCCESS: Color = Color::Rgb(166, 227, 161);

    /// Warning indicator color (yellow).
    pub const WARNING: Color = Color::Rgb(249, 226, 175);

    /// Error indicator color (red).
    pub const ERROR: Color = Color::Rgb(243, 139, 168);

    /// Muted text color (gray) for help text and placeholders.
    pub const MUTED: Color = Color::Rgb(147, 153, 178);

    /// Highlight background color (dark surface).
    pub const HIGHLIGHT_BG: Color = Color::Rgb(49, 50, 68);

    /// Default border color.
    pub const BORDER: Color = Color::Rgb(88, 91, 112);

    /// Active/focused border color.
    pub const BORDER_ACTIVE: Color = Color::Rgb(137, 180, 250);
}

/// Composable style functions for consistent text formatting.
///
/// Each function returns a `ratatui::style::Style` value that can be
/// applied to widgets and text spans.
pub mod styles {
    use super::{Modifier, Style, colors};

    /// Default text style with no special formatting.
    #[must_use]
    pub fn default() -> Style {
        Style::default()
    }

    /// Title text style -- primary color with bold modifier.
    #[must_use]
    pub fn title() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    /// Muted text style for help text and secondary information.
    #[must_use]
    pub fn muted() -> Style {
        Style::default().fg(colors::MUTED)
    }

    /// Input text style for user-entered content.
    #[must_use]
    pub fn input() -> Style {
        Style::default()
    }

    /// Active input style for cursor position highlighting.
    #[must_use]
    pub fn input_active() -> Style {
        Style::default()
            .bg(colors::HIGHLIGHT_BG)
            .add_modifier(Modifier::BOLD)
    }

    /// Success text style (green).
    #[must_use]
    pub fn success() -> Style {
        Style::default().fg(colors::SUCCESS)
    }

    /// Error text style (red with bold).
    #[must_use]
    pub fn error() -> Style {
        Style::default()
            .fg(colors::ERROR)
            .add_modifier(Modifier::BOLD)
    }

    /// Warning text style (yellow).
    #[must_use]
    pub fn warning() -> Style {
        Style::default().fg(colors::WARNING)
    }

    /// Key hint style for keyboard shortcut labels.
    #[must_use]
    pub fn key_hint() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }
}
