// # Here is the most cleaner and updated version of creating rectangle with structs

#[derive(Debug)] // this is line is help us to debug struct values
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    // creating rectangle instance

    let rectangle: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is: {} pixels",
        calculate_area(&rectangle)
    );

    // println!("{}", rectangle); // This line does not work!
    println!("{rectangle:?}"); // In this way we use pretty way to display
}

fn calculate_area(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.height
}
