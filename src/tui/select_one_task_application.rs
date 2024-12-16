use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, widgets::Paragraph, Frame};

use crate::task::Task;

use super::{field::Field, traits::CanHandleUserinput};

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

impl CanHandleUserinput for Application <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        self.content.handle_userinput(userinput);
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

        let mut items: Vec<TaskListItem> = areas.iter().enumerate().map(
            |(index_of_area, area)| {
                TaskListItem::new(& tasks[index_of_area], * area)
            }
        ).collect();

        let index_of_selected_item = 0;
        items[index_of_selected_item].focus();

        Self { items, index_of_selected_item }
    }
    fn render (& self, frame: & mut Frame) {
        for item in self.items.iter() {
            item.render(frame);
        }
    }
    fn select_next_item(& mut self) {
        self.items[self.index_of_selected_item].unfocus();
        self.index_of_selected_item += 1;
        self.items[self.index_of_selected_item].focus();
    }
    fn select_previous_item(& mut self) {
        self.items[self.index_of_selected_item].unfocus();
        self.index_of_selected_item -= 1;
        self.items[self.index_of_selected_item].focus();
    }
}

impl CanHandleUserinput for TaskList <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Down => {
                if self.index_of_selected_item < self.items.len() {
                    self.select_next_item();
                }
            }
            KeyCode::Up => {
                if self.index_of_selected_item > 0 {
                    self.select_previous_item();
                }
            }
            _ => {}
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
    fn focus(& mut self) {
        self.task.focus();
    }
    fn unfocus(& mut self) {
        self.task.unfocus();
    }
    fn render(& self, frame: & mut Frame) {
        self.task.render(frame.buffer_mut());
    }
}
