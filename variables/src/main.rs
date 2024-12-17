/*
    you cant define variables outside the main fn
    let global_scope_variable: &str = "Global";
*/

/*
    constants only declare in UPPER_CASE + SNAKE_CASE only other wise ready for errors and warning
*/
const JWT_SECRET_KEY: &str = "939393939";

fn main() {
    println!("Constant: {}", JWT_SECRET_KEY);
    // First of all rust is static type language so you can't do any stupidity like JavaScript

    let username: &str = "John Doe";
    println!("Hello, {}", username);

    /*
        By default in rust all variables are immutable its mean you cant change it
        to mutate this variable i use mut thats mean rust allow me to change my variable
    */

    let mut age: u8 = 18;
    age += 1;
    age += 1;

    println!("My Age is: {}", age);

    let jwt_secret_key: &str = "93939392049298483838";
    println!("My Secret token is {}", jwt_secret_key);

    // This is not possible in rust because let is immutable variable so i cant change it without mut keyword

    // jwt_secret_key = "99999999999"

    let is_admin: bool = true;
    println!("Are you admin ? {}", is_admin);

    let negative_number: i32 = -122;
    println!("Static Negative number: {}", negative_number);

    const PI: f64 = 3.14;
    println!("The value of PI is: {}", PI);

    let my_heart_4_you: char = '🦀';
    println!("I like you crab!: {}", my_heart_4_you);
}
