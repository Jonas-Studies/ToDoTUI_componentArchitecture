use super::traits::{CanBeFocused, CanHandleUserinput, IsContent, MayDisplayCursor};

use ratatui::{crossterm::event::KeyCode, prelude::{Buffer, Position, Rect}};

pub struct Field <Content> where Content: IsContent {
    area: Rect,
    content: Content
}

impl <Content> Field <Content> where Content: IsContent {
    pub fn new (area: Rect, content: Content) -> Self {
        Self { area, content }
    }

    pub fn render (& self, buffer: & mut Buffer) {
        self.content.render_ref(self.area, buffer);
    }
}

impl <Content> CanBeFocused for Field <Content> where Content: IsContent {
    fn focus(& mut self) {
        self.content.focus();
    }
    fn unfocus(& mut self) {
        self.content.unfocus();
    }
}

impl <Content> MayDisplayCursor for Field <Content> where Content: IsContent {
    fn get_cursor_position(& self) -> Option<Position> {
        if let Some(mut position) = self.content.get_cursor_position() {
            position.x += self.area.x;
            position.y += self.area.y;

            Some(position)
        }
        else {
            None
        }
    }
}

impl <Content> CanHandleUserinput for Field <Content> where Content: IsContent {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        self.content.handle_userinput(userinput);
    }
}
