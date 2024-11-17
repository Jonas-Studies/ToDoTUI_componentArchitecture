use super::traits::IsContent;

use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{WidgetRef, Block, Paragraph}
};

impl IsContent for Paragraph <'_> {}

pub struct TextField <'textfields_lifetime> {
    text: String,
    borders: Block <'textfields_lifetime>
}

impl TextField <'_> {
    pub fn new (text: String, title: String) -> Self {
        let borders = Block::bordered().title(title);

        Self { text, borders }
    }
}

impl IsContent for TextField <'_> {}

impl WidgetRef for TextField <'_> {
    fn render_ref(& self, area: Rect, buffer: & mut Buffer) {
        Paragraph::new(self.text.clone()).block(self.borders.clone()).render_ref(area, buffer);
    }
}
