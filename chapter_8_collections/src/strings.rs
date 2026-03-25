use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;
fn strings() {
    // Strings are stored as collection of UTF-8 encoded bytes in rust so strings can be written in any other languages like hindi or chinese or marathi
    // creating new strings
    let s1 = String::new();
    let s2 = "hello world";
    let s3 = s2.to_string();
    let s4 = String::from("Hello world");

    // Appending to strings - just like vector string can shrink or grow in size
    let mut s = String::from("foo");
    s.push_str("Hello"); // use this to avoid taking ownership of the string
    s.push('a');

    let s5 = s1 + &s3; // adding the strings using the + operator 
    // In the above the ownership of s1 is moved to s5 and hence it cannot be used later

    // using the format macro to avoid taking the ownership
    let s6 = format!("{}{}", s5, s2);

    // Indexing into a string
    let test = String::from("Hello");

    // UTF8 possible represenatations
    /*
    1. using bytes => [224, 1,2,34,432,34]

    2. using scalar values => [न म स  ्् त े  ््]

    3. using Grapheme clusters => [न म स् ते]
     */

    // Rust does not support indexing like high level languages

    let hello = String::from("नमस्ते");

    for b in hello.bytes() {
        println!("{}", b);
    }

    for b in hello.chars() {
        println!("{}", b);
    }

    // The ability to iterate over grapheme clusters is not provided by default by rust
    // use the crate unicode-segmentation
    for b in hello.graphemes(true) {
        println!("{}", b);
    }
}
