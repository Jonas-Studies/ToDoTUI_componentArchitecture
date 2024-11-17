mod traits;
mod content;
mod field;
mod edit_one_task_application;

use crate::Task;

use ratatui::crossterm::event::{self, Event, KeyEventKind, KeyCode};

pub fn edit_one_task (task_to_edit: & mut Task) {
    let mut terminal = ratatui::init();

    let app = edit_one_task_application::Application::new(task_to_edit, terminal.get_frame().area());

    terminal.draw(
        |frame| {
            app.render(frame.buffer_mut())
        }
    ).expect("Failed to draw the app to the terminal");

    loop {
        if let Event::Key(key) = event::read().expect("Error reading an event") {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    ratatui::restore();
}
