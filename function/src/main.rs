fn calculate_and_return(n1: i32, n2: i32) -> f64 {
    return (n2 as f64 / n1 as f64) * 100.0;
}

fn main() {
    println!("Hello Rust Functions");
    let student1: f64 = calculate_and_return(200, 147);
    println!("Student1 percentage is: {}", student1);
    add_two_numbers(90, 18);
}

fn add_two_numbers(num1: i32, num2: i32) {
    let result: i32 = num1 + num2;
    println!("{} + {} = {}", num1, num2, result);
}
