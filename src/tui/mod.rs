mod traits;
mod content;
mod field;
mod edit_one_task_application;

use crate::Task;

pub fn edit_one_task (task_to_edit: & mut Task) {
    let mut terminal = ratatui::init();

    let app = edit_one_task_application::Application::new(task_to_edit, terminal.get_frame().area());

    ratatui::restore();
}
