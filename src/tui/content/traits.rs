use ratatui::{crossterm::event::KeyCode, prelude::{Buffer, Rect}};

pub trait CanBeRendered {
    fn render (&self, area: Rect, buffer: &mut Buffer);
}

pub trait CanBeFocused {
    fn render_focused (&self, area: Rect, buffer: &mut Buffer);
}

pub trait CanHandleUserinput<PossibleActions> {
    // fn is supposed to return a type of an enum of possible actions to result from userinput.
    // This is supposed to be getting defined at an controlling point in the Software.
    fn handle_userinpt(&mut self, userinput: KeyCode) -> Option<PossibleActions>;
}

pub trait CanContainValue<TypeOfValue> {
    fn get_value(&self) -> TypeOfValue;
}
