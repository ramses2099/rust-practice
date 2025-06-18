#![allow(dead_code)]
use std::io;
use std::result::Result;

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let mut numbers = Vec::new();
    let mut trimmed = input.trim().to_string();

    println!("Welcome to the Future Value Calculator");
    println!("Enter numbers (type 'done to finish'):");

    while trimmed.to_lowercase() != "done" {
        match trimmed.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => println!("Invalid input. Please enter a number or 'done'"),
        }

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        trimmed = input.trim().to_string();
    }

    print!("You entered: {:?}", numbers);

    Ok(())
}
