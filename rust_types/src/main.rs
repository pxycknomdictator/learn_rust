fn main() {
    // There are different types of data types in RUST

    // string literals
    let username: &str = "John Doe";

    /*
        Numbers (Signed && Unsigned)
        signed for positive and negative integers
        unsigned means only positive integers
    */
    let age: u32 = 90;
    let street: i32 = -90;
    let max_number: usize = 909999999999999;

    println!("{}, {}, {}, {}", username, age, street, max_number);

}
