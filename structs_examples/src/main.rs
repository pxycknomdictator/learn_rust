fn main() {
    // # Here is the updated and clear version of calculating area by tuple
    let rectangle: (i32, i32) = (30, 50);
    println!("The area of the rectangle is: {} pixels", calculate_area(rectangle));
}

fn calculate_area(dimension: (i32, i32)) -> i32 {
    dimension.0 * dimension.1
}