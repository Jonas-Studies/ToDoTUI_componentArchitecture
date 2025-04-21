use core::clone::Clone;

use button::Button;
use container::Container;
use ratatui::prelude::{Rect, Buffer};
use textinput::Textinput;
use title::Title;

use super::traits::{CanBeFocused, CanBeRendered, CanHandleUserinput, MayDisplayCursor};

pub mod title;
pub mod textinput;
pub mod button;
pub mod container;

pub enum TypesOfContent<PossibleActions> where PossibleActions: Clone {
    Title(Title),
    Textinput(Textinput),
    Button(Button<PossibleActions>),
    Contaier(Container<PossibleActions>)
}

impl<PossibleActions> CanBeRendered for TypesOfContent<PossibleActions> where PossibleActions: Clone {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            TypesOfContent::Title(content) => { content.render(area, buffer); }
            TypesOfContent::Textinput(content) => { content.render(area, buffer); }
            TypesOfContent::Button(content) => { content.render(area, buffer); }
            TypesOfContent::Contaier(content) => { content.render(area, buffer); }
        }
    }
}

impl<PossibleActions> CanBeFocused for TypesOfContent<PossibleActions> where PossibleActions: Clone {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            TypesOfContent::Textinput(content) => { content.render_focused(area, buffer); }
            TypesOfContent::Button(content) => { content.render_focused(area, buffer); }
            _ => { self.render(area, buffer); }
        }
    }
}

impl<PossibleActions> CanHandleUserinput<PossibleActions> for TypesOfContent<PossibleActions> where PossibleActions: Clone { 
    fn handle_userinpt(&mut self, userinput: ratatui::crossterm::event::KeyCode) -> Option<PossibleActions> {
        match self {
            TypesOfContent::Textinput(content) => { content.handle_userinpt(userinput) }
            TypesOfContent::Button(content) => { content.handle_userinpt(userinput) }
            _ => { None }
        }
    }
}

impl<PossibleActions> MayDisplayCursor for TypesOfContent<PossibleActions> where PossibleActions: Clone {
    fn get_cursor_position(&self, area: Rect) -> Option<ratatui::prelude::Position> {
        match self {
            TypesOfContent::Textinput(content) => { content.get_cursor_position(area) }
            _ => { None }
        }
    }
}
