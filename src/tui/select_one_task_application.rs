use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, widgets::Paragraph, DefaultTerminal, Frame};

use crate::task::Task;

use super::{field::Field, traits::CanHandleUserinput};

pub struct Application <'lists_lifetime> {
    items: Vec<TaskListItem <'lists_lifetime>>,
    index_of_selected_item: usize
}

impl Application <'_> {
    pub fn new (tasks: & Vec<Task>, area: Rect) -> Self {
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
    pub fn run(& mut self, terminal: & mut DefaultTerminal) -> Option<usize> {
        let mut result = None;

        loop {
            terminal.draw(
                |frame| {
                    self.render(frame);
                }
            ).expect("Failed to draw the app to the terminal");

            if let Event::Key(key) = event::read().expect("Error reading an event") {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Enter => {
                            result = Some(self.get_index_of_selected_item());
                            break;
                        }
                        _ => { self.handle_userinput(& key.code) }
                    }
                }
            }
        }

        result
    }
    fn render (& self, frame: & mut Frame) {
        for item in self.items.iter() {
            item.render(frame);
        }
    }
    fn get_index_of_selected_item(& self) -> usize {
        self.index_of_selected_item
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

impl CanHandleUserinput for Application <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Down => {
                if self.index_of_selected_item < self.items.len() - 1 {
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
