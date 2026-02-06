//! # Text Input Widget
//!
//! A single-line text input with cursor management, keyboard handling,
//! placeholder text display, max length enforcement, and horizontal scrolling.
//!
//! ## Design Note
//!
//! `TextInput` does **not** implement the `ratatui::widgets::Widget` trait directly
//! because `Widget::render` has a consuming signature (`fn render(self, ...)`).
//! Since `TextInput` retains state across renders (cursor position, value, focus),
//! consuming `self` on each render would require cloning. Instead, we provide
//! [`TextInput::render_with_block`] and [`TextInput::render_content`] methods
//! that take `&self` and write directly to the buffer.

// Layer 1: Standard library
use std::cmp::min;

// Layer 2: External crates
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

// Layer 3: Internal crates/modules
use crate::theme::{colors, styles};

/// A single-line text input widget with cursor, placeholder, and max length.
///
/// Supports keyboard navigation (Left, Right, Home, End), character insertion
/// at cursor position, deletion (Backspace, Delete), and horizontal scrolling
/// when the input value exceeds the visible area width.
///
/// # Builder Pattern
///
/// ```ignore
/// let input = TextInput::new()
///     .placeholder("Enter a value")
///     .max_length(100);
/// ```
#[derive(Debug, Clone)]
pub struct TextInput {
    value: String,
    cursor: usize,
    placeholder: String,
    focused: bool,
    max_length: usize,
}

