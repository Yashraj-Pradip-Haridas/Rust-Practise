fn ownership(){
    let s = String::from("Hello World");
    takes_ownership(s);
    // println!("{}",s); This will give error because s is borrowed by the function just like assigning s to other variable
    // To run this function needs to return the string
}

// This does not apply for scalar datatypes, they are copied and not moved

fn takes_ownership(some_string: String)->String{
    println!("{}", some_string);
    // After this scope some_string gets dropped
    some_string
}
// -------------------------------------------------------------

fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("Hellow");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn gives_ownership()->String {
    let some_string = String::from("Hellow");
    some_string
}
fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// --------------------------
// using variables without taking ownership - use references
