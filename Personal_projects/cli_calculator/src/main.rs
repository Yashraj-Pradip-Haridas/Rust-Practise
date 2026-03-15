use colored::Colorize;
use std::io;
fn main() {
    loop {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut operation = String::new();
        let mut exit_op = String::new();
        println!("{}", "Enter the first operand: ".blue());
        io::stdin()
            .read_line(&mut input1)
            .expect("Failed to read input 1");

        let input1: i32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number.".red());
                continue;
            }
        };

        println!("{}", "Enter the second operand: ".blue());
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read input 2");

        let input2: i32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number.".red());
                continue;
            }
        };

        println!(
            "{}",
            "Enter the choice to perform operation: (+,-,*,/)".yellow()
        );
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read operator");

        let operation = operation.trim().chars().next();

        let output = match operation {
            Some('+') => add(input1, input2),
            Some('-') => subtract(input1, input2),
            Some('*') => multiply(input1, input2),
            Some('/') => divide(input1, input2),
            None => todo!(),
            _ => error(),
        };
        println!("The output is {}", output.to_string().green());

        println!("{}", "Press n/N to exit else continue:".red());
        io::stdin()
            .read_line(&mut exit_op)
            .expect("Failed to read input 1");

        let exit_op = exit_op.trim().chars().next();
        match exit_op {
            Some('n') => break,
            Some('N') => break,
            None => continue,
            _ => continue,
        }
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn divide(a: i32, b: i32) -> f64 {
    if b != 0 {
        (a / b).into()
    } else {
        println!("The denominator cannot be zero");
        0
    }
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn error() -> i32 {
    println!("Invalid input");
    0
}

// --------------------------------- easier version
// fn calculate(a: f64, b: f64, op: char) -> Option<f64> {
//     match op {
//         '+' => Some(a + b),
//         '-' => Some(a - b),
//         '*' => Some(a * b),
//         '/' => {
//             if b == 0.0 {
//                 println!("Cannot divide by zero");
//                 None
//             } else {
//                 Some(a / b)
//             }
//         }
//         _ => {
//             println!("Invalid operator");
//             None
//         }
//     }
// }
