fn main() {
    let number = 69;
    let input_number = 69;

    'game_loop: loop {
        println!("Welcome to guessing Game");
        println!("Guess the number");

        if number != input_number {
            continue;
        }

        if number == input_number {
            break 'game_loop;
        }
    }

    println!("You Guess {} which is correct :)", input_number);
}
