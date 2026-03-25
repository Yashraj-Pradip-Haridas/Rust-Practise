use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    SHARED_DATA,
    commands::{del_key_value_pair, set_key_value_pair},
};

fn fetch_hashmap(file_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&file_name);
    if !path.exists() {
        println!("No snapshot found");
        return Err("No previous snapshot found".into());
    }
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let data: HashMap<String, String> = serde_json::from_reader(reader)?;
    if data.is_empty() {
        return Err("No previous snapshot found".into());
    }
    {
        let mut mutex_data = SHARED_DATA.lock().unwrap();
        *mutex_data = data.clone();
    }
    Ok(())
}

pub fn get_state(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    match fetch_hashmap("snapshot.json".to_string()) {
        Ok(_) => {
            println!("Snapshot loaded successfully");
        }
        Err(_) => {
            println!("No snapshot found");
        }
    };
    let path = Path::new(&file_path);
    if !path.exists() {
        println!("File does not exist");
        return Err("Path not found".into());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let command: Vec<&str> = line.trim().splitn(3, ' ').collect();
        // println!("{:?}", command);
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
                println!("Invalid command found");
                continue;
                // return Err("Invalid command".into());
            }
        }
    }
    Ok(())
}
