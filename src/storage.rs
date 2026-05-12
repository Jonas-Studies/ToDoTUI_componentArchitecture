use crate::Task;
use std::fs::File;
use std::io::{Read, Write};

const DATA_PATH: & str = "/home/coding/Coding/Projects/ToDoTUI/data/tasks.json";

pub fn get_tasks() -> Option<Vec<Task>> {
    if let Ok(mut file) = File::open(DATA_PATH) {
        let mut content = String::new();
        file.read_to_string(& mut content).expect("Error reading file");
        Some(serde_json::from_str(& content).expect("Error converting json-string to task"))
    }
    else {
        None
    }
}

pub fn set_tasks(task: & Vec<Task>) {
    let mut file = File::create(DATA_PATH).expect("Error creating file");

    let content = serde_json::to_string(& task).expect("Error converting task to json-string");
    file.write_all(content.as_bytes()).expect("Error writing file");
}
