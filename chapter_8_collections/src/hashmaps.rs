use std::{collections::HashMap, hash::Hash};

fn hashmps() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // moves the ownership of the strings into the hashmaps
    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let score = scores.get(&String::from("blue")); // we need to pass the key as reference

    // iterating over elements in hashmap
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    // The following syntax will overwrite the value
    scores.insert(String::from("yellow"), 20);
    scores.insert(String::from("yellow"), 30);

    // To avoid this use the following syntax - inserts the entry only if no previous entry is found
    scores.entry(String::from("yellow")).or_insert(32);
    scores.entry(String::from("yellow")).or_insert(35);

    // updating a hashmap
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
