use ratatui::{crossterm::event::KeyCode, layout::{Constraint, Layout}};

use crate::task::Task;

use super::content::{traits::{CanBeFocused, CanBeRendered, CanHandleUserinput}, types_of_content::{textinput::Textinput, title::Title, TypesOfContent}, Content};

pub struct Application {
    layout: Layout,
    task: Task,
    content: Vec<Content>,
    nr_of_focused_content: usize
}

impl Application {
    pub fn new(task_to_edit: Task) -> Self {
        let layout = Layout::vertical(
            [ Constraint::Length(1), Constraint::Length(3) ]
        ).spacing(1).vertical_margin(1).horizontal_margin(3);

        let mut content = Vec::new();

        content.push(
            Content::new(TypesOfContent::Title(Self::get_title(task_to_edit.is_finished())))
        );
        content.push(
            Content::new(
                TypesOfContent::Textinput(Textinput::new(task_to_edit.get_name(), String::from("Name")))
            ).as_can_be_focused()
        );

        Self { layout, task: task_to_edit, content, nr_of_focused_content: 1 }
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
    fn reference_focused_content(&self) -> &Content {
        &self.content[self.nr_of_focused_content]
    }
    fn reference_focused_content_mutable(&mut self) -> &mut Content {
        &mut self.content[self.nr_of_focused_content]
    }
    fn get_nr_of_last_content(&self) -> usize {
        self.content.len() - 1
    }
    fn get_nr_of_first_content(&self) -> usize {
        0
    }
    fn focus_next_content(&mut self) {
        if self.nr_of_focused_content == self.get_nr_of_last_content() {
            self.nr_of_focused_content = self.get_nr_of_first_content();
        }
        else {
            self.nr_of_focused_content += 1;
        }
        
        if self.reference_focused_content().can_be_focused() == false {
            self.focus_next_content();
        }
    }
    fn focus_previous_content(&mut self) {
        if self.nr_of_focused_content == self.get_nr_of_first_content() {
            self.nr_of_focused_content = self.get_nr_of_last_content();
        }
        else {
            self.nr_of_focused_content -= 1;
        }
        
        if self.reference_focused_content().can_be_focused() == false {
            self.focus_previous_content();
        }
    }
}

impl CanBeRendered for Application {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }
    }
}

impl CanBeFocused for Application {
    fn render_focused (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            if nr_of_area == self.nr_of_focused_content {
                self.content[nr_of_area].render_focused(*area, buffer);
            }
            else {
                self.content[nr_of_area].render(*area, buffer);
            }
        }
    }
}

pub enum PossibleActions {
    Exit
}

impl CanHandleUserinput<PossibleActions> for Application {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        let mut result = None;

        match userinput {
            KeyCode::Esc => {
                result = Some(PossibleActions::Exit);
            }
            KeyCode::Tab => {
                self.focus_next_content();
            }
            KeyCode::BackTab => {
                self.focus_previous_content();
            }
            _ => {
                result = self.reference_focused_content_mutable().handle_userinpt(userinput)
            }
        }

        return result;
    }
}
