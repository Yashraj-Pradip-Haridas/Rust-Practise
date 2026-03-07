use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // High end is exclusive whereas low end is inclusive
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "exit"{
            print!("Gracefully quitting the game\n");
            break;
        }

        // This is called shadowing 
        // In rust using let keyword we can reassign the variable
        // let guess: u32 = guess.trim().parse().expect("Please input a number");
                let guess: u32 = match guess.trim().parse(){
                    Ok(num)=>num,
                    Err(_)=>{
                        println!("{}", "Please input a valid number".red());
                        continue;
                    }
                };
                    

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","Congrats!! You won".green());
                break;
            },
            Ordering::Greater => println!("{}", "Guessed number is greater".blue()),
            Ordering::Less => println!("{}","Guessed number is smaller".blue()),
        }

        println!("You guessed: {}", guess);
    }
    println!("The Secret number is: {}", secret_number);
}
