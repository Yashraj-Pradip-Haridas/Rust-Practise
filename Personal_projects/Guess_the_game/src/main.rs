use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("--------------------------------------------");
    println!("--------Welcome to the guessing game--------");
    println!("Guess the number between 0 and 100");
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..101);
    loop {
        let mut input = String::new();
        println!("Enter your number: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        if input.trim() == "exit" {
            break;
        }

        let int_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid input".red());
                continue;
            }
        };

        match random_number.cmp(&int_input) {
            Ordering::Equal => {
                println!("{}", "Hooray!! You won the game".green());
                println!(
                    "The hidden number was: {}",
                    random_number.to_string().green()
                );
                break;
            }
            Ordering::Greater => {
                println!("{}", "The guessed number is smaller".red());
            }
            Ordering::Less => {
                println!("{}", "The guessed number is greater".red());
            }
        }
    }
    print!("{}", "Thank you for playing the game!! Se ya soon".yellow());
}
