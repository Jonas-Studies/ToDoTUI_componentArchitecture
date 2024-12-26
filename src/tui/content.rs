use super::traits::{IsContent, CanBeFocused, MayDisplayCursor, CanHandleUserinput};

use ratatui::{
    crossterm::event::KeyCode, prelude::{Buffer, Position, Rect}, style::{Color, Style}, widgets::{Block, Paragraph, WidgetRef}
};

impl IsContent for Paragraph <'_> {}

impl CanBeFocused for Paragraph <'_> {
    fn focused(self) -> Self {
        self.style(
            Style::default().fg( Color::Yellow.into() )
        )
    }
    fn unfocused(self) -> Self {
        self.style(
            Style::reset()
        )
    }
}

impl MayDisplayCursor for Paragraph <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        None
    }
}

impl CanHandleUserinput for Paragraph <'_> {
    fn handle_userinput(& mut self, _userinput: & KeyCode) {}
}

impl CanBeFocused for Block <'_> {
    fn focused(self) -> Self {
        self.border_style( Style::new().fg(Color::Yellow) )
    }
    fn unfocused(self) -> Self {
        self.border_style( Style::reset() )
    }
}

#[derive(Clone)]
pub struct TextField <'textfields_lifetime> {
    // The text is getting stored as char-vector as using strings with utf8 does not work here
    value: Vec<char>,
    borders: Block <'textfields_lifetime>,
    cursor_offset: usize
}

impl TextField <'_> {
    pub fn new(text: String, title: String) -> Self {
        let borders = Block::bordered().title(title);
        let value = Vec::from_iter(text.chars().into_iter());

        Self { value, borders, cursor_offset: 0 }
    }
    pub fn get_value (& self) -> String {
        String::from_iter(self.value.iter().clone())
    }
    fn get_index_of_last_character(& self) -> usize {
        self.value.len() - 1
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
        self.value.remove(self.cursor_offset);
    }
    fn delete_previous_character(& mut self) {
        self.value.remove(self.cursor_offset - 1);
        self.move_cursor_left();
    }
}

impl IsContent for TextField <'_> {}

impl WidgetRef for TextField <'_> {
    fn render_ref(& self, area: Rect, buffer: & mut Buffer) {
        Paragraph::new(String::from_iter(self.value.iter().clone())).block(self.borders.clone()).render_ref(area, buffer);
    }
}

impl CanBeFocused for TextField <'_> {
    fn focused(mut self) -> Self {
        self.borders = self.borders.clone().focused();
        self
    }
    fn unfocused(mut self) -> Self {
        self.borders = self.borders.clone().focused();
        self
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
                self.value.insert(self.cursor_offset, *character);
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

#[derive(Clone)]
pub struct Button <'buttons_lifetime> {
    text: Paragraph <'buttons_lifetime>,
    borders: Block <'buttons_lifetime>
}

impl Button <'_> {
    pub fn new (text: String) -> Self {
        let text = Paragraph::new(text).centered();
        let borders = Block::bordered();

        Self { text, borders }
    }
}

impl CanBeFocused for Button <'_> {
    fn focused(mut self) -> Self {
        self.borders = self.borders.clone().focused();
        self
    }
    fn unfocused(mut self) -> Self {
        self.borders = self.borders.clone().focused();
        self
    }
}

impl MayDisplayCursor for Button <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        None
    }
}

impl CanHandleUserinput for Button <'_> {
    fn handle_userinput (& mut self, _: & KeyCode) {}
}

impl WidgetRef for Button <'_> {
    fn render_ref(& self, area: Rect, buffer: & mut Buffer) {
        self.text.clone().block(self.borders.clone()).render_ref(area, buffer);
    }
}

impl IsContent for Button <'_> {}
