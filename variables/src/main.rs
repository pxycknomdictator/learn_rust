fn main() {
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

}
