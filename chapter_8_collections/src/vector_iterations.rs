fn iterations() {
    let mut v = vec![1, 2, 3, 4];

    // using for loop
    // Taking immutable reference to each element
    // To take mutable reference use &mut v instead of &v
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50; 
        // * is also called as dereference operator
    }
}
