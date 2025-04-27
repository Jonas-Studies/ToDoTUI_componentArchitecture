use core::{clone::Clone, ops::DerefMut};

use ratatui::{crossterm::event::KeyCode, layout::Layout, style::{Style, Stylize}, widgets::{Block, Widget}};

use crate::tui::content::{possible_actions::PossibleActions, traits::{CanBeFocused, CanBeRendered, CanHandleUserinput, MayDisplayCursor}, Content};

use super::TypesOfContent;

pub struct Container<'containers_lifetime> {
    layout: Layout,
    content: Vec<Content<'containers_lifetime>>,
    nr_of_focused_content: usize,
    borders: Option<Block<'containers_lifetime>>
}

impl <'callers_lifetime> Container<'callers_lifetime> {
    pub fn new(layout: Layout) -> Self {
        let content = Vec::new();

        Self { layout, content, nr_of_focused_content: 0, borders: None }
    }
    pub fn with_borders(mut self, borders: Block<'callers_lifetime>) -> Self {
        self.borders = Some(borders);
        self
    }
    pub fn push_content(&mut self, content: Content<'callers_lifetime>) {
        self.content.push(content);
    }
    fn reference_focused_content(&self) -> &Content {
        &self.content[self.nr_of_focused_content]
    }
    fn reference_focused_content_mutable(&mut self) -> &mut Content<'callers_lifetime> {
        &mut self.content[self.nr_of_focused_content]
    }
    pub fn reference_content(&self, nr: usize) -> &Content {
        &self.content[nr]
    }
    pub fn get_nr_of_focused_content(&self) -> usize {
        self.nr_of_focused_content.clone()
    }
    fn get_nr_of_last_content(&self) -> usize {
        self.content.len() - 1
    }
    fn get_nr_of_first_content(&self) -> usize {
        0
    }
    pub fn focus_first(&mut self) -> Option<usize> {
        self.nr_of_focused_content = self.get_nr_of_first_content();

        if self.reference_focused_content().can_be_focused() {
            Some(self.nr_of_focused_content)
        }
        else {
            self.focus_next()
        }
    }
    fn focus_last(&mut self) -> Option<usize> {
        self.nr_of_focused_content = self.get_nr_of_last_content();

        if self.reference_focused_content().can_be_focused() {
            Some(self.nr_of_focused_content)
        }
        else {
            self.focus_previous()
        }
    }
    fn focus_next(&mut self) -> Option<usize> {
        if self.nr_of_focused_content == self.get_nr_of_last_content() {
            None
        }
        else {
            self.nr_of_focused_content += 1;

            if self.reference_focused_content().can_be_focused() {
                Some(self.nr_of_focused_content)
            }
            else {
                self.focus_next()
            }
        }
    }
    fn focus_previous(&mut self) -> Option<usize> {
        if self.nr_of_focused_content == self.get_nr_of_first_content() {
            None
        }
        else {
            self.nr_of_focused_content -= 1;

            if self.reference_focused_content().can_be_focused() {
                Some(self.nr_of_focused_content)
            }
            else {
                self.focus_previous()
            }
        }
    }
}

impl CanBeRendered for Container<'_> {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }

        if let Some(borders) = self.borders.clone() {
            borders.render(area, buffer);
        }
    }
}

impl CanBeFocused for Container<'_> {
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
        
        if let Some(borders) = self.borders.clone() {
            borders.clone().border_style( Style::new().yellow() ).render(area, buffer);
        }
    }
}

impl  CanHandleUserinput for Container<'_> {
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions> {
        let mut result = None;

        // Special case when selected content is a container too. Then, in case the focus is
        // supposed to be changed, it needs to be decided if that should happen in the current
        // or the focused container
        if let TypesOfContent::Contaier(container) = self.reference_focused_content_mutable().deref_mut() {
            match userinput {
                KeyCode::Tab => {
                    if container.focus_next().is_none() {
                        if self.focus_next().is_none() {
                            self.focus_first();
                        }
                    }
                }
                KeyCode::BackTab => {
                    if container.focus_previous().is_none() {
                        if self.focus_previous().is_none() {
                            self.focus_last();
                        }
                    }
                }
                _ => {
                    if self.reference_focused_content_mutable().can_handle_userinput() {
                        result = self.reference_focused_content_mutable().handle_userinpt(userinput);
                    }
                }
            }
        }
        else {
            match userinput {
                KeyCode::Tab => {
                    if self.focus_next().is_none() {
                        self.focus_first();
                    }
                    if let TypesOfContent::Contaier(container) = self.reference_focused_content_mutable().deref_mut() {
                        container.focus_first();
                    }
                }
                KeyCode::BackTab => {
                    if self.focus_previous().is_none() {
                        self.focus_last();
                    }
                    if let TypesOfContent::Contaier(container) = self.reference_focused_content_mutable().deref_mut() {
                        container.focus_last();
                    }
                }
                _ => {
                    if self.reference_focused_content_mutable().can_handle_userinput() {
                        result = self.reference_focused_content_mutable().handle_userinpt(userinput);
                    }
                }
            }

        }
        
        return result;
    }
}

impl  MayDisplayCursor for Container<'_> {
    fn get_cursor_position(&self, area: ratatui::prelude::Rect) -> Option<ratatui::prelude::Position> {
        let mut result = None;

        if self.reference_focused_content().can_display_cursor() {
            let areas = self.layout.split(area);

            result = self.reference_focused_content().get_cursor_position(areas[self.nr_of_focused_content]);
        }

        return result;
    }
}
