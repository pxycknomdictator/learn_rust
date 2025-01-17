use std::io;
use rand::Rng;

fn main() {
    // This will generate random number from 1 to 100 inclusive

    let number = rand::thread_rng().gen_range(1..=100);
    let mut input_number = String::new();

    println!("Welcome to guessing Game");
    println!("Enter your number");

    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to get Input");

    if input_number.trim() == number.to_string() {
        println!("Correct Number Guess {}", input_number.trim());
    } else {
        println!("Wrong Number Guess {}", input_number.trim());
    }
}
