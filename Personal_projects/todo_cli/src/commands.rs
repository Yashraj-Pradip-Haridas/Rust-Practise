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
    println!("{:?}", data);
}
