fn compound() {
    // compound types
    let tup = ("Let's get rusty", 100_000); //fixed size array of related data of different data types

    // destructuring a tuple
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    // Tuples as well as arrays both start at index 0
    let error_codes = [404, 500, 200, 301];
    let not_found = error_codes[0];

    let byte = [0; 8]; // creates array with 8 values all set to 0
}
