use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
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
        // let data_output;
        // {
        //     let output = connections_handle.lock().unwrap();
        //     data_output = &output.iter().clone();
        // }
        // 1. Lock and create a list of clones manually
        let data_output: Vec<TcpStream> = {
            let output = connections_handle.lock().unwrap();
            output
                .iter()
                .filter_map(|s| s.try_clone().ok()) // Safely clone each stream
                .collect()
        };
        let address = stream.peer_addr()?;
        // println!("Mutex value is : {:?}", *output)
        for mut conn in data_output {
            if let Ok(recipient_addr) = conn.peer_addr() {
                if recipient_addr != address {
                    let _ = conn.write_all(response.as_bytes());
                }
            }
        }
    }
    println!("{}", "Client Disconnected");
    {
        let mut vec_conns = connections_handle.lock().unwrap();

        // Get the target address once to avoid repeated syscalls in the loop
        if let Ok(target_addr) = stream.peer_addr() {
            if let Some(index) = vec_conns.iter().position(|x| {
                x.peer_addr()
                    .map(|addr| addr == target_addr)
                    .unwrap_or(false)
            }) {
                // Use swap_remove for O(1) performance if order is not critical
                vec_conns.swap_remove(index);
            }
        }

        Ok(())
    }
}
