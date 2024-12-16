mod traits;
mod content;
mod field;
mod edit_one_task_application;
mod select_one_task_application;

use crate::Task;

use ratatui::crossterm::event::{self, Event, KeyEventKind, KeyCode};

use traits::{CanHandleUserinput, MayDisplayCursor};

pub fn edit_one_task (task_to_edit: & mut Task) {
    let mut terminal = ratatui::init();
    let mut app = edit_one_task_application::Application::new(task_to_edit, terminal.get_frame().area());

    loop {
        terminal.draw(
            |frame| {
                app.render(frame.buffer_mut());

                if let Some(cursor_position) = app.get_cursor_position() {
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
                        app.save_task(task_to_edit);
                        break;
                    }
                    _ => { app.handle_userinput(& key.code) }
                }
            }
        }
    }

    ratatui::restore();
}

pub fn select_one_task (tasks_to_select_from:& Vec<Task>) -> usize {
    let mut terminal = ratatui::init();
    let mut app = select_one_task_application::Application::new(tasks_to_select_from, terminal.get_frame().area());

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
                        break;
                    }
                    _ => { app.handle_userinput(& key.code) }
                }
            }
        }
    }

    ratatui::restore();

    0
}
