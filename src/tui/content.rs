use super::traits::{IsContent, CanBeFocused, MayDisplayCursor, CanHandleUserinput};

use ratatui::{
    crossterm::event::KeyCode, prelude::{Buffer, Position, Rect}, style::{Color, Style}, widgets::{Block, Paragraph, WidgetRef}
};

impl IsContent for Paragraph <'_> {}

impl CanBeFocused for Paragraph <'_> {
    fn focus(& mut self) {}
    fn unfocus(& mut self) {}
}

impl MayDisplayCursor for Paragraph <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        None
    }
}

impl CanHandleUserinput for Paragraph <'_> {
    fn handle_userinput(& mut self, _userinput: & KeyCode) {}
}

pub struct TextField <'textfields_lifetime> {
    text: String,
    borders: Block <'textfields_lifetime>,
    cursor_offset: usize
}

impl TextField <'_> {
    pub fn new(text: String, title: String) -> Self {
        let borders = Block::bordered().title(title);

        Self { text, borders, cursor_offset: 0 }
    }
    fn get_index_of_last_character(& self) -> usize {
        self.text.len() - 1
    }
    fn get_index_of_first_character(& self) -> usize {
        0
    }
    fn can_cursor_move_right(& self) -> bool {
        self.cursor_offset <= self.get_index_of_last_character()
    }
    fn can_cursor_move_left(& self) -> bool {
        self.cursor_offset > self.get_index_of_first_character()
    }
    fn can_delete_current_character(& self) -> bool {
        self.cursor_offset <= self.get_index_of_last_character()
    }
    fn can_delete_previous_character(& self) -> bool {
        self.cursor_offset > self.get_index_of_first_character()
    }
    fn move_cursor_right(& mut self) {
        self.cursor_offset += 1;
    }
    fn move_cursor_left(& mut self) {
        self.cursor_offset -= 1;
    }
    fn move_cursor_end(& mut self) {
        self.cursor_offset = self.get_index_of_last_character() + 1;
    }
    fn move_cursor_start(& mut self) {
        self.cursor_offset = self.get_index_of_first_character();
    }
    fn delete_current_character(& mut self) {
        self.text.remove(self.cursor_offset);
    }
    fn delete_previous_character(& mut self) {
        self.text.remove(self.cursor_offset - 1);
        self.move_cursor_left();
    }
}

impl IsContent for TextField <'_> {}

impl WidgetRef for TextField <'_> {
    fn render_ref(& self, area: Rect, buffer: & mut Buffer) {
        Paragraph::new(self.text.clone()).block(self.borders.clone()).render_ref(area, buffer);
    }
}

impl CanBeFocused for TextField <'_> {
    fn focus(& mut self) {
        self.borders = self.borders.clone().border_style(Style::new().fg(Color::Yellow));
    }
    fn unfocus(& mut self) {
        self.borders = self.borders.clone().style(Style::default());
    }
}

impl MayDisplayCursor for TextField <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        // Minimum position of 1, 1 because the Textfield is getting rendered with borders
        Some(Position::new(1 + self.cursor_offset as u16, 1))
    }
}

impl CanHandleUserinput for TextField <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Right => {
                if self.can_cursor_move_right() {
                    self.move_cursor_right();
                }
            }
            KeyCode::Left => {
                if self.can_cursor_move_left() {
                    self.move_cursor_left()
                }
            }
            KeyCode::End => {
                self.move_cursor_end();
            }
            KeyCode::Home => {
                self.move_cursor_start();
            }
            KeyCode::Char(character) => {
                self.text.insert(self.cursor_offset, *character);
                self.move_cursor_right();
            }
            KeyCode::Delete => {
                if self.can_delete_current_character() {
                    self.delete_current_character();
                }
            }
            KeyCode::Backspace => {
                if self.can_delete_previous_character() {
                    self.delete_previous_character();
                }
            }
            _ => {}
        }
    }
}
