pub mod traits;
pub mod types_of_content;

use core::ops::{Deref, DerefMut};

use types_of_content::TypesOfContent;

pub struct Content<PossibleActions> where PossibleActions: Clone {
    content: TypesOfContent<PossibleActions>,
    can_be_focused: bool,
    can_handle_userinput: bool
}

impl <PossibleActions> Content<PossibleActions> where PossibleActions: Clone {
    pub fn new(content: TypesOfContent<PossibleActions>) -> Self {
        Self { content, can_be_focused: false, can_handle_userinput: false }
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
}

impl <PossibleActions> Deref for Content<PossibleActions> where PossibleActions: Clone {
    type Target = TypesOfContent<PossibleActions>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl <PossibleActions> DerefMut for Content<PossibleActions> where PossibleActions: Clone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
