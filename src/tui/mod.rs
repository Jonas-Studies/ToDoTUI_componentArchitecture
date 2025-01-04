mod traits;
mod content;
mod field;
mod edit_one_task_application;
mod select_one_task_application;

use crate::Task;

use ratatui::crossterm::event::{self, Event, KeyEventKind, KeyCode};

use traits::CanHandleUserinput;

pub fn edit_one_task (task_to_edit: Task) -> Option<Task> {
    let mut terminal = ratatui::init();
    
    let result = edit_one_task_application::Application::new(
        task_to_edit,
        terminal.get_frame().area()
    ).run(& mut terminal);

    ratatui::restore();

    return result;
}

pub fn select_one_task (tasks_to_select_from:& Vec<Task>) -> Option<usize> {
    let mut terminal = ratatui::init();
    let mut app = select_one_task_application::Application::new(tasks_to_select_from, terminal.get_frame().area());
    let mut result = None;

    loop {
        terminal.draw(
            |frame| {
                app.render(frame);
            }
        ).expect("Failed to draw the app to the terminal");

        if let Event::Key(key) = event::read().expect("Error reading an event") {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Enter => {
                        result = Some(app.get_index_of_selected_item());
                        break;
                    }
                    _ => { app.handle_userinput(& key.code) }
                }
            }
        }
    }

    ratatui::restore();
    result
}
