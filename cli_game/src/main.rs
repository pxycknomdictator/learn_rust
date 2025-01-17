use std::io;

fn main() {
    let mut input_number = String::new();
    let number = 9;

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
