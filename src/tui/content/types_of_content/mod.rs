use core::clone::Clone;

use button::Button;
use container::Container;
use ratatui::prelude::{Rect, Buffer};
use textinput::Textinput;
use title::Title;
use checkbox::Checkbox;

use super::{possible_actions::PossibleActions, traits::{CanBeFocused, CanBeRendered, CanHandleUserinput, MayDisplayCursor}};

pub mod title;
pub mod textinput;
pub mod button;
pub mod container;
pub mod checkbox;

pub enum TypesOfContent<'content_types_lifetime> {
    Title(Title),
    Textinput(Textinput),
    Checkbox(Checkbox),
    Button(Button),
    Contaier(Container<'content_types_lifetime>),
}

impl CanBeRendered for TypesOfContent<'_> {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            TypesOfContent::Title(content) => { content.render(area, buffer); }
            TypesOfContent::Textinput(content) => { content.render(area, buffer); }
            TypesOfContent::Button(content) => { content.render(area, buffer); }
            TypesOfContent::Contaier(content) => { content.render(area, buffer); }
            TypesOfContent::Checkbox(content) => { content.render(area, buffer); }
        }
    }
}

impl CanBeFocused for TypesOfContent<'_> {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            TypesOfContent::Textinput(content) => { content.render_focused(area, buffer); }
            TypesOfContent::Button(content) => { content.render_focused(area, buffer); }
            TypesOfContent::Contaier(content) => { content.render_focused(area, buffer); }
            TypesOfContent::Checkbox(content) => { content.render_focused(area, buffer); }
            _ => { self.render(area, buffer); }
        }
    }
}

impl CanHandleUserinput for TypesOfContent<'_> { 
    fn handle_userinpt(&mut self, userinput: ratatui::crossterm::event::KeyCode) -> Option<PossibleActions> {
        match self {
            TypesOfContent::Textinput(content) => { content.handle_userinpt(userinput) }
            TypesOfContent::Button(content) => { content.handle_userinpt(userinput) }
            TypesOfContent::Contaier(content) => { content.handle_userinpt(userinput) }
            _ => { None }
        }
    }
}

impl MayDisplayCursor for TypesOfContent<'_> {
    fn get_cursor_position(&self, area: Rect) -> Option<ratatui::prelude::Position> {
        match self {
            TypesOfContent::Textinput(content) => { content.get_cursor_position(area) }
            TypesOfContent::Contaier(content) => { content.get_cursor_position(area) }
            _ => { None }
        }
    }
}
