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

// # IMPORTANT we can give any name to a function also struct fields name like with
impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    // self means i can access struct fields and i can pass custom parameters to fn :D

    // no way i can use &Self to mention that use self struct impl oh my god
    fn can_hold(&self, custom_dimension: &Self) -> bool {
        self.width > custom_dimension.width && self.height > custom_dimension.height
    }
}

struct NumberStruct {
    number: i32,
    divided_number: i32,
}

impl NumberStruct {
    fn is_even(&self) -> bool {
        if self.number % self.divided_number == 2 {
            return true;
        }
        false
    }

    fn increment(&mut self) -> i32 {
        self.number += 1;
        self.number
    }

    fn decrement(&mut self) -> i32 {
        self.number -= 1;
        self.number
    }

    fn multiply(&mut self) -> i32 {
        self.number *= 2;
        self.number
    }
}

// # Rust uses struct for data and impl for methods, with self as the instance reference, unlike JavaScript which uses class and this.

fn main() {
    let rec1 = Rectangle {
        width: 89,
        height: 49,
    };

    let rec2 = Rectangle {
        width: 0,
        height: 3,
    };

    println!("area is: {}", rec1.calculate_area());
    println!("Can Hold? : {}", rec1.can_hold(&rec2));

    let mut num1 = NumberStruct {
        number: 90,
        divided_number: 2,
    };

    println!("Is Even ? {}", num1.is_even());

    println!("Increment {}", num1.increment());
    println!("Decrement {}", num1.decrement());

    println!("Multiplication {}", num1.multiply());
}
