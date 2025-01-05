fn main() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is: {} pixels", calculate_area(width, height));
}


fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}