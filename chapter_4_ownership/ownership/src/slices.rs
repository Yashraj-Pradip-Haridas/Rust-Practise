// slices let you refernces a contiguous sequence of elements within a collection instead of referencing the entire collection
// Slices also do not take the ownership of the underlying data

fn main(){
    let mut s =  String::from("Hello World");
    let word =  first_words(&s);
    // println!("{}", word);
    s.clear(); // makes the string an empty string
    // This will cause the word length 5 even when string is empty so its not tied to each other
}

fn first_words(s: &String)-> usize{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}  


// ---------------------------------
// Using string slices

fn main2(){
    let mut s =  String::from("Hello World");
    // if starting from 0 we can omit the starting value and same goes for end
    let hello =   &s[..5];
    let world =   &s[6..];

    let entire_string = &s[..];
    
    let s2 = "Hello World"; // string liters are string slices and are stored directly into the binary

    let word =  first_words2(&s);// string reference gets automatically coerced to string slice.
    let word2 =  first_words2(&s2)

}

fn first_words2(s: &str)-> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}  



// We can also have slices on different types  of collections
fn collection_slices(){
    let a = [1,2,3,4,5];
    let slice = &a[0..2];
}