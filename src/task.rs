use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    is_finished: bool
}

impl Task {
    pub fn new (name: String) -> Self {
        Self { name, is_finished: false }
    }

    pub fn get_name (& self) -> String {
        self.name.clone()
    }
    pub fn set_name (& mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn is_finished (& self) -> bool {
        self.is_finished
    }
    pub fn finish (& mut self) {
        self.is_finished = true;
    }
}
