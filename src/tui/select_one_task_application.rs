use ratatui::{layout::{Constraint, Layout, Rect}, style::Style, widgets::{WidgetRef, Paragraph}, Frame};

use crate::task::Task;

use super::field::Field;

pub struct Application <'applications_lifetime> {
    content: TaskList <'applications_lifetime>
}

impl Application <'_> {
    pub fn new (tasks_to_select_from: & Vec<Task>, area: Rect) -> Self {
        let content = TaskList::new(tasks_to_select_from, area);

        Self { content }
    }

    pub fn render (& self, frame: & mut Frame) {
        self.content.render(frame);
    }
}

struct TaskList <'lists_lifetime> {
    items: Vec<TaskListItem <'lists_lifetime>>,
    index_of_selected_item: usize
}

impl TaskList <'_> {
    fn new (tasks: & Vec<Task>, area: Rect) -> Self {
        let areas = Layout::vertical(
            vec![ Constraint::Length(2); tasks.len() ]
        ).split(area);

        let items: Vec<TaskListItem> = areas.iter().enumerate().map(
            |(index_of_area, area)| {
                TaskListItem::new(& tasks[index_of_area], * area)
            }
        ).collect();

        Self { items, index_of_selected_item: 0 }
    }

    fn render (& self, frame: & mut Frame) {
        for item in self.items.iter() {
            item.render(frame);
        }
    }
}

struct TaskListItem <'items_lifetime> {
    task: Field<Paragraph <'items_lifetime>>,
}

impl TaskListItem <'_> {
    fn new(task: & Task, area: Rect) -> Self {
        let text = if task.is_finished() {
            String::from("Done: ")
        }
        else {
            String::from("Todo: ")
        } + & task.get_name();

        let task = Field::new(area, Paragraph::new(text));

        Self { task }
    }
    fn render(& self, frame: & mut Frame) {
        self.task.render(frame.buffer_mut());
    }
}
