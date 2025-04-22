mod task;
mod tasks;
mod storage;
mod tui;

use task::Task;
use tasks::Tasks;

fn main() {
    let mut tasks = Tasks::from_ordered(vec![ Task::new(String::from("Add new task ...")) ]);

    if let Some(stored_tasks) = storage::get_tasks() {
        tasks.reference_content_mutable().extend(stored_tasks);
    }

    while let Some(index_of_selected_task) = tui::select_one_task(&tasks) {
        // The first element is a placeholder, when selecting it a new task has to be created
        if index_of_selected_task == 0 {
            let new_task = Task::new(String::new());
            
            if let Some(new_task) = tui::edit_one_task(new_task) {
                tasks.reference_content_mutable().insert(1, new_task);
            }

            // Finished tasks need to be placed in the second half
            if tasks.reference_content()[1].is_finished() == true {
                tasks.move_task_to_second_half(& 1);
            }
        }
        else {
            if let Some(edited_task) = tui::edit_one_task(tasks.reference_content()[index_of_selected_task].clone()) {
                if edited_task.is_finished() == true && tasks.reference_content()[index_of_selected_task].is_finished() == false {
                    tasks.reference_content_mutable()[index_of_selected_task] = edited_task;

                    // Finished tasks need to be placed in the second half
                    tasks.move_task_to_second_half(& index_of_selected_task);
                }
                else {
                    tasks.reference_content_mutable()[index_of_selected_task] = edited_task;
                }
            }
            else {
                tasks.reference_content_mutable().remove(index_of_selected_task);
            }
        }
    }

    storage::set_tasks(& tasks.reference_content_mutable().split_off(1));
}
