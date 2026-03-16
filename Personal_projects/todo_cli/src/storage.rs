use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use crate::structures::Task;

pub fn create_file(file_name: String) -> bool {
    if Path::new(&file_name).exists() {
        return true;
    }
    let path = Path::new(&file_name);
    let display = path.display();
    let _file = match File::create(&path) {
        Err(why) => {
            // crashes the program if file is not created properly
            // panic!("Couldn't create file {}: {}", display, why);
            println!("Could'nt create file {}: {}", display, why);
            return false;
        }
        Ok(file) => {
            println!("File created successfully");
            file
        }
    };
    true
}

pub fn write_to_file(task: &Task) -> bool {
    let mut file = match OpenOptions::new().append(true).open("data.json") {
        Ok(file) => file,
        Err(_) => {
            return false;
        }
    };

    // serializing the struct to json
    let json_string = match serde_json::to_string(task) {
        Ok(json_string) => json_string,
        Err(_) => {
            println!("Failed to serialize");
            return false;
        }
    };

    // writing json string followed by new line character
    match writeln!(&mut file, "{}", json_string) {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

pub fn read_file() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();
    let file = match File::open("data.json") {
        Err(_) => {
            println!("Failed to read the file!!");
            return tasks;
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        };
        // skip empty lines if any
        if line.trim().is_empty() {
            println!("Empty line");
            continue;
        }
        let task: Task = match serde_json::from_str(&line) {
            Ok(task) => task,
            Err(_) => {
                println!("Failed to deserialize");
                continue;
            }
        };
        tasks.push(task)
    }
    tasks
}

pub fn update_status(tasks: Vec<Task>) -> bool {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.json")
    {
        Ok(file) => file,
        Err(_) => {
            return false;
        }
    };
    for task in tasks {
        // serializing the struct to json
        let json_string = match serde_json::to_string(&task) {
            Ok(json_string) => json_string,
            Err(_) => {
                println!("Failed to serialize");
                continue;
            }
        };

        // writing json string followed by new line character
        match writeln!(&mut file, "{}", json_string) {
            Ok(_) => {}
            Err(_) => continue,
        };
    }
    true
}
