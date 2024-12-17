fn main() {
    /*
        There are TWO major types of data types in rust
        1 -> Scalar
        2 -> Compounds
    */

    // 1 -> Scalar
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

    let is_iam_moron: bool = false;
    println!("I am moron ? {}", is_iam_moron);

    // characters aka char in rust
    let animal: char = '🦀';
    println!("My Favorite animal is: {}", animal);

    let character: char = 'N';
    println!("My name start with: {}", character);

    // 2 -> Compounds
    // There are 2 types of compound types in RUST

    // Tuples
    let programmers: (&str, &str, &str) = ("John Doe", "Jane Doe", "Kevin Doe");
    println!("Tuples {:?}", programmers);

    // I can access single values

    println!("My name is: {}", programmers.0);
    println!("My name is: {}", programmers.1);
    println!("My name is: {}", programmers.2);

    // I can destructure values from tuple like JavaScript

    let (first, second, third) = programmers;
    println!("{}, {}, {}", first, second, third);

    // I can use different data types in tuples
    let another_tuple: (i32, f64, &str, bool) = (90, 90.4, "John Doe", true);
    println!("{:?}", another_tuple);

    // Arrays
    const DAYS: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    println!("Today is: {}", DAYS[1]);

    // To create array with same number with multiple times you can use this O_O
    let some_numbers: [i32; 10] = [4; 10];
    println!("{:?}", some_numbers);
}
