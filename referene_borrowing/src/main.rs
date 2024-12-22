fn main() {
    // so lets talk about what is Borrowing or references in RUST

    /*
        what is going on here?
        First i give ownership to calculate_length and he calculate the length or name and return a length and name ownership back
    */

    let name: String = String::from("John Doe");
    let (name, length): (String, usize) = calculate_length(name);
    println!("My name is: {} and my name length is: {}", name, length);
}

fn calculate_length(name: String) -> (String, usize) {
    let length: usize = name.len();
    (name, length)
}
