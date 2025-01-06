// // # Here is the most cleaner and updated version of creating rectangle with structs

// #[derive(Debug)] // this is line is help us to debug struct values
// struct Rectangle {
//     width: i32,
//     height: i32,
// }

// fn main() {
//     // creating rectangle instance

//     let rectangle: Rectangle = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is: {} pixels",
//         calculate_area(&rectangle)
//     );

//     // println!("{}", rectangle); // This line does not work!
//     println!("{rectangle:?}"); // In this way we use pretty way to display
// }

// fn calculate_area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }

// let's learn about method syntax
struct Rectangle {
    width: u32,
    height: u32,
}

// so what the hell i am doing here? as a javascript dev i think we are creating a methods for Rectangle structs and here self just like (this) keyword in JS

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 89,
        height: 49,
    };

    println!("area is: {}", rec1.calculate_area());
}