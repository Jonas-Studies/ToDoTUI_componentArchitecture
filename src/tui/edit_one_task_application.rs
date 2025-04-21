use ratatui::layout::{Constraint, Layout};

use crate::task::Task;

use super::content::{traits::CanBeRendered, types_of_content::{title::Title, TypesOfContent}, Content};

pub struct Application {
    layout: Layout,
    task: Task,
    content: Vec<Content>
}

impl Application {
    pub fn new(task_to_edit: Task) -> Self {
        let layout = Layout::vertical( [ Constraint::Length(1) ] );

        let mut content = Vec::new();

        content.push(
            Content::new(TypesOfContent::Title(Self::get_title(task_to_edit.is_finished())))
        );

        Self { layout, task: task_to_edit, content }
    }
    fn get_title(is_task_finished: bool) -> Title {
        Title::new(
            String::from(
                if is_task_finished {
                    "Done:"
                }
                else {
                    "ToDo:"
                }
            )
        )
    }
}

impl CanBeRendered for Application {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }
    }
}
