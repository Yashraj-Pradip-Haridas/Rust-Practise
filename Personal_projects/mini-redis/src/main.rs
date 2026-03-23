use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::storage::create_file;
mod client;
mod commands;
mod restore_state;
mod storage;

static SHARED_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    match restore_state::get_state("data.txt".to_string()) {
        Ok(_) => {
            println!("File created successfully");
        }
        Err(_) => {
            println!("No previous backup found");
            create_file("data.txt".to_string())?;
        }
    };
    for stream in listener.incoming() {
        let stream = stream?;
        let address = stream.peer_addr()?;
        thread::spawn(move || {
            println!("Connection {} Established", address);

            let _ = client::handle_client(stream);
        });
    }

    Ok(())
}
