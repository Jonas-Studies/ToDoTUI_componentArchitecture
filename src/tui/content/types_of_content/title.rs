use ratatui::{style::{Modifier, Stylize}, widgets::{Paragraph, Widget}};

use crate::tui::content::traits::CanBeRendered;

pub struct Title {
    value: String
}

impl Title {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl CanBeRendered for Title {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_value()).add_modifier(Modifier::BOLD).render(area, buffer);
    }
}
