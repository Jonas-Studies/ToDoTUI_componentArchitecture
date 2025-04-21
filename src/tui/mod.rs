mod content;

use crate::Task;

pub fn edit_one_task (task_to_edit: Task) -> Option<Task> {
    let mut terminal = ratatui::init();
    
    ratatui::restore();

    return None;
}

pub fn select_one_task (tasks_to_select_from: & Vec<Task>) -> Option<usize> {
    let mut terminal = ratatui::init();

    ratatui::restore();
    return None;
}
