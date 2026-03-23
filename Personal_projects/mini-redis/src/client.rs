use crate::commands;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};
pub fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = BufReader::new(&stream);
    // let address = stream.peer_addr()?;
    loop {
        let mut body_buffer = String::new();
        let data = buff.read_line(&mut body_buffer)?;
        // println!("Entered thread {}", address);
        // println!("{}: {}", address, body_buffer.trim());
        if data == 0 {
            break;
        }
        if body_buffer.trim() == "EXIT".to_string() {
            break;
        }

        // println!("{:?}", command);

        match commands::handle_commands(body_buffer, &stream) {
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
