use crate::Task;

use super::field::Field;

use ratatui::{
    widgets::Paragraph,
    layout::{Rect, Layout, Constraint}
};

pub struct Application <'applications_lifetime> {
    status: Field <Paragraph <'applications_lifetime>>,
    name: Field <Paragraph <'applications_lifetime>>
}

impl Application <'_> {
    pub fn new (task_to_edit: & Task, area: Rect) -> Self {
        let [ status_area, name_area ] = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(1) ]
        ).areas(area);

        let status = Field::new(
            status_area,
            Paragraph::new(
                if task_to_edit.is_finished() {
                    String::from("Done:")
                }
                else {
                    String::from("Todo:")
                }
            )
        );

        let name = Field::new(
            name_area,
            Paragraph::new(task_to_edit.get_name())
        );

        Self { status, name }
    }
}
