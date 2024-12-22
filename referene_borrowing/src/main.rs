fn main() {
    // so lets talk about what is Borrowing or references in RUST

    /*
        what is going on here?
        First i give ownership to calculate_length and he calculate the length or name and return a length and name ownership back
    */

    // let name: String = String::from("John Doe");
    // let (name, length): (String, usize) = calculate_length(name);
    // println!("My name is: {} and my name length is: {}", name, length);

    // =========================================================================================================

    // when i put & symbol thats mean i give you a reference (borrow this immutable variable and read only) this make our life easier
    let name: String = String::from("John Doe");
    let length: usize = calculate_length(&name);
    println!("My name is: {} and my name length is: {}", name, length);
}

fn calculate_length(name: &String) -> usize {
    let length: usize = name.len();
    length
}

// fn calculate_length(name: String) -> (String, usize) {
//     let length: usize = name.len();
//     (name, length)
// }
