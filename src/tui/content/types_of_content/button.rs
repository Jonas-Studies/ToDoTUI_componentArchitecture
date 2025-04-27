use ratatui::{crossterm::event::KeyCode, style::{Style, Stylize}, widgets::{Block, Paragraph, Widget}};

use crate::tui::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanBeRendered, CanHandleUserinput}};

pub struct Button {
    label: String,
    action_on_click: PossibleActions
}

impl Button {
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

impl CanBeRendered for Button {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_label()).block(Block::bordered()).centered().render(area, buffer);
    }
}

impl  CanBeFocused for Button {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.get_label()).block(
            Block::bordered().border_style(Style::new().yellow())
        ).centered().render(area, buffer);
    }
}

impl  CanHandleUserinput for Button {
    fn handle_userinpt(&mut self, userinput: ratatui::crossterm::event::KeyCode) -> Option<PossibleActions> {
        match userinput {
            KeyCode::Enter => { Some(self.get_action_on_click()) }
            _ => { None }
        }
    }
}
