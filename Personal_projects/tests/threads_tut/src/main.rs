use std::thread::{self, Thread};
use std::time::Duration;
fn main() {
    let mut handles = Vec::new();

    for i in 1..3 {
        // spawn a new thread

        let handle = thread::spawn(move || {
            println!("Inside thread {}", i);
            thread::sleep(Duration::from_secs(1));
            for j in 1..11 {
                println!("{:>3}", i * j);
            }
        });

        handles.push(handle);
    }

    // wait for all threads to complete by calling join() on their handles
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Exiting main thread");
}
