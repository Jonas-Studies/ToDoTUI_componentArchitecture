pub mod possible_actions;
pub mod traits;
pub mod types_of_content;

use core::ops::{Deref, DerefMut};

use types_of_content::TypesOfContent;

pub struct Content<'contents_lifetime> {
    content: TypesOfContent<'contents_lifetime>,
    can_be_focused: bool,
    can_handle_userinput: bool,
    can_display_cursor: bool
}

impl <'callers_lifetime> Content<'callers_lifetime> {
    pub fn new(content: TypesOfContent<'callers_lifetime>) -> Self {
        Self { content, can_be_focused: false, can_handle_userinput: false, can_display_cursor: false }
    }
    pub fn as_can_be_focused(mut self) -> Self {
        self.can_be_focused = true;
        self
    }
    pub fn can_be_focused(&self) -> bool {
        self.can_be_focused.clone()
    }
    pub fn as_can_handle_userinput(mut self) -> Self {
        self.can_handle_userinput = true;
        self
    }
    pub fn can_handle_userinput(&self) -> bool {
        self.can_handle_userinput.clone()
    }
    pub fn as_can_display_cursor(mut self) -> Self {
        self.can_display_cursor = true;
        self
    }
    pub fn can_display_cursor(&self) -> bool {
        self.can_display_cursor.clone()
    }
}

impl <'callers_lifetime> Deref for Content<'callers_lifetime> {
    type Target = TypesOfContent<'callers_lifetime>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Content<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
