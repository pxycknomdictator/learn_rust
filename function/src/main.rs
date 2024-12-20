const CELSIUS: i32 = 38;

fn calculate_and_return(n1: i32, n2: i32) -> f64 {
    return (n2 as f64 / n1 as f64) * 100.0;
}

fn main() {
    println!("Hello Rust Functions");
    let student1: f64 = calculate_and_return(200, 147);
    println!("Student1 percentage is: {}", student1);
    add_two_numbers(90, 18);

    let temp: f64 = celsius_to_fahrenheit(CELSIUS);
    println!("{} Celsius to Fahrenheit is: {}", CELSIUS, temp.round());

    let temp2: i32 = fahrenheit_to_celsius(temp);
    println!("{} Fahrenheit to Celsius is: {}", temp, temp2);
}

fn add_two_numbers(num1: i32, num2: i32) {
    let result: i32 = num1 + num2;
    println!("{} + {} = {}", num1, num2, result);
}

fn celsius_to_fahrenheit(celsius: i32) -> f64 {
    1.8 * celsius as f64 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> i32 {
    ((fahrenheit - 32.0) * (5.0 / 9.0)) as i32
}