use ratatui::layout::Layout;

use crate::tui::content::{traits::CanBeRendered, Content};

pub struct Container<PossibleActions> where PossibleActions: Clone {
    layout: Layout,
    content: Vec<Content<PossibleActions>>,
    nr_of_focused_content: Option<usize> 
}

impl <PossibleActions> Container<PossibleActions> where PossibleActions: Clone {
    fn new(layout: Layout) -> Self {
        let content = Vec::new();

        Self { layout, content, nr_of_focused_content: None }
    }
}

impl <PossibleActions> CanBeRendered for Container<PossibleActions> where PossibleActions: Clone {
    fn render (&self, area: ratatui::prelude::Rect, buffer: &mut ratatui::prelude::Buffer) {
        let areas = self.layout.split(area);

        for (nr_of_area, area) in areas.iter().enumerate() {
            self.content[nr_of_area].render(*area, buffer);
        }
    }
}
