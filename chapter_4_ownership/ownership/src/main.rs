mod other;
mod ownership;
mod refernces;
mod dangling_references;
mod slices;
fn main() {
    println!("Hello, world!");
    // --------ownership rules---------
    /*
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    {
        // s is not valid here, it's not yet declared
        let s = String::from("Hello World!!"); // s is valid from this point onward
        // do stuff with s (The string is now stored in heap)
    } // this scope is now over, and s is no  longer valid - rusts deallocates automatically
}
