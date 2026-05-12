use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout}, widgets::Paragraph};

use crate::{task::Task, tasks::Tasks};

use super::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanContainValue, CanHandleUserinput}, types_of_content::{container::Container, TypesOfContent}, Content};

pub struct Application<'applications_lifetime> {
    content: Container<'applications_lifetime>
}

impl <'applications_lifetime> Application<'applications_lifetime> {
    pub fn new(tasks: &'applications_lifetime Tasks) -> Self {
        let layout = Layout::vertical(
            vec![ Constraint::Length(1); tasks.len() ]
        ).spacing(1);

        let mut content = Container::new(layout);

        for task in tasks.iter() {
            content.push_content(
                Content::new(
                    TypesOfContent::Contaier(Self::get_tasklistitem(task))
                )
            );
        }

        content.focus_first();

        Self { content }
    }

    fn get_tasklistitem(task: &Task) -> Container {
        let mut result = Container::new(
            Layout::horizontal(
                [ Constraint::Length(5), Constraint::Length(50) ]
            ).spacing(1)
        );

        result.push_content(
            Content::new(
                TypesOfContent::Paragraph(
                    Paragraph::new(
                        String::from(
                            if task.is_finished() {
                                "Done:"
                            }
                            else {
                                "ToDo:"
                            }
                        )
                    )
                )
            )
        );
        result.push_content(
            Content::new(
                TypesOfContent::Paragraph(Paragraph::new(task.get_name()))
            )
        );

        return result;
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
