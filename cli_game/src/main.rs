use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // This will generate random number from 1 to 100 inclusive
    let number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to guessing Game");
    println!("Enter your number");

    loop {
        let mut input_number = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to get Input");

        // here i use shadowing concept && parsing input string to number and handling error when user enter a string
        let input_number: u8 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let output: &str = match input_number.cmp(&number) {
            Ordering::Less => "To Small!",
            Ordering::Greater => "To Big!",
            Ordering::Equal => {
                println!("You Win! The number is: {}", number);
                break;
            }
        };

        if input_number != number {
            println!("{}", output);
            continue;
        }
    }
}
