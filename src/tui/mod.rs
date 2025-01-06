mod traits;
mod content;
mod field;
mod edit_one_task_application;
mod select_one_task_application;

use crate::Task;

pub fn edit_one_task (task_to_edit: Task) -> Option<Task> {
    let mut terminal = ratatui::init();
    
    let result = edit_one_task_application::Application::new(
        task_to_edit,
        terminal.get_frame().area()
    ).run(& mut terminal);

    ratatui::restore();

    return result;
}

pub fn select_one_task (tasks_to_select_from: & Vec<Task>) -> Option<usize> {
    let mut terminal = ratatui::init();

    let result = select_one_task_application::Application::new(
        tasks_to_select_from,
        terminal.get_frame().area()
    ).run(& mut terminal);

    ratatui::restore();
    result
}
