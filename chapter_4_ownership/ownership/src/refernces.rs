// --------------------------
// using variables without taking ownership - use references

use std::string;


fn main(){
    let s1 = String::from("Hello");
    let (s2, length) = calculate_length(s1);
    println!("The lenght of {} is {}", s2, length);
}

fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s, length)
}


// updated code using referneces
fn main2(){
    let s1 = String::from("Hello");
    // passing a reference to a string
    let length = calculate_length2(&s1);
    println!("The lenght of {} is {}", s1, length);
}

// References dont take the ownerships of the underlying values.
// passing in refernces as function parameters is called as borrowing
// References are immutable by default

// Now to pass as refernce just add & infront of the variable
fn calculate_length2(s: &String)-> usize {
    // s.push_str("Oops"); This will give an error
    let length = s.len();  // returns the length of a string
   length
}


// modifying values of a references
// To do this the string should  be mutable and a mutable refernce and mutable parameter should be provided
// You can only have one mutable refernce to a particular piece of data in a particular scope
fn main3(){
    let mut s1 = String::from("Hello");
    cal_length(&mut s1);
}

fn cal_length(s:&mut String){
    s.push_str(" World");
    let length = s.len();
}


// --------------------------------------------------------------
// You can only have one mutable refernce to a particular piece of data in a particular scope
fn main4(){
    let mut s = String::from("Hello");

    let r1 = &mut s;
    // let r2 = &mut s; This will give error
    // cannot borrow `s` as mutable more than once at a time second mutable borrow occurs here
    // This prevents data races at compile time
    // A data race occurs if we have two pointers, pointing to same piece of data and one of those pointers is used to write to the data and there is no mechanism to synchronise data access between the two pointers

    // println!("{},{}", r1, r2);
}
// Alternative for this - switch to immutable
fn main4_1(){
        let s = String::from("Hello");
        let r1 = & s;
        let r2 = & s;
        println!("{},{}", r1, r2);
}

// mixing immutable and mutable
fn main4_2(){
        let mut s = String::from("Hello");
        let r1 = & s; // scope of refernce starts with its introduction
        let r2 = & s;
        // let r3 = &mut s;
        // cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here

        println!("{},{}", r1, r2);  //scope of reference ends with its last usage
        let r3 = &mut s;  // using this will now be completely fine since r1 and r2 are out of scope
}