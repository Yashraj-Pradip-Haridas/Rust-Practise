use std::io::{self};
mod commands;
mod storage;
mod structures;
fn main() {
    println!("Welcome to the Todo CLI built using RUST: ");
    let file_status = storage::create_file(String::from("data.json"));
    if file_status == false {
        println!("Failed to create file for storage");
        return;
    }
    loop {
        println!(
            "Please enter your choice: \n 1. Create new task \n 2. View all tasks \n 3. Update task \n 4.exit"
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let input_num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid choice");
                continue;
            }
        };

        match input_num {
            1 => {
                commands::add_task();
            }
            2 => {
                commands::print_all_tasks();
            }
            3 => {
                println!("Yet to be implemented");
            }
            4 => {
                break;
            }
            _ => {
                println!("Yet to be implemented");
            }
        }
    }
}