impl TextInput {
    /// Creates a new empty text input with default settings.
    ///
    /// The input starts unfocused with no placeholder and a max length of 256.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            placeholder: String::new(),
            focused: false,
            max_length: 256,
        }
    }

    /// Sets the placeholder text displayed when the input is empty.
    #[must_use]
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Sets the maximum allowed character count.
    #[must_use]
    pub fn max_length(mut self, length: usize) -> Self {
        self.max_length = length;
        self
    }

    /// Sets the initial value and positions the cursor at the end.
    #[must_use]
    pub fn value(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.cursor = text.len();
        self.value = text;
        self
    }

    /// Sets the focus state of this input.
    ///
    /// When unfocused, keyboard events are ignored.
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Returns whether this input is currently focused.
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Returns the current text value.
    #[must_use]
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Returns whether the input value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Handles a keyboard event, modifying the input state.
    ///
    /// Returns `true` if the event was consumed (input was focused and key
    /// was recognized), `false` otherwise.
    ///
    /// # Supported Keys
    ///
    /// - `Char(c)` -- Insert character at cursor position
    /// - `Backspace` -- Delete character before cursor
    /// - `Delete` -- Delete character at cursor
    /// - `Left` / `Right` -- Move cursor one position
    /// - `Home` / `End` -- Jump to start / end
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if !self.focused {
            return false;
        }

        match key.code {
            KeyCode::Char(c) => {
                if self.value.len() < self.max_length {
                    self.value.insert(self.cursor, c);
                    self.cursor += 1;
                }
                true
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
                true
            }
            KeyCode::Delete => {
                if self.cursor < self.value.len() {
                    self.value.remove(self.cursor);
                }
                true
            }
            KeyCode::Left => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }
            KeyCode::Right => {
                if self.cursor < self.value.len() {
                    self.cursor += 1;
                }
                true
            }
            KeyCode::Home => {
                self.cursor = 0;
                true
            }
            KeyCode::End => {
                self.cursor = self.value.len();
                true
            }
            _ => false,
        }
    }

    /// Renders the input with a titled block border.
    ///
    /// Draws a bordered container with the given title, then renders
    /// the input content inside. The border color reflects focus state.
    pub fn render_with_block(&self, area: Rect, buf: &mut Buffer, title: &str) {
        let border_style = if self.focused {
            ratatui::style::Style::default().fg(colors::BORDER_ACTIVE)
        } else {
            ratatui::style::Style::default().fg(colors::BORDER)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        block.render(area, buf);
        self.render_content(inner, buf);
    }

    /// Renders the input content (value or placeholder) into the given area.
    ///
    /// Handles horizontal scrolling when the cursor moves past the visible
    /// width, and highlights the cursor position when focused.
    pub fn render_content(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let width = area.width as usize;

        if self.value.is_empty() {
            // Show placeholder text when empty
            let display = if self.placeholder.len() > width {
                &self.placeholder[..width]
            } else {
                &self.placeholder
            };
            buf.set_string(area.x, area.y, display, styles::muted());
        } else {
            // Compute horizontal scroll offset
            let scroll_offset = if self.cursor > width.saturating_sub(1) {
                self.cursor.saturating_sub(width.saturating_sub(1))
            } else {
                0
            };

            let end = min(scroll_offset + width, self.value.len());
            let visible = &self.value[scroll_offset..end];
            buf.set_string(area.x, area.y, visible, styles::input());
        }

        // Highlight cursor position when focused
        if self.focused {
            let scroll_offset = if self.cursor > width.saturating_sub(1) {
                self.cursor.saturating_sub(width.saturating_sub(1))
            } else {
                0
            };
            #[allow(
                clippy::cast_possible_truncation,
                reason = "cursor offset is bounded by terminal width (u16)"
            )]
            let cursor_x = area.x + (self.cursor - scroll_offset) as u16;
            if cursor_x < area.x + area.width {
                buf[(cursor_x, area.y)].set_style(styles::input_active());
            }
        }
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
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
    fn test_new_input() {
        let input = TextInput::new();
        assert!(input.is_empty());
        assert!(!input.is_focused());
        assert_eq!(input.get_value(), "");
    }

    #[test]
    fn test_placeholder() {
        let input = TextInput::new().placeholder("type here");
        assert!(input.is_empty());
        assert_eq!(input.get_value(), "");
        // Placeholder is stored but doesn't affect value
        assert_eq!(input.placeholder, "type here");
    }

    #[test]
    fn test_initial_value() {
        let input = TextInput::new().value("hello");
        assert!(!input.is_empty());
        assert_eq!(input.get_value(), "hello");
        // Cursor should be at end of initial value
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_char_input() {
        let mut input = TextInput::new();
        input.set_focused(true);

        let consumed = input.handle_key(key_event(KeyCode::Char('a')));
        assert!(consumed);
        assert_eq!(input.get_value(), "a");

        input.handle_key(key_event(KeyCode::Char('b')));
        input.handle_key(key_event(KeyCode::Char('c')));
        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_backspace() {
        let mut input = TextInput::new().value("hello");
        input.set_focused(true);

        input.handle_key(key_event(KeyCode::Backspace));
        assert_eq!(input.get_value(), "hell");
        assert_eq!(input.cursor, 4);

        // Backspace at start does nothing
        input.cursor = 0;
        input.handle_key(key_event(KeyCode::Backspace));
        assert_eq!(input.get_value(), "hell");
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_max_length() {
        let mut input = TextInput::new().max_length(3);
        input.set_focused(true);

        input.handle_key(key_event(KeyCode::Char('a')));
        input.handle_key(key_event(KeyCode::Char('b')));
        input.handle_key(key_event(KeyCode::Char('c')));
        assert_eq!(input.get_value(), "abc");

        // Fourth character should be rejected
        input.handle_key(key_event(KeyCode::Char('d')));
        assert_eq!(input.get_value(), "abc");
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_cursor_movement() {
        let mut input = TextInput::new().value("abc");
        input.set_focused(true);

        // Cursor starts at end (3)
        assert_eq!(input.cursor, 3);

        input.handle_key(key_event(KeyCode::Left));
        assert_eq!(input.cursor, 2);

        input.handle_key(key_event(KeyCode::Left));
        assert_eq!(input.cursor, 1);

        // Insert at cursor position
        input.handle_key(key_event(KeyCode::Char('X')));
        assert_eq!(input.get_value(), "aXbc");
        assert_eq!(input.cursor, 2);

        input.handle_key(key_event(KeyCode::Right));
        assert_eq!(input.cursor, 3);
    }

    #[test]
    fn test_home_end() {
        let mut input = TextInput::new().value("hello");
        input.set_focused(true);

        input.handle_key(key_event(KeyCode::Home));
        assert_eq!(input.cursor, 0);

        input.handle_key(key_event(KeyCode::End));
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_unfocused_ignores_input() {
        let mut input = TextInput::new();
        // Not focused by default

        let consumed = input.handle_key(key_event(KeyCode::Char('a')));
        assert!(!consumed);
        assert!(input.is_empty());

        let consumed = input.handle_key(key_event(KeyCode::Backspace));
        assert!(!consumed);
    }
}
