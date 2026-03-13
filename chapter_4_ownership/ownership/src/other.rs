fn other(){
    let x = 32;
    let y = x; //copy not a move
    

    let s1 = String::from("Hello world");
    let s2 = s1;

    // println!("{}", s1); This wont run will give error
    // compile time error - since s1 is already moved to s2

    // To create copy use
    let s3 = s2.clone(); // cannot use s1 since borrow moved to s2
}