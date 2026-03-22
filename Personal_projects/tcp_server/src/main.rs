use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            println!("Connection Established");
            let _ = handle_client(stream);
        });
    }
}

fn handle_client(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buff = BufReader::new(stream);

    loop {
        let mut body_buffer = String::new();
        let data = buff.read_line(&mut body_buffer)?;
        println!("{}", body_buffer.trim());
        if data == 0 {
            break;
        }
    }
    println!("{}", "Client Disconnected");
    Ok(())
}
