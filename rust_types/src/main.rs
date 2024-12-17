fn main() {
    // There are different types of data types in RUST

    // string literals
    let username: &str = "John Doe";

    println!("My name is: {}", username);

    /*
        Numbers (Signed && Unsigned)
        signed for positive and negative integers
        unsigned means only positive integers
    */
    let age: u32 = 90;
    let street: i32 = -90;
    let max_number: usize = 909999999999999;

    println!(
        "My age is: {} and street number: {} and longest number {}",
        age, street, max_number
    );
    // floats values

    const PI: f32 = 3.14;
    let my_float_number: f64 = 90.3884939484843;
    let floating_number: f32 = 83.29894;

    println!(
        "The value of PI is: {}, longest floating number {}, and smallest floating number {}",
        PI, my_float_number, floating_number
    );

    // booleans aka bool in rust

    let is_programmer: bool = true;
    println!("I am a programmer ? {}", is_programmer);

}
