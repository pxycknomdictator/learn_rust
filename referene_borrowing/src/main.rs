fn main() {
    // so lets talk about what is Borrowing or references in RUST

    /*
        what is going on here?
        First i give ownership to calculate_length and he calculate the length or name and return a length and name ownership back
    */

    // let name: String = String::from("John Doe");
    // let (name, length): (String, usize) = calculate_length(name);
    // println!("My name is: {} and my name length is: {}", name, length);

    // =========================================================================================================

    // when i put & symbol thats mean i give you a reference (borrow this immutable variable and read only) this make our life easier
    let name: String = String::from("John Doe");
    let length: usize = calculate_length(&name);
    println!("My name is: {} and my name length is: {}", name, length);

    // =========================================================================

    // What happened you want that a fn mutate your variable also what should we do in those scenario ?
    let mut my_car = String::from("hello");
    modify_car(&mut my_car);
    println!("{}", my_car);

    // =====================================================

    // we cannot give mutable reference more that one time until we not use one

    // Invalid
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Valid
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    println!("{}", r2);

    // This is a mutable reference but this variable in block scope
    {
        let r1 = &mut s;
        println!("Inside the scope {}", r1);
    }

    let r2 = &mut s;
    println!("Outside the scope {}", r2);

    // ========================================

    // This the valid because we are using immutable variable on first
    let mut student_name = String::from("John Doe");

    let r1 = &student_name;
    let r2 = &student_name;
    println!("{} {}", r1, r2);

    let r3 = &mut student_name;
    println!("{}", r3);

    /*
        There is the big problem

        let mut student_name = String::from("John Doe");

        let r1 = &student_name;
        let r2 = &student_name;
        let r3 = &mut student_name; // big problem is here because r1 and r2 get his immutable reference but we don't know r3 will be change on future than r1 and r2 calculation are failed or invalid

        println!("{} {} {}", r1, r2, r3);
    */
}

fn calculate_length(name: &String) -> usize {
    let length: usize = name.len();
    length
}

fn modify_car(car: &mut String) {
    car.push_str(" Color change to gray");
}

// fn calculate_length(name: String) -> (String, usize) {
//     let length: usize = name.len();
//     (name, length)
// }
