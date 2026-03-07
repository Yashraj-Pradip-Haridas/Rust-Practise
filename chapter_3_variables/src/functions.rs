fn functions_in_rust() {
    // Defined using fn keyword
    // Rust uses snake case convention for naming functions.
    // function name all lower case and if space then use '_'.

    let result = add(3, 5);
    println!("{}", result);
}
// statemets perform actions but doesn't return values
// Whereas expressions returns a value

fn add(a: i32, b: i32) -> i32 {
    a + b
    // if there is single value to return no need to write return keyword and ;
}
