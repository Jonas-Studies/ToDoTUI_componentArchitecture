use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout}};

use crate::tasks::Tasks;

use super::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanContainValue, CanHandleUserinput}, types_of_content::{container::Container, textinput::Textinput, TypesOfContent}, Content};

pub struct Application<'applications_lifetime> {
    content: Container<'applications_lifetime>
}

impl Application<'_> {
    pub fn new(tasks: &Tasks) -> Self {
        let layout = Layout::vertical(
            vec![ Constraint::Length(3); tasks.len() ]
        );

        let mut content = Container::new(layout);

        for task in tasks.iter() {
            content.push_content(
                Content::new(
                    TypesOfContent::Textinput(Textinput::new(task.get_name(), String::new()))
                ).as_can_be_focused()
            );
        }

        content.focus_first();

        Self { content }
    }
}

impl CanBeFocused for Application<'_> {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        self.content.render_focused(area, buffer);
    }
}

impl CanHandleUserinput for Application<'_> {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        match userinput {
            KeyCode::Esc => {
                Some(PossibleActions::Cancel)
            }
            KeyCode::Enter => {
                Some(PossibleActions::Select)
            }
            _ => {
                self.content.handle_userinpt(userinput);
                None
            }
        }
    }
}

impl CanContainValue<usize> for Application<'_> {
    fn get_value(&self) -> usize {
        self.content.get_nr_of_focused_content()
    }
}
