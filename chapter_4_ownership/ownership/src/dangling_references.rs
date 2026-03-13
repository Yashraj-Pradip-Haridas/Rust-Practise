// Refernces that points to invalid data - dangling references
fn main(){

    /*
    Rules of refernces
    1. At any given time, you can have either one mutable reference or any number of immutable refernces.
    2. References must always be valid.
    */
    let refernce_to_nothing = dangle();
}

fn dangle()-> &String{
    let s =  String::from("Hello");
    &s //As the function finishes the rust will deallocate the string and hence it will return dangling pointer
}
