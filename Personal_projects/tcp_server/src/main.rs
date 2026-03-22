use std::{
    io::{BufRead, BufReader, Write},
    mem,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, MutexGuard},
    thread,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let connections = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
    for stream in listener.incoming() {
        let stream = stream?;
        let stream_clone = stream.try_clone()?;
        let address = stream.peer_addr()?;
        let connections_handle = Arc::clone(&connections);
        thread::spawn(move || {
            println!("Connection {} Established", address);
            {
                let mut conn_vec = connections_handle.lock().unwrap();
                conn_vec.push(stream_clone);
            }
            let _ = handle_client(stream, &connections_handle);
        });
    }

    Ok(())
}

fn handle_client(
    stream: TcpStream,
    connections_handle: &Arc<Mutex<Vec<TcpStream>>>,
) -> Result<(), Box<dyn std::error::Error>> {
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
        let response = address.to_string() + " :" + &body_buffer;
        let output = connections_handle.lock().unwrap();
        // println!("Mutex value is : {:?}", *output)
        for mut i in &*output {
            if i.peer_addr()? != stream.peer_addr()? {
                let _ = i.write_all(response.as_bytes());
            }
        }
    }
    println!("{}", "Client Disconnected");
    Ok(())
}
