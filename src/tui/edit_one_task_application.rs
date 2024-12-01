use crate::Task;

use super::{
    content::TextField,
    field::Field,
    traits::{CanBeFocused, CanHandleUserinput, MayDisplayCursor}
};

use ratatui::{
    crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, prelude::{Buffer, Position}, style::{Modifier, Stylize}, widgets::Paragraph
};

pub struct Application <'applications_lifetime> {
    status: Field <Paragraph <'applications_lifetime>>,
    name: Field <TextField <'applications_lifetime>>
}

impl Application <'_> {
    pub fn new (task_to_edit: & Task, mut area: Rect) -> Self {
        area.x += 2;
        area.width -= 4;
        area.y += 1;
        area.height -= 2;

        let [ status_area, name_area ] = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3) ]
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

        Self { status, name }
    }

    pub fn render (& self, buffer: & mut Buffer) {
        self.status.render(buffer);
        self.name.render(buffer);
    }

    pub fn save_task(& self, task: & mut Task) {
        task.set_name(self.name.reference_content().get_text());
    }
}

impl MayDisplayCursor for Application <'_> {
    fn get_cursor_position(& self) -> Option<Position> {
        self.name.get_cursor_position()
    }
}

impl CanHandleUserinput for Application <'_> {
    fn handle_userinput(& mut self, userinput: & KeyCode) {
        self.name.handle_userinput(userinput);
    }
}
