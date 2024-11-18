use ratatui::{
    prelude::Position,
    widgets::WidgetRef
};

pub trait IsContent: WidgetRef + CanBeFocused {}

pub trait CanBeFocused {
    fn focus(& mut self);
    fn unfocus(& mut self);
}

pub trait MayDisplayCursor {
    fn get_cursor_position (& self) -> Option<Position>;
}
