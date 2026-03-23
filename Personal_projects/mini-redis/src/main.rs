use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};
mod client;
mod commands;

static SHARED_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
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
