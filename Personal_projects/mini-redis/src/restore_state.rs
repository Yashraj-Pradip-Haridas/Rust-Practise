use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::commands::{del_key_value_pair, set_key_value_pair};

pub fn get_state(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("Path not found".into());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let command: Vec<&str> = line.split_whitespace().collect();
        if command.len() < 1 {
            return Err("No command found".into());
        }
        match command[0].trim() {
            "SET" => {
                println!("Entered the set function");
                if command.len() < 3 || command[1].trim().is_empty() || command[2].trim().is_empty()
                {
                    return Err("Key/Value not found".into());
                }
                set_key_value_pair(command[1], command[2]);
            }
            "DEL" => {
                if command.len() < 2 || command[1].trim().is_empty() {
                    return Err("Key not found".into());
                }
                match del_key_value_pair(command[1]) {
                    Some(val) => {
                        println!("Removed value: {}", val);
                    }
                    None => {
                        println!("No value found");
                    }
                };
            }
            _ => {
                continue;
                // return Err("Invalid command".into());
            }
        }
    }
    Ok(())
}
