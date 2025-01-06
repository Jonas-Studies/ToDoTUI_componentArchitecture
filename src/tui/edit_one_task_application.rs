use crate::Task;

use super::{
    content::{Button, TextField},
    field::Field,
    traits::{CanHandleUserinput, MayDisplayCursor}
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Rect}, prelude::{Buffer, Position}, style::{Modifier, Stylize}, widgets::Paragraph, DefaultTerminal
};

pub enum SelectableItems {
    Name,
    Finish,
    Delete
}

pub struct Application <'applications_lifetime> {
    task_to_edit: Option<Task>,
    status: Field <Paragraph <'applications_lifetime>>,
    name: Field <TextField <'applications_lifetime>>,
    delete: Field <Button <'applications_lifetime>>,
    finish: Field <Button <'applications_lifetime>>,
    index_of_selected_item: SelectableItems
}

impl Application <'_> {
    pub fn new (task_to_edit: Task, mut area: Rect) -> Self {
        area.x += 2;
        area.width -= 4;
        area.y += 1;
        area.height -= 2;

        let [ status_area, name_area, delete_area ] = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3), Constraint::Length(3) ]
        ).spacing(1).areas(area);

        let status = Field::new(
            status_area,
            Paragraph::new(
                if task_to_edit.is_finished() {
                    String::from("Done:")
                }
                else {
                    String::from("Todo:")
                }
            ).add_modifier(Modifier::BOLD)
        );

        let mut name = Field::new(
            name_area,
            TextField::new(task_to_edit.get_name(), String::from("Name"))
        );
        name.focus();
        let index_of_selected_item = SelectableItems::Name;

        let [ delete_area, finish_area ] = Layout::horizontal(
            [ Constraint::Length(10), Constraint::Length(10) ]
        ).areas(delete_area);

        let delete = Field::new(
            delete_area,
            Button::new(String::from("Delete"))
        );

        let finish = Field::new(
            finish_area,
            Button::new(String::from("Finish"))
        );

        Self { task_to_edit: Some(task_to_edit), status, name, delete, finish, index_of_selected_item }
    }

    pub fn run(& mut self, terminal: & mut DefaultTerminal) -> Option<Task> {
        let mut result = self.task_to_edit.clone();

        loop {
            terminal.draw(
                |frame| {
                    self.render(frame.buffer_mut());

                    if let Some(cursor_position) = self.get_cursor_position() {
                        frame.set_cursor_position(cursor_position);
                    }
                }
            ).expect("Failed to draw the app to the terminal");

            if let Event::Key(key) = event::read().expect("Error reading an event") {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            break;
                        }
                        KeyCode::Enter => {
                            match self.index_of_selected_item {
                                SelectableItems::Delete => {
                                    result = None;
                                }
                                SelectableItems::Finish => {
                                }
                                _ => {
                                    self.task_to_edit.as_mut()?.finish();
                                    result = self.get_task().clone();
                                }
                            }
                            break;
                        }
                        _ => { self.handle_userinput(& key.code) }
                    }
                }
            }
        }

        return result;
    }

    pub fn render(& self, buffer: & mut Buffer) {
        self.status.render(buffer);
        self.name.render(buffer);
        self.delete.render(buffer);
        self.finish.render(buffer);
    }

    fn select_next_item(& mut self) {
        match self.index_of_selected_item {
            SelectableItems::Name => {
                self.name.unfocus();
                self.index_of_selected_item = SelectableItems::Delete;
                self.delete.focus();
            }
            SelectableItems::Delete => {
                self.delete.unfocus();
                self.index_of_selected_item = SelectableItems::Finish;
                self.finish.focus();
            }
            SelectableItems::Finish => {
                self.finish.unfocus();
                self.index_of_selected_item = SelectableItems::Name;
                self.name.focus();
            }
        }
    }

    fn select_previous_item(& mut self) {
        match self.index_of_selected_item {
            SelectableItems::Finish => {
                self.finish.unfocus();
                self.index_of_selected_item = SelectableItems::Delete;
                self.delete.focus();
            }
            SelectableItems::Delete => {
                self.delete.unfocus();
                self.index_of_selected_item = SelectableItems::Name;
                self.name.focus();
            }
            SelectableItems::Name => {
                self.name.unfocus();
                self.index_of_selected_item = SelectableItems::Finish;
                self.finish.focus();
            }
        }
    }

    pub fn get_task(& mut self) -> & Option<Task> {
        if let Some(task) = & mut self.task_to_edit {
            task.set_name(self.name.reference_content().get_value());
        }

        & self.task_to_edit
    }
}

impl MayDisplayCursor for Application <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        if let SelectableItems::Name = self.index_of_selected_item {
            self.name.get_cursor_position()
        }
        else {
            None
        }
    }
}

impl CanHandleUserinput for Application <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        match userinput {
            KeyCode::Tab => {
                self.select_next_item();
            }
            KeyCode::BackTab => {
                self.select_previous_item();
            }
            _ => {
                if let SelectableItems::Name = self.index_of_selected_item {
                    self.name.handle_userinput(userinput);
                }
            }
        }
    }
}
