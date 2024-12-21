fn main() {
    // Ownership is the hardest and Final boss concept in Rust lets checkout how much hard is that

    {
        let _s = "Hello world"; // scope start from here
    } // scope end here

    // here values are copied
    let mut student_1: &str = "Registration Form";
    let student_2: &str = student_1;

    student_1 = "School Leaving certificate";

    println!("student 1: {} ", student_1);
    println!("student 2: {} ", student_2);

    // same values are copied
    let mut age: i32 = 20;
    let age2: i32 = age;

    age = 90;

    println!("Age: {}", age);
    println!("Age2: {}", age2);

    // what about special && complex data types like String

    let mut todo: String = String::from("Hello World from, ");

    todo.push_str("Ownership");

    println!("{todo}");

    // here we go we got and error because we move ownership to s2 now s1 is useless
    // let s1: String = String::from("Hi Rust");
    // let s2 = s1;

    // println!("s1 value is: {s1}");

    // let mut s = String::from("hello");
    // s = String::from("ahoy");

    // println!("{s}, world!");

    // expensive way to solve problem
    // let s1: String = String::from("Hello, World");
    // let s2: String = s1.clone();

    // println!("{s1}, {s2}");

    // here is a way to steal ownership

    // let jwt_secret_key: String = String::from("9393939");
    // steal_ownership(jwt_secret_key);
    // println!("{}", jwt_secret_key);
}

// fn steal_ownership(key: String) {
//     println!("{key}");
// }
