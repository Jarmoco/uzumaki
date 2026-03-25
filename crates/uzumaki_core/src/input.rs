use std::time::Instant;
use winit::keyboard::{Key, NamedKey};

use crate::text_model::TextModel;

// ── Selection ────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct TextSelection {
    /// Anchor point (where selection started), flat grapheme index
    pub anchor: usize,
    /// Active point / cursor position, flat grapheme index
    pub active: usize,
}

impl TextSelection {
    pub fn new(anchor: usize, active: usize) -> Self {
        Self { anchor, active }
    }

    pub fn is_collapsed(&self) -> bool {
        self.anchor == self.active
    }

    pub fn start(&self) -> usize {
        self.anchor.min(self.active)
    }

    pub fn end(&self) -> usize {
        self.anchor.max(self.active)
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.anchor = pos;
        self.active = pos;
    }
}

// ── EditEvent ────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum EditKind {
    Insert,
    DeleteBackward,
    DeleteForward,
    DeleteWordBackward,
    DeleteWordForward,
}

#[derive(Clone, Debug)]
pub struct EditEvent {
    pub kind: EditKind,
    pub inserted: Option<String>,
}

// ── KeyResult ────────────────────────────────────────────────────────

pub enum KeyResult {
    Edit(EditEvent),
    Blur,
    Handled,
    Ignored,
}

// ── InputState ───────────────────────────────────────────────────────
// Owns selection, delegates buffer mutations to TextModel.
// Handles key events, movement, and presentation concerns.

