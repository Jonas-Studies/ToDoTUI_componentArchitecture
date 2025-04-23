use ratatui::{layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, Paragraph, Widget}};

use crate::tui::content::traits::{CanBeFocused, CanBeRendered};

pub struct Checkbox {
    value: bool
}

impl Checkbox {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    fn get_widget<'callers_lifetime>(&self, border_style: Style) -> Paragraph<'callers_lifetime> {
        Paragraph::new(
            String::from(
                if self.value == true {
                    " X "
                }
                else {
                    "   "
                }
            )
        ).block(Block::bordered().border_style(border_style))
    }
}

impl CanBeRendered for Checkbox {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let area_as_5x5 = get_area_5x5(area);

        self.get_widget(Style::new()).render(area_as_5x5, buffer);
    }
}

impl CanBeFocused for Checkbox {
    fn render_focused (&self, area: Rect, buffer: &mut ratatui::prelude::Buffer) {
        let area_as_5x5 = get_area_5x5(area);

        self.get_widget(Style::new().yellow()).render(area_as_5x5, buffer);
    }
}

fn get_area_5x5(area: Rect) -> Rect {
    let [ area ] = Layout::horizontal( [ Constraint::Length(5) ] ).areas(area);
    let [ area ] = Layout::vertical( [ Constraint::Length(5) ] ).areas(area);
    return area;
}
