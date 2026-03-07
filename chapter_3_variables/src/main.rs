mod data_types;
mod functions;
mod control_flow;
mod loops;
fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; - This will give an error since varibles in rust are immutable

    let mut y = 10;
    println!("The value of y is {}", y);
    y = 20;
    println!("The updated value of y is {}", y);

    // created const variables in rust - common practise to have variable name all uppercase
    // const variables cannot be mutable and they needs to be type annotated
    // rust allows proper representation of large numbers
    const SUB_COUNT: u32 = 100_000;
    println!("{}", SUB_COUNT);

    // shadowing - creating new variables using existing names
    let x = "Hello World";

    // Scalar datatypes and compond data types in rust
}