pub struct InputState {
    pub model: TextModel,
    pub selection: TextSelection,
    pub placeholder: String,
    pub scroll_offset: f32,
    pub scroll_offset_y: f32,
    pub focused: bool,
    pub blink_reset: Instant,
    pub disabled: bool,
    pub secure: bool,
    pub multiline: bool,
    /// Preserved column for vertical navigation (sticky column in grapheme units).
    pub sticky_col: Option<usize>,
    /// Preserved X coordinate for vertical navigation.
    pub sticky_x: Option<f32>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            model: TextModel::new(),
            selection: TextSelection::default(),
            placeholder: String::new(),
            scroll_offset: 0.0,
            scroll_offset_y: 0.0,
            focused: false,
            blink_reset: Instant::now(),
            disabled: false,
            secure: false,
            multiline: false,
            sticky_col: None,
            sticky_x: None,
        }
    }

    // ── Selection helpers ────────────────────────────────────────────

    /// Delete the current selection. Returns true if something was deleted.
    fn delete_selection(&mut self) -> bool {
        if self.selection.is_collapsed() {
            return false;
        }
        let start = self.selection.start();
        let end = self.selection.end();
        self.model.delete_range(start, end);
        self.selection.set_cursor(start);
        true
    }

    /// Get the selected text.
    pub fn selected_text(&self) -> String {
        if self.selection.is_collapsed() {
            return String::new();
        }
        self.model.text_in_range(self.selection.start(), self.selection.end())
    }

    // ── Editing ──────────────────────────────────────────────────────

    pub fn insert_text(&mut self, ch: &str) -> Option<EditEvent> {
        self.sticky_x = None;
        self.sticky_col = None;
        if self.disabled {
            return None;
        }
        // Single-line: reject newlines
        let text_to_insert;
        let input = if !self.multiline {
            text_to_insert = ch
                .chars()
                .filter(|&c| c != '\n' && c != '\r')
                .collect::<String>();
            if text_to_insert.is_empty() {
                return None;
            }
            text_to_insert.as_str()
        } else {
            ch
        };

        self.delete_selection();
        let pos = self.selection.active;

        match self.model.insert(pos, input, 0) {
            Some(new_pos) => {
                self.selection.set_cursor(new_pos);
                self.reset_blink();
                Some(EditEvent {
                    kind: EditKind::Insert,
                    inserted: Some(input.to_string()),
                })
            }
            None => None,
        }
    }

    pub fn delete_backward(&mut self) -> Option<EditEvent> {
        self.sticky_x = None;
        self.sticky_col = None;
        if self.disabled {
            return None;
        }
        if self.delete_selection() {
            self.reset_blink();
            return Some(EditEvent {
                kind: EditKind::DeleteBackward,
                inserted: None,
            });
        }
        match self.model.delete_backward(self.selection.active) {
            Some(new_pos) => {
                self.selection.set_cursor(new_pos);
                self.reset_blink();
                Some(EditEvent {
                    kind: EditKind::DeleteBackward,
                    inserted: None,
                })
            }
            None => None,
        }
    }

    pub fn delete_forward(&mut self) -> Option<EditEvent> {
        self.sticky_x = None;
        self.sticky_col = None;
        if self.disabled {
            return None;
        }
        if self.delete_selection() {
            self.reset_blink();
            return Some(EditEvent {
                kind: EditKind::DeleteForward,
                inserted: None,
            });
        }
        match self.model.delete_forward(self.selection.active) {
            Some(new_pos) => {
                self.selection.set_cursor(new_pos);
                self.reset_blink();
                Some(EditEvent {
                    kind: EditKind::DeleteForward,
                    inserted: None,
                })
            }
            None => None,
        }
    }

    pub fn delete_word_backward(&mut self) -> Option<EditEvent> {
        self.sticky_x = None;
        self.sticky_col = None;
        if self.disabled {
            return None;
        }
        if self.delete_selection() {
            self.reset_blink();
            return Some(EditEvent {
                kind: EditKind::DeleteWordBackward,
                inserted: None,
            });
        }
        match self.model.delete_word_backward(self.selection.active) {
            Some(new_pos) => {
                self.selection.set_cursor(new_pos);
                self.reset_blink();
                Some(EditEvent {
                    kind: EditKind::DeleteWordBackward,
                    inserted: None,
                })
            }
            None => None,
        }
    }

    pub fn delete_word_forward(&mut self) -> Option<EditEvent> {
        self.sticky_x = None;
        self.sticky_col = None;
        if self.disabled {
            return None;
        }
        if self.delete_selection() {
            self.reset_blink();
            return Some(EditEvent {
                kind: EditKind::DeleteWordForward,
                inserted: None,
            });
        }
        match self.model.delete_word_forward(self.selection.active) {
            Some(new_pos) => {
                self.selection.set_cursor(new_pos);
                self.reset_blink();
                Some(EditEvent {
                    kind: EditKind::DeleteWordForward,
                    inserted: None,
                })
            }
            None => None,
        }
    }

    // ── Movement ─────────────────────────────────────────────────────

    pub fn move_left(&mut self, extend: bool) {
        self.sticky_x = None;
        if !extend && !self.selection.is_collapsed() {
            let pos = self.selection.start();
            self.selection.set_cursor(pos);
        } else if self.selection.active > 0 {
            self.selection.active -= 1;
            if !extend {
                self.selection.anchor = self.selection.active;
            }
        }
        self.reset_blink();
    }

    pub fn move_right(&mut self, extend: bool) {
        self.sticky_x = None;
        let count = self.model.grapheme_count();
        if !extend && !self.selection.is_collapsed() {
            let pos = self.selection.end();
            self.selection.set_cursor(pos);
        } else if self.selection.active < count {
            self.selection.active += 1;
            if !extend {
                self.selection.anchor = self.selection.active;
            }
        }
        self.reset_blink();
    }

    pub fn move_word_left(&mut self, extend: bool) {
        self.sticky_x = None;
        let pos = self.model.find_word_start(self.selection.active);
        self.selection.active = pos;
        if !extend {
            self.selection.anchor = pos;
        }
        self.reset_blink();
    }

    pub fn move_word_right(&mut self, extend: bool) {
        self.sticky_x = None;
        let pos = self.model.find_word_end(self.selection.active);
        self.selection.active = pos;
        if !extend {
            self.selection.anchor = pos;
        }
        self.reset_blink();
    }

    pub fn move_home(&mut self, extend: bool) {
        self.sticky_x = None;
        let (row, _) = self.model.flat_to_rowcol(self.selection.active);
        let flat = self.model.rowcol_to_flat(row, 0);
        self.selection.active = flat;
        if !extend {
            self.selection.anchor = flat;
        }
        self.reset_blink();
    }

    pub fn move_end(&mut self, extend: bool) {
        self.sticky_x = None;
        let (row, _) = self.model.flat_to_rowcol(self.selection.active);
        let line_len = self.model.line_grapheme_count(row);
        let flat = self.model.rowcol_to_flat(row, line_len);
        self.selection.active = flat;
        if !extend {
            self.selection.anchor = flat;
        }
        self.reset_blink();
    }

    pub fn move_absolute_home(&mut self, extend: bool) {
        self.sticky_x = None;
        self.selection.active = 0;
        if !extend {
            self.selection.anchor = 0;
        }
        self.reset_blink();
    }

    pub fn move_absolute_end(&mut self, extend: bool) {
        self.sticky_x = None;
        let count = self.model.grapheme_count();
        self.selection.active = count;
        if !extend {
            self.selection.anchor = count;
        }
        self.reset_blink();
    }

    pub fn move_up(&mut self, extend: bool, sticky_col: Option<usize>) -> bool {
        let (row, col) = self.model.flat_to_rowcol(self.selection.active);
        if row == 0 {
            self.selection.active = 0;
            if !extend {
                self.selection.anchor = 0;
            }
            return false;
        }
        let target_col = sticky_col.unwrap_or(col);
        let prev_line_len = self.model.line_grapheme_count(row - 1);
        let new_col = target_col.min(prev_line_len);
        let flat = self.model.rowcol_to_flat(row - 1, new_col);
        self.selection.active = flat;
        if !extend {
            self.selection.anchor = flat;
        }
        true
    }

    pub fn move_down(&mut self, extend: bool, sticky_col: Option<usize>) -> bool {
        let (row, col) = self.model.flat_to_rowcol(self.selection.active);
        if row >= self.model.line_count() - 1 {
            let count = self.model.grapheme_count();
            self.selection.active = count;
            if !extend {
                self.selection.anchor = count;
            }
            return false;
        }
        let target_col = sticky_col.unwrap_or(col);
        let next_line_len = self.model.line_grapheme_count(row + 1);
        let new_col = target_col.min(next_line_len);
        let flat = self.model.rowcol_to_flat(row + 1, new_col);
        self.selection.active = flat;
        if !extend {
            self.selection.anchor = flat;
        }
        true
    }

    pub fn move_to(&mut self, pos: usize, extend: bool) {
        let count = self.model.grapheme_count();
        self.selection.active = pos.min(count);
        if !extend {
            self.selection.anchor = self.selection.active;
        }
        self.reset_blink();
    }

    pub fn select_all(&mut self) {
        self.sticky_x = None;
        self.selection.anchor = 0;
        self.selection.active = self.model.grapheme_count();
        self.reset_blink();
    }

    pub fn word_at(&self, grapheme_idx: usize) -> (usize, usize) {
        self.model.word_at(grapheme_idx)
    }

    pub fn set_value(&mut self, value: String) {
        self.model.set_value(value);
        let count = self.model.grapheme_count();
        if self.selection.active > count {
            self.selection.active = count;
        }
        if self.selection.anchor > count {
            self.selection.anchor = count;
        }
    }

    /// Current (row, col) of the active cursor position.
    pub fn cursor_rowcol(&self) -> (usize, usize) {
        self.model.flat_to_rowcol(self.selection.active)
    }

    // ── Widget-layer concerns ────────────────────────────────────────

    pub fn grapheme_count(&self) -> usize {
        self.model.grapheme_count()
    }

    pub fn reset_blink(&mut self) {
        self.blink_reset = Instant::now();
    }

    pub fn blink_visible(&self, window_focused: bool) -> bool {
        if !self.focused || !window_focused {
            return false;
        }
        let elapsed = self.blink_reset.elapsed().as_millis();
        (elapsed % 1060) < 530
    }

    pub fn display_text(&self) -> String {
        if self.secure {
            "\u{2022}".repeat(self.model.grapheme_count())
        } else {
            self.model.text()
        }
    }

    pub fn update_scroll(&mut self, cursor_x: f32, visible_width: f32) {
        if visible_width <= 0.0 {
            return;
        }
        if cursor_x - self.scroll_offset < 0.0 {
            self.scroll_offset = cursor_x;
        } else if cursor_x - self.scroll_offset > visible_width {
            self.scroll_offset = cursor_x - visible_width;
        }
        if self.scroll_offset < 0.0 {
            self.scroll_offset = 0.0;
        }
    }

    pub fn update_scroll_y(&mut self, cursor_y: f32, line_height: f32, visible_height: f32) {
        if visible_height <= 0.0 {
            return;
        }
        let cursor_bottom = cursor_y + line_height;
        if cursor_y < self.scroll_offset_y {
            self.scroll_offset_y = cursor_y;
        } else if cursor_bottom > self.scroll_offset_y + visible_height {
            self.scroll_offset_y = cursor_bottom - visible_height;
        }
        if self.scroll_offset_y < 0.0 {
            self.scroll_offset_y = 0.0;
        }
    }

    // ── Key handling ─────────────────────────────────────────────────

    pub fn handle_key(&mut self, key: &Key, modifiers: u32) -> KeyResult {
        if self.disabled {
            return KeyResult::Ignored;
        }
        // Single-line: reject Enter
        if !self.multiline {
            if matches!(key, Key::Named(NamedKey::Enter)) {
                return KeyResult::Ignored;
            }
        }
        self.sticky_x = None;
        self.sticky_col = None;

        let shift = modifiers & 4 != 0;
        let ctrl = modifiers & 1 != 0;

        match key {
            Key::Character(ch) => {
                if ctrl {
                    if ch.eq_ignore_ascii_case("a") {
                        self.select_all();
                        return KeyResult::Handled;
                    }
                    return KeyResult::Ignored;
                }
                match self.insert_text(ch) {
                    Some(edit) => KeyResult::Edit(edit),
                    None => KeyResult::Handled,
                }
            }
            Key::Named(named) => match named {
                NamedKey::Backspace => {
                    if ctrl {
                        match self.delete_word_backward() {
                            Some(edit) => KeyResult::Edit(edit),
                            None => KeyResult::Handled,
                        }
                    } else {
                        match self.delete_backward() {
                            Some(edit) => KeyResult::Edit(edit),
                            None => KeyResult::Handled,
                        }
                    }
                }
                NamedKey::Delete => {
                    if ctrl {
                        match self.delete_word_forward() {
                            Some(edit) => KeyResult::Edit(edit),
                            None => KeyResult::Handled,
                        }
                    } else {
                        match self.delete_forward() {
                            Some(edit) => KeyResult::Edit(edit),
                            None => KeyResult::Handled,
                        }
                    }
                }
                NamedKey::ArrowLeft => {
                    if ctrl {
                        self.move_word_left(shift);
                    } else {
                        self.move_left(shift);
                    }
                    KeyResult::Handled
                }
                NamedKey::ArrowRight => {
                    if ctrl {
                        self.move_word_right(shift);
                    } else {
                        self.move_right(shift);
                    }
                    KeyResult::Handled
                }
                NamedKey::ArrowUp | NamedKey::ArrowDown => KeyResult::Ignored,
                NamedKey::Home => {
                    if ctrl {
                        self.move_absolute_home(shift);
                    } else {
                        self.move_home(shift);
                    }
                    KeyResult::Handled
                }
                NamedKey::End => {
                    if ctrl {
                        self.move_absolute_end(shift);
                    } else {
                        self.move_end(shift);
                    }
                    KeyResult::Handled
                }
                NamedKey::Space => match self.insert_text(" ") {
                    Some(edit) => KeyResult::Edit(edit),
                    None => KeyResult::Handled,
                },
                NamedKey::Escape => KeyResult::Blur,
                NamedKey::Enter => match self.insert_text("\n") {
                    Some(edit) => KeyResult::Edit(edit),
                    None => KeyResult::Ignored,
                },
                NamedKey::Tab => match self.insert_text("    ") {
                    Some(edit) => KeyResult::Edit(edit),
                    None => KeyResult::Ignored,
                },
                _ => KeyResult::Ignored,
            },
            _ => KeyResult::Ignored,
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn input(text: &str) -> InputState {
        let mut is = InputState::new();
        is.set_value(text.to_string());
        is
    }

    fn input_at(text: &str, cursor: usize) -> InputState {
        let mut is = input(text);
        is.selection.set_cursor(cursor);
        is
    }

    fn input_sel(text: &str, anchor: usize, active: usize) -> InputState {
        let mut is = input(text);
        is.selection = TextSelection::new(anchor, active);
        is
    }

    // ── Insert ───────────────────────────────────────────────────────

    #[test]
    fn insert_text_basic() {
        let mut is = InputState::new();
        is.insert_text("hello");
        assert_eq!(is.model.text(), "hello");
        assert_eq!(is.selection.active, 5);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn insert_text_at_cursor() {
        let mut is = input_at("hllo", 1);
        is.insert_text("e");
        assert_eq!(is.model.text(), "hello");
        assert_eq!(is.selection.active, 2);
    }

    #[test]
    fn insert_replaces_selection() {
        let mut is = input_sel("hello world", 0, 5);
        is.insert_text("goodbye");
        assert_eq!(is.model.text(), "goodbye world");
        assert_eq!(is.selection.active, 7);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn insert_newline_in_single_line_mode() {
        let mut is = input_at("hello", 5);
        // multiline is false by default
        let result = is.insert_text("\n");
        assert!(result.is_none());
        assert_eq!(is.model.text(), "hello");
    }

    #[test]
    fn insert_newline_in_multiline_mode() {
        let mut is = input_at("hello", 5);
        is.multiline = true;
        let result = is.insert_text("\n");
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hello\n");
    }

    #[test]
    fn insert_disabled_does_nothing() {
        let mut is = input_at("hello", 5);
        is.disabled = true;
        assert!(is.insert_text("!").is_none());
        assert_eq!(is.model.text(), "hello");
    }

    // ── Delete backward ──────────────────────────────────────────────

    #[test]
    fn delete_backward_at_end() {
        let mut is = input_at("hello", 5);
        let result = is.delete_backward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hell");
        assert_eq!(is.selection.active, 4);
    }

    #[test]
    fn delete_backward_at_start() {
        let mut is = input_at("hello", 0);
        assert!(is.delete_backward().is_none());
        assert_eq!(is.model.text(), "hello");
    }

    #[test]
    fn delete_backward_with_selection() {
        let mut is = input_sel("hello world", 5, 11);
        let result = is.delete_backward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hello");
        assert_eq!(is.selection.active, 5);
    }

    #[test]
    fn delete_backward_joins_lines() {
        let mut is = input_at("hello\nworld", 6);
        is.multiline = true;
        let result = is.delete_backward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "helloworld");
        assert_eq!(is.selection.active, 5);
    }

    // ── Delete forward ───────────────────────────────────────────────

    #[test]
    fn delete_forward_at_start() {
        let mut is = input_at("hello", 0);
        let result = is.delete_forward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "ello");
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn delete_forward_at_end() {
        let mut is = input_at("hello", 5);
        assert!(is.delete_forward().is_none());
    }

    #[test]
    fn delete_forward_with_selection() {
        let mut is = input_sel("hello world", 0, 6);
        let result = is.delete_forward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "world");
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn delete_forward_joins_lines() {
        let mut is = input_at("hello\nworld", 5);
        is.multiline = true;
        let result = is.delete_forward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "helloworld");
    }

    // ── Delete word backward ─────────────────────────────────────────

    #[test]
    fn delete_word_backward_basic() {
        let mut is = input_at("hello world", 11);
        let result = is.delete_word_backward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hello ");
        assert_eq!(is.selection.active, 6);
    }

    #[test]
    fn delete_word_backward_at_start() {
        let mut is = input_at("hello", 0);
        assert!(is.delete_word_backward().is_none());
    }

    #[test]
    fn delete_word_backward_with_selection() {
        let mut is = input_sel("hello world", 6, 11);
        let result = is.delete_word_backward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hello ");
        assert_eq!(is.selection.active, 6);
    }

    // ── Delete word forward ──────────────────────────────────────────

    #[test]
    fn delete_word_forward_basic() {
        let mut is = input_at("hello world", 0);
        let result = is.delete_word_forward();
        assert!(result.is_some());
        assert_eq!(is.model.text(), "world");
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn delete_word_forward_at_end() {
        let mut is = input_at("hello", 5);
        assert!(is.delete_word_forward().is_none());
    }

    // ── Move left/right ──────────────────────────────────────────────

    #[test]
    fn move_left_basic() {
        let mut is = input_at("hello", 3);
        is.move_left(false);
        assert_eq!(is.selection.active, 2);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn move_left_collapses_selection() {
        let mut is = input_sel("hello", 1, 4);
        is.move_left(false);
        assert_eq!(is.selection.active, 1);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn move_left_extend() {
        let mut is = input_at("hello", 3);
        is.move_left(true);
        assert_eq!(is.selection.active, 2);
        assert_eq!(is.selection.anchor, 3);
    }

    #[test]
    fn move_left_at_start() {
        let mut is = input_at("hello", 0);
        is.move_left(false);
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn move_right_basic() {
        let mut is = input_at("hello", 3);
        is.move_right(false);
        assert_eq!(is.selection.active, 4);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn move_right_collapses_selection() {
        let mut is = input_sel("hello", 1, 4);
        is.move_right(false);
        assert_eq!(is.selection.active, 4);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn move_right_extend() {
        let mut is = input_at("hello", 3);
        is.move_right(true);
        assert_eq!(is.selection.active, 4);
        assert_eq!(is.selection.anchor, 3);
    }

    #[test]
    fn move_right_at_end() {
        let mut is = input_at("hello", 5);
        is.move_right(false);
        assert_eq!(is.selection.active, 5);
    }

    #[test]
    fn move_right_across_newline() {
        let mut is = input_at("ab\ncd", 2);
        is.multiline = true;
        is.move_right(false);
        assert_eq!(is.selection.active, 3);
        assert_eq!(is.model.flat_to_rowcol(3), (1, 0));
    }

    #[test]
    fn move_left_across_newline() {
        let mut is = input_at("ab\ncd", 3);
        is.multiline = true;
        is.move_left(false);
        assert_eq!(is.selection.active, 2);
        assert_eq!(is.model.flat_to_rowcol(2), (0, 2));
    }

    // ── Move word left/right ─────────────────────────────────────────

    #[test]
    fn move_word_left() {
        let mut is = input_at("hello world foo", 15);
        is.move_word_left(false);
        assert_eq!(is.selection.active, 12);
        is.move_word_left(false);
        assert_eq!(is.selection.active, 6);
        is.move_word_left(false);
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn move_word_right() {
        let mut is = input_at("hello world foo", 0);
        is.move_word_right(false);
        assert_eq!(is.selection.active, 6);
        is.move_word_right(false);
        assert_eq!(is.selection.active, 12);
        is.move_word_right(false);
        assert_eq!(is.selection.active, 15);
    }

    #[test]
    fn move_word_left_with_extend() {
        let mut is = input_at("hello world", 11);
        is.move_word_left(true);
        assert_eq!(is.selection.active, 6);
        assert_eq!(is.selection.anchor, 11);
    }

    // ── Move home/end ────────────────────────────────────────────────

    #[test]
    fn move_home_single_line() {
        let mut is = input_at("hello", 3);
        is.move_home(false);
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn move_end_single_line() {
        let mut is = input_at("hello", 2);
        is.move_end(false);
        assert_eq!(is.selection.active, 5);
    }

    #[test]
    fn move_home_on_second_line() {
        let mut is = input_at("hello\nworld", 8);
        is.multiline = true;
        is.move_home(false);
        assert_eq!(is.selection.active, 6);
    }

    #[test]
    fn move_end_on_first_line() {
        let mut is = input_at("hello\nworld", 2);
        is.multiline = true;
        is.move_end(false);
        assert_eq!(is.selection.active, 5);
    }

    #[test]
    fn move_home_end_each_line() {
        let mut is = input_at("abc\ndef\nghi", 1);
        is.multiline = true;

        is.move_home(false);
        assert_eq!(is.selection.active, 0);
        is.move_end(false);
        assert_eq!(is.selection.active, 3);

        is.move_to(5, false);
        is.move_home(false);
        assert_eq!(is.selection.active, 4);
        is.move_end(false);
        assert_eq!(is.selection.active, 7);

        is.move_to(9, false);
        is.move_home(false);
        assert_eq!(is.selection.active, 8);
        is.move_end(false);
        assert_eq!(is.selection.active, 11);
    }

    // ── Move absolute home/end ───────────────────────────────────────

    #[test]
    fn move_absolute_home() {
        let mut is = input_at("hello\nworld", 8);
        is.move_absolute_home(false);
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn move_absolute_end() {
        let mut is = input_at("hello\nworld", 2);
        is.move_absolute_end(false);
        assert_eq!(is.selection.active, 11);
    }

    // ── Move up/down ─────────────────────────────────────────────────

    #[test]
    fn move_up_basic() {
        let mut is = input_at("hello\nworld", 8);
        is.multiline = true;
        let moved = is.move_up(false, None);
        assert!(moved);
        assert_eq!(is.selection.active, 2);
    }

    #[test]
    fn move_down_basic() {
        let mut is = input_at("hello\nworld", 2);
        is.multiline = true;
        let moved = is.move_down(false, None);
        assert!(moved);
        assert_eq!(is.selection.active, 8);
    }

    #[test]
    fn move_up_from_first_line() {
        let mut is = input_at("hello\nworld", 3);
        let moved = is.move_up(false, None);
        assert!(!moved);
        assert_eq!(is.selection.active, 0);
    }

    #[test]
    fn move_down_from_last_line() {
        let mut is = input_at("hello\nworld", 8);
        let moved = is.move_down(false, None);
        assert!(!moved);
        assert_eq!(is.selection.active, 11);
    }

    #[test]
    fn move_up_down_preserves_sticky_col() {
        let mut is = input_at("abcdef\nab\nabcdef", 5);
        is.multiline = true;

        let (_, col) = is.cursor_rowcol();
        assert_eq!(col, 5);

        is.move_down(false, Some(5));
        assert_eq!(is.cursor_rowcol(), (1, 2));

        is.move_down(false, Some(5));
        assert_eq!(is.cursor_rowcol(), (2, 5));
    }

    #[test]
    fn move_up_with_extend() {
        let mut is = input_at("hello\nworld", 8);
        is.multiline = true;
        is.move_up(true, None);
        assert_eq!(is.selection.active, 2);
        assert_eq!(is.selection.anchor, 8);
    }

    // ── Select all ───────────────────────────────────────────────────

    #[test]
    fn select_all_basic() {
        let mut is = input("hello");
        is.select_all();
        assert_eq!(is.selection.anchor, 0);
        assert_eq!(is.selection.active, 5);
    }

    #[test]
    fn select_all_multiline() {
        let mut is = input("hello\nworld");
        is.select_all();
        assert_eq!(is.selection.anchor, 0);
        assert_eq!(is.selection.active, 11);
        assert_eq!(is.selected_text(), "hello\nworld");
    }

    // ── Selected text ────────────────────────────────────────────────

    #[test]
    fn selected_text_empty_when_collapsed() {
        let is = input_at("hello", 3);
        assert_eq!(is.selected_text(), "");
    }

    #[test]
    fn selected_text_within_line() {
        let is = input_sel("hello world", 0, 5);
        assert_eq!(is.selected_text(), "hello");
    }

    #[test]
    fn selected_text_across_lines() {
        let is = input_sel("abc\ndef\nghi", 2, 6);
        assert_eq!(is.selected_text(), "c\nde");
    }

    #[test]
    fn selected_text_reversed_selection() {
        let is = input_sel("hello world", 5, 0);
        assert_eq!(is.selected_text(), "hello");
    }

    // ── move_to ──────────────────────────────────────────────────────

    #[test]
    fn move_to_basic() {
        let mut is = input("hello");
        is.move_to(3, false);
        assert_eq!(is.selection.active, 3);
        assert!(is.selection.is_collapsed());
    }

    #[test]
    fn move_to_with_extend() {
        let mut is = input_at("hello", 1);
        is.move_to(4, true);
        assert_eq!(is.selection.active, 4);
        assert_eq!(is.selection.anchor, 1);
    }

    #[test]
    fn move_to_clamps() {
        let mut is = input("hello");
        is.move_to(100, false);
        assert_eq!(is.selection.active, 5);
    }

    // ── word_at ──────────────────────────────────────────────────────

    #[test]
    fn word_at_basic() {
        let is = input("hello world");
        assert_eq!(is.word_at(0), (0, 5));
        assert_eq!(is.word_at(7), (6, 11));
    }

    // ── set_value ────────────────────────────────────────────────────

    #[test]
    fn set_value_clamps_selection() {
        let mut is = input_at("hello world", 11);
        is.set_value("hi".to_string());
        assert_eq!(is.selection.active, 2);
        assert_eq!(is.selection.anchor, 2);
    }

    // ── display_text ─────────────────────────────────────────────────

    #[test]
    fn display_text_normal() {
        let is = input("hello");
        assert_eq!(is.display_text(), "hello");
    }

    #[test]
    fn display_text_secure() {
        let mut is = input("hello");
        is.secure = true;
        assert_eq!(is.display_text(), "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}");
    }

    // ── cursor_rowcol ────────────────────────────────────────────────

    #[test]
    fn cursor_rowcol_basic() {
        let is = input_at("hello\nworld", 8);
        assert_eq!(is.cursor_rowcol(), (1, 2));
    }

    // ── Disabled guard ───────────────────────────────────────────────

    #[test]
    fn disabled_blocks_all_edits() {
        let mut is = input_at("hello", 5);
        is.disabled = true;
        assert!(is.insert_text("!").is_none());
        assert!(is.delete_backward().is_none());
        assert!(is.delete_forward().is_none());
        assert!(is.delete_word_backward().is_none());
        assert!(is.delete_word_forward().is_none());
        assert_eq!(is.model.text(), "hello");
    }

    // ── Delete selection spanning lines ──────────────────────────────

    #[test]
    fn delete_selection_multiline() {
        let mut is = input_sel("abc\ndef\nghi", 2, 9);
        is.multiline = true;
        is.delete_backward();
        assert_eq!(is.model.text(), "abhi");
        assert_eq!(is.selection.active, 2);
    }

    // ── Integration: type, select, delete, type ──────────────────────

    #[test]
    fn integration_type_select_delete_type() {
        let mut is = InputState::new();
        is.insert_text("hello world");
        assert_eq!(is.model.text(), "hello world");

        // Select "world"
        is.selection = TextSelection::new(6, 11);
        is.insert_text("rust");
        assert_eq!(is.model.text(), "hello rust");

        // Move to start, delete forward
        is.move_to(0, false);
        is.delete_forward();
        assert_eq!(is.model.text(), "ello rust");
    }

    #[test]
    fn integration_multiline_editing() {
        let mut is = InputState::new();
        is.multiline = true;
        is.insert_text("line1\nline2\nline3");
        assert_eq!(is.model.text(), "line1\nline2\nline3");
        assert_eq!(is.model.line_count(), 3);

        // Cursor should be at end
        assert_eq!(is.selection.active, is.model.grapheme_count());

        // Move up
        let (_, col) = is.cursor_rowcol();
        is.move_up(false, Some(col));
        assert_eq!(is.cursor_rowcol(), (1, 5));

        // Delete backward (delete "2")
        is.delete_backward();
        assert_eq!(is.model.text(), "line1\nline\nline3");
    }

    // ── Max length ───────────────────────────────────────────────────

    #[test]
    fn max_length_blocks_insert() {
        let mut is = InputState::new();
        is.model.max_length = Some(5);
        is.insert_text("hello");
        assert_eq!(is.model.text(), "hello");
        assert!(is.insert_text("!").is_none());
        assert_eq!(is.model.text(), "hello");
    }

    #[test]
    fn max_length_allows_replace_within_limit() {
        let mut is = InputState::new();
        is.model.max_length = Some(5);
        is.insert_text("hello");
        // Select all and replace with shorter text
        is.select_all();
        let result = is.insert_text("hi");
        assert!(result.is_some());
        assert_eq!(is.model.text(), "hi");
    }
}
