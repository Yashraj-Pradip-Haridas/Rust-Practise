use crate::{
    SHARED_DATA,
    storage::{clear_log_file, create_snapshot, write_to_file},
};
use std::{io::Write, net::TcpStream};

pub fn handle_commands(
    command: String,
    mut stream: &TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let str_command = command.clone();
    let command: Vec<&str> = command.trim().splitn(3, ' ').collect();
    // println!("{:?}", command);
    if command.len() < 1 {
        return Err("No command found".into());
    }
    match command[0].trim() {
        "GET" => {
            if command.len() < 2 || command[1].trim().is_empty() {
                return Err("Key not found".into());
            }
            // println!("Entered the get function");
            let mut out_value = get_key_value_pair(command[1]);
            println!(" Value: {}", out_value);
            if out_value.is_empty() {
                out_value = "(nil)".to_string();
            }
            stream.write_all((out_value + "\n").as_bytes())?;
        }
        "SET" => {
            // println!("Entered the set function");
            if command.len() < 3 || command[1].trim().is_empty() || command[2].trim().is_empty() {
                return Err("Key/Value not found".into());
            }
            if command[2].len() > 50 {
                stream.write_all("Please enter smaller value\n".as_bytes())?;
                return Err("Input out of size".into());
            }
            set_key_value_pair(command[1], command[2]);
            stream.write_all(String::from("OK\n").as_bytes())?;
            write_to_file("data.txt".to_string(), str_command)?;
        }
        "DEL" => {
            if command.len() < 2 || command[1].trim().is_empty() {
                return Err("Key not found".into());
            }
            match del_key_value_pair(command[1]) {
                Some(val) => {
                    // println!("Removed value: {}", val);
                    stream.write_all((val + "\n").as_bytes())?;
                    write_to_file("data.txt".to_string(), str_command)?;
                }
                None => {
                    println!("No value found");
                    stream.write_all("(nil)\n".as_bytes())?;
                }
            };
        }
        "SNAPSHOT" => {
            if command.len() < 1 || command[0].trim().is_empty() {
                return Err("Key not found".into());
            }
            create_snapshot("snapshot.json".to_string())?;
            clear_log_file("data.txt".to_string())?;
            stream.write_all("Snapshot successful\n".as_bytes())?;
        }
        _ => {
            stream.write_all("Invalid command \n".as_bytes())?;
            return Err("Invalid command".into());
        }
    }

    Ok(())
}

pub fn set_key_value_pair(key: &str, value: &str) {
    {
        let mut data = SHARED_DATA.lock().unwrap();
        data.insert(key.to_string(), value.to_string());
    }
}

pub fn get_key_value_pair(key: &str) -> String {
    let data = SHARED_DATA.lock().unwrap();
    let value = match data.get(key) {
        Some(value) => value.clone(),
        None => return String::new(),
    };
    value
}

pub fn del_key_value_pair(key: &str) -> Option<String> {
    let mut data = SHARED_DATA.lock().unwrap();
    data.remove(key)
}
