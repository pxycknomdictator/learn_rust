// # Here is the most cleaner and updated version of creating rectangle with structs
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    // creating rectangle instance

    let rectangle: Rectangle = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is: {} pixels", calculate_area(&rectangle)); // Giving a reference or borrow to calculate_area because we don't want to loose ownership
}

fn calculate_area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}