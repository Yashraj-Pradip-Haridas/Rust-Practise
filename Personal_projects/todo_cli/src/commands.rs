use crate::storage;
use crate::structures::Task;
use std::io;
pub fn add_task() -> bool {
    println!("Please enter your task: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let input = input.trim().to_string();
    let task = Task {
        name: input,
        status: false,
    };

    // code to append the task to file/folder
    storage::write_to_file(&task);
    true
}

pub fn print_all_tasks() {
    let data = storage::read_file();
    println!("{} - {}", "Task", "Status");
    for i in data {
        println!("{} - {}", i.name, i.status);
    }
}

pub fn mark_completed() {
    let mut data = storage::read_file();
    println!("Enter the value to be updated: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let input = input.trim().to_string();

    for i in data.iter_mut() {
        if i.name == input {
            i.status = true;
        }
    }
    storage::update_status(data);
    // better version
    //     if let Some(task) = data.iter_mut().find(|t| t.name == input) {
    //     task.status = true;
    // }
}

pub fn remove_task() {
    let mut data = storage::read_file();
    println!("Enter the value to be removed: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let input = input.trim().to_string();

    data.retain(|value| value.name != input);
    storage::update_status(data);
}
