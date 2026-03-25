fn main() {
    println!("Hello, world!");

    // creating a vector in rust
    let mut v: Vec<i32> = Vec::new();
    //pushing values into vector
    v.push(1);
    v.push(2);

    // creating a vector and initializing it with values
    let mut v2 = vec![1, 2, 3];
    // vectors are stored in heap and will be dropped when they go out of scope

    // Accessing elements inside of vector
    // 1. Directly reference an index in the vector
    let third = &v2[2];
    // v2.push(6);
    // Above will give error because
    // cannot borrow `v2` as mutable because it is also borrowed as immutable mutable borrow occurs here
    println!("The third element is : {}", third);
    // Problem with this approach is that we can specify an invalid index
    // Like runtime error - out of bound error
    // But incase of array we will get compile time error
    //

    // 2. using get method - to handle failure/ error gracefully
    match v2.get(3) {
        Some(third) => println!("The third element is : {}", third),
        None => println!("There is no third element"),
    };
}
