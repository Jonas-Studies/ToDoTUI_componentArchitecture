use ratatui::{crossterm::event::KeyCode, layout::Positions, prelude::{Buffer, Position, Rect}, style::{Style, Stylize}, widgets::{Block, Paragraph, Widget}};

use crate::tui::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanBeRendered, CanContainValue, CanHandleUserinput, MayDisplayCursor}};

pub struct Textinput {
    // Using a vector of chars instead of String because there would be problems with utf-8
    value: Vec<char>,
    title: String,
    cursor_offset: usize
}

impl Textinput {
    pub fn new(value: String, title: String) -> Self {
        let value = Vec::from_iter(value.chars());

        Self { value, title, cursor_offset: 0 }
    }
    fn get_title(&self) -> String {
        self.title.clone()
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

impl CanBeRendered for Textinput {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_value()).block(
            Block::bordered().title(self.get_title())
        ).render(area, buffer);
    }
}

impl CanBeFocused for Textinput {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_value()).block(
            Block::bordered().title(self.get_title()).border_style(Style::new().yellow())
        ).render(area, buffer);
    }
}

impl CanHandleUserinput for Textinput {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
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
                self.value.insert(self.cursor_offset, character);
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
        None
    }
}

impl CanContainValue<String> for Textinput {
    fn get_value(&self) -> String {
        String::from_iter(self.value.iter())
    }
}

impl MayDisplayCursor for Textinput {
    fn get_cursor_position(&self, area: Rect) -> Option<Position> {
        Some(
            // Always adding 1 because of the borders
            Position::new(
                area.x + 1 + self.cursor_offset as u16,
                area.y + 1
            )
        )
    }
}
