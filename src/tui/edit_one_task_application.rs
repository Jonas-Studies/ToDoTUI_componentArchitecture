use std::ops::Deref;

use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout}, widgets::Block};

use crate::task::Task;

use super::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanBeRendered, CanContainValue, CanHandleUserinput, MayDisplayCursor}, types_of_content::{button::Button, container::Container, textinput::Textinput, title::Title, TypesOfContent}, Content};

pub struct Application<'applications_lifetime> {
    task: Task,
    content: Container<'applications_lifetime>
}

impl Application<'_> {
    pub fn new(task_to_edit: Task) -> Self {
        let layout = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3), Constraint::Length(3) ]
        ).spacing(1).vertical_margin(1).horizontal_margin(3);

        let mut content = Container::new(layout);

        content.push_content(
            Content::new(TypesOfContent::Title(Self::get_title(task_to_edit.is_finished())))
        );
        content.push_content(
            Content::new(
                TypesOfContent::Textinput(Textinput::new(task_to_edit.get_name(), String::from("Name")))
            ).as_can_be_focused().as_can_handle_userinput().as_can_display_cursor()
        );
        content.push_content(
            Content::new(
                TypesOfContent::Contaier(Self::get_buttons_as_container())
            ).as_can_be_focused().as_can_handle_userinput()
        );

        content.focus_first();

        Self { task: task_to_edit, content }
    }
    fn get_title(is_task_finished: bool) -> Title {
        Title::new(
            String::from(
                if is_task_finished {
                    "Done:"
                }
                else {
                    "ToDo:"
                }
            )
        )
    }
    fn get_buttons_as_container<'callers_lifetime>() -> Container<'callers_lifetime> {
        let layout = Layout::horizontal(
            [ Constraint::Length(10), Constraint::Length(10) ]
        );

        let mut result = Container::new(layout);

        result.push_content(
            Content::new(
                TypesOfContent::Button(Button::new(String::from("Finish"), PossibleActions::Finish))
            ).as_can_be_focused().as_can_handle_userinput()
        );
        result.push_content(
            Content::new(
                TypesOfContent::Button(Button::new(String::from("Delete"), PossibleActions::Delete))
            ).as_can_be_focused().as_can_handle_userinput()
        );

        return result;
    }
}

impl CanBeRendered for Application <'_> {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        self.content.render(area, buffer);
    }
}

impl CanBeFocused for Application <'_> {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        self.content.render_focused(area, buffer);
    }
}

impl CanHandleUserinput for Application <'_> {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        let mut result = self.content.handle_userinpt(userinput);

        if result.is_none() {
            match userinput {
                KeyCode::Esc => {
                    result = Some(PossibleActions::Cancel);
                }
                KeyCode::Enter => {
                    result = Some(PossibleActions::Save);
                }
                _ => {  }
            }
        }

        return result;
    }
}

impl CanContainValue<Task> for Application <'_> {
    fn get_value(&self) -> Task {
        let mut result = self.task.clone();

        if let TypesOfContent::Textinput(name) = self.content.reference_content(1).deref() {
            result.set_name(name.get_value());
        }

        return result;
    }
}

impl MayDisplayCursor for Application <'_> {
    fn get_cursor_position(&self, area: ratatui::prelude::Rect) -> Option<ratatui::prelude::Position> {
        self.content.get_cursor_position(area)
    }
}
