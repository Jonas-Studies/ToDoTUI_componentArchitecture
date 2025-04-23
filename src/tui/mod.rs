mod content;
mod edit_one_task_application;
mod select_one_task_application;

use content::traits::{CanBeFocused, CanContainValue, CanHandleUserinput, MayDisplayCursor};
use edit_one_task_application::PossibleActions;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::{tasks::Tasks, Task};

pub fn edit_one_task (task_to_edit: Task) -> Option<Task> {
    let mut result = None;
    let mut terminal = ratatui::init();

    let mut app = edit_one_task_application::Application::new(task_to_edit);

    loop {
        terminal.draw(
            |frame| {
                app.render_focused(frame.area(), frame.buffer_mut());

                if let Some(cursor_position) = app.get_cursor_position(frame.area()) {
                    frame.set_cursor_position(cursor_position);
                }
            }
        ).expect("Error");

        let mut action = None;

        if let Event::Key(key) = event::read().expect("Error") {
            if key.kind == KeyEventKind::Press {
                action = app.handle_userinpt(key.code);
            }
        }

        if let Some(action) = action {
            match action {
                PossibleActions::Exit => {
                    break;
                }
                PossibleActions::Save => {
                    result = Some(app.get_value());
                    break;
                }
                PossibleActions::Finish => {
                    let mut task = app.get_value();
                    task.finish();
                    result = Some(task);
                    break;
                }
                PossibleActions::Delete => {
                    result = None;
                    break;
                }
            }
        }
    }

    ratatui::restore();

    return result;
}

pub fn select_one_task (tasks_to_select_from: &Tasks) -> Option<usize> {
    let mut result = None;

    let mut terminal = ratatui::init();

    let mut app = select_one_task_application::Application::new(tasks_to_select_from);

    loop {
        terminal.draw(
            |frame| {
                app.render_focused(frame.area(), frame.buffer_mut());
            }
        ).expect("Error");

        let mut action = None;

        if let Event::Key(key) = event::read().expect("Error") {
            if key.kind == KeyEventKind::Press {
                action = app.handle_userinpt(key.code);
            }
        }

        if let Some(action) = action {
            match action {
                select_one_task_application::PossibleActions::Exit => {
                    break;
                }
                select_one_task_application::PossibleActions::Select => {
                    result = Some(app.get_value());
                    break;
                }
            }
        }
    }

    ratatui::restore();
    return result;
}
