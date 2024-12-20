fn main() {
    println!("Hello Rust Functions");
    add_two_numbers(90, 18);
}

fn add_two_numbers(num1: i32, num2: i32) {
    let result: i32 = num1 + num2;
    println!("{} + {} = {}", num1, num2, result);
}
