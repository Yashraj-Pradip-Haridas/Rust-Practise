use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
static SHARED_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        let address = stream.peer_addr()?;
        thread::spawn(move || {
            println!("Connection {} Established", address);

            let _ = handle_client(stream);
        });
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = BufReader::new(&stream);
    let address = stream.peer_addr()?;
    loop {
        let mut body_buffer = String::new();
        let data = buff.read_line(&mut body_buffer)?;
        // println!("Entered thread {}", address);
        println!("{}: {}", address, body_buffer.trim());
        if data == 0 {
            break;
        }
        if body_buffer == "EXIT".trim().to_string() {
            break;
        }

        // println!("{:?}", command);

        match handle_commands(body_buffer, &stream) {
            Ok(_) => {
                println!("Command successful");
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
    }
    println!("{}", "Client Disconnected");

    Ok(())
}

fn handle_commands(
    command: String,
    mut stream: &TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let command: Vec<&str> = command.split_whitespace().collect();
    if command.len() < 1 {
        return Err("No command found".into());
    }
    match command[0].trim() {
        "GET" => {
            if command.len() < 2 || command[1].trim().is_empty() {
                return Err("Key not found".into());
            }
            println!("Entered the get function");
            let mut out_value = get_key_value_pair(command[1]);
            if out_value.is_empty() {
                out_value = "(nil)".to_string();
            }
            println!(" Value: {}", out_value);
            stream.write_all((out_value + "\n").as_bytes())?;
        }
        "SET" => {
            println!("Entered the set function");
            if command.len() < 3 || command[1].trim().is_empty() || command[2].trim().is_empty() {
                return Err("Key/Value not found".into());
            }
            set_key_value_pair(command[1], command[2]);
            stream.write_all(String::from("Ok\n").as_bytes())?;
        }
        "DEL" => {
            if command.len() < 2 || command[1].trim().is_empty() {
                return Err("Key not found".into());
            }
            match del_key_value_pair(command[1]) {
                Some(val) => {
                    println!("Removed value: {}", val);
                    stream.write_all((val + "\n").as_bytes())?;
                }
                None => {
                    println!("No value found");
                    stream.write_all("(nil)\n".as_bytes())?;
                }
            };
        }
        _ => {
            stream.write_all("Invalid command \n".as_bytes())?;
            return Err("Invalid command".into());
        }
    }
    Ok(())
}

fn set_key_value_pair(key: &str, value: &str) {
    {
        let mut data = SHARED_DATA.lock().unwrap();
        data.insert(key.to_string(), value.to_string());
    }
}

fn get_key_value_pair(key: &str) -> String {
    let data = SHARED_DATA.lock().unwrap();
    let value = match data.get(key) {
        Some(value) => value.clone(),
        None => return String::new(),
    };
    value
}

fn del_key_value_pair(key: &str) -> Option<String> {
    let mut data = SHARED_DATA.lock().unwrap();
    data.remove(key)
}
