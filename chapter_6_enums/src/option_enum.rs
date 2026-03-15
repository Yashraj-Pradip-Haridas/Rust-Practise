// There are no null values in Rust
fn main() {
    // Standard structure of option enum
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // examples
    let some_number = Some(54);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Needs annotation

    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn some_fn() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // gives error - cannot add `Option<i8>` to `i8` (compile time error)

    // use y if value else some default value
    let sum = x + y.unwrap_or(0); // will run even if y is none
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ..
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
        Coin::Penny => 1,
    }
}

// -------------------------- combining match expression with option enum

// function usage

fn use_Plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match expressions are exhaustive and we have to match all the possible values
    match x {
        Some(i) => Some(i + 1), // since return type is option we need to wrap output in some
        _ => None,              // or you can also use none if required
    }
}

// if else syntax
fn main2() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    // if else syntax
    if let Some(3) = some_value {
        println!("Three");
    }
}
