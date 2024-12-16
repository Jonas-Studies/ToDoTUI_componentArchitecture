use super::traits::{CanBeFocused, CanHandleUserinput, IsContent, MayDisplayCursor};

use ratatui::{crossterm::event::KeyCode, prelude::{Buffer, Position, Rect}};

pub struct Field <Content> where Content: IsContent {
    area: Rect,
    content: Content
}

impl <Content> Field <Content> where Content: IsContent + Clone {
    pub fn new (area: Rect, content: Content) -> Self {
        Self { area, content }
    }

    pub fn reference_content(& self) -> & Content {
        & self.content
    }

    pub fn focus(& mut self) {
        self.content = self.content.clone().focused();
    }

    pub fn unfocus(& mut self) {
        self.content = self.content.clone().unfocused();
    }

    pub fn render (& self, buffer: & mut Buffer) {
        self.content.render_ref(self.area, buffer);
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
