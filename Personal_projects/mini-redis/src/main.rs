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

        let command: Vec<&str> = body_buffer.split_whitespace().collect();
        // println!("{:?}", command);

        let _ = handle_commands(command, &stream);
    }
    println!("{}", "Client Disconnected");

    Ok(())
}

fn handle_commands(
    command: Vec<&str>,
    mut stream: &TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    match command[0].trim() {
        "GET" => {
            println!("Entered the get function");
            let out_value = get_key_value_pair(command[1]);
            println!(" Value: {}", out_value);
            let response = out_value + "\n";
            stream.write_all(response.as_bytes())?;
        }
        "SET" => {
            println!("Entered the set function");
            set_key_value_pair(command[1], command[2]);
        }
        _ => {}
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
