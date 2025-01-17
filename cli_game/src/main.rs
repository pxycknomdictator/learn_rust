use rand::Rng;
use std::io;

fn main() {
    // This will generate random number from 1 to 100 inclusive
    let number: u8 = rand::thread_rng().gen_range(1..=100);

    'game_loop: loop {
        println!("Welcome to guessing Game");
        println!("Enter your number");

        let mut input_number = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to get Input");

        // here i use shadowing concept && parsing input string to number
        let input_number: u8 = input_number.trim().parse().expect("Failed to parse");
        println!("Secret Number was: {}\nYou Guess: {}", number, input_number);

        if input_number != number {
            continue;
        }

        if input_number == number {
            break 'game_loop;
        }
    }
}
