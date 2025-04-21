use ratatui::widgets::{Block, Paragraph, Widget};

use crate::tui::content::traits::CanBeRendered;

pub struct Button<PossibleActions> {
    label: String,
    on_click: PossibleActions
}

impl <PossibleActions> Button<PossibleActions> {
    pub fn new(label: String, on_click: PossibleActions) -> Self {
        Self {label, on_click }
    }
    fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl <PossibleActions> CanBeRendered for Button<PossibleActions> {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_label()).block(Block::bordered()).centered().render(area, buffer);
    }
}
