use ratatui::{buffer::Buffer, layout::Rect, widgets::{List, ListItem, WidgetRef}};

use crate::task::Task;

pub struct Application <'applications_lifetime> {
    area: Rect,
    content: List <'applications_lifetime>
}

impl Application <'_> {
    pub fn new (tasks_to_select_from: & Vec<Task>, area: Rect) -> Self {
        let content = List::new(
            tasks_to_select_from.iter().map(
                |task| {
                    ListItem::new(
                        String::from(
                            if task.is_finished() {
                                "Done: "
                            }
                            else {
                                "Todo: "
                            }
                        ) + & task.get_name()
                    )
                }
            ).collect::<Vec<ListItem>>()
        );

        Self { content, area }
    }

    pub fn render (& self, buffer: & mut Buffer) {
        self.content.render_ref(self.area, buffer);
    }
}
