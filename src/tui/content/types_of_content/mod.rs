use ratatui::prelude::{Rect, Buffer};

use super::traits::{CanBeFocused, CanBeRendered};

pub enum TypesOfContent {
}

impl CanBeRendered for TypesOfContent {
    fn render (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            _ => { }
        }
    }
}

impl CanBeFocused for TypesOfContent {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer) {
        match self {
            _ => { }
        }
    }
}
