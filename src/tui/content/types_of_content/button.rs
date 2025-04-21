use ratatui::{crossterm::event::KeyCode, style::{Style, Stylize}, widgets::{Block, Paragraph, Widget}};

use crate::tui::content::traits::{CanBeFocused, CanBeRendered, CanHandleUserinput};

pub struct Button<PossibleActions> where PossibleActions: Clone {
    label: String,
    action_on_click: PossibleActions
}

impl <PossibleActions> Button<PossibleActions> where PossibleActions: Clone {
    pub fn new(label: String, action_on_click: PossibleActions) -> Self {
        Self {label, action_on_click }
    }
    fn get_label(&self) -> String {
        self.label.clone()
    }
    fn get_action_on_click(&self) -> PossibleActions {
        self.action_on_click.clone()
    }
}

impl <PossibleActions> CanBeRendered for Button<PossibleActions> where PossibleActions: Clone {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_label()).block(Block::bordered()).centered().render(area, buffer);
    }
}

impl <PossibleActions> CanBeFocused for Button<PossibleActions> where PossibleActions: Clone {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_label()).block(
            Block::bordered().border_style(Style::new().yellow())
        ).centered().render(area, buffer);
    }
}

impl <PossibleActions> CanHandleUserinput<PossibleActions> for Button<PossibleActions> where PossibleActions: Clone {
    fn handle_userinpt(&mut self, userinput: ratatui::crossterm::event::KeyCode) -> Option<PossibleActions> {
        match userinput {
            KeyCode::Enter => { Some(self.get_action_on_click()) }
            _ => { None }
        }
    }
}
