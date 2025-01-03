// structs are used to create custom data types in Rust that can help us to create multiple instances of a struct with different values. just like classes in other programming languages. This is look like interface in TypeScript and create multiple instances  with different values.

struct User {
    username: String,
    age: i32,
    active: bool,
    jwt_token: String,
}

struct Employee {
    name: String,
    age: u8,
    address: String,
    email : String
}

struct Student {
    name: String,
    age: i32,
    is_student: bool,
}

fn main() {
    // Custom data types han? remind me of interfaces in TypeScript :D
    let first_user: User = User {
        jwt_token: String::from("9922iks9o293901"),
        age: 92,
        username: String::from("Noman"),
        active: true,
    };

    print!("{} \n", first_user.active);
    print!("{} \n", first_user.username);
    print!("{} \n", first_user.age);
    print!("{} \n", first_user.jwt_token);

    let second_user: User = User {
        age: 92,
        jwt_token: String::from("992kd82992929"),
        username: String::from("John Doe"),
        active: false,
    };

    print!("{} \n", second_user.active);
    print!("{} \n", second_user.username);
    print!("{} \n", second_user.age);
    print!("{} \n", second_user.jwt_token);

    // We can also change the values of the struct after creating the instance, but we can't mutate a specific field of a struct instance unless we make the entire instance mutable.

    let mut third_user: User = User {
        age: 29,
        jwt_token: String::from("992kd82992929d92020e"),
        username: String::from("Jane Doe"),
        active: true,
    };

    third_user.age = 18;

    print!("{} \n", third_user.active);
    print!("{} \n", third_user.username);
    print!("{} \n", third_user.age);
    print!("{} \n", third_user.jwt_token);

    let student_1: Student = generate_struct(String::from("Noman"), 69, true);
    print!(
        "My name is: {}\nMy age is: {}\nand I am a student {}",
        student_1.name, student_1.age, student_1.is_student
    );

    /* As you can see, I created a `Student` struct instance as mutable, passing ownership.
    This allows me to change the student's name or other fields later in the code. */

    let mut last_student: Student = Student {
        age: 9292,
        is_student: true,
        name: String::from("John Doe")
    };

    // I know this is exploding your mind, but this is how Rust works. It's a bit different from other programming languages.
    let name_ref = last_student.name;
    last_student.name = String::from("Jane Doe");

    println!("\nReference Name is: {}", name_ref);
    println!("\nChanged Name is: {}", last_student.name);



    // lets suppose we have 2 structs and both structs some values are same

    let employee1: Employee = Employee {
        name: String::from("Noman"),
        address: String::from("Karachi, Pakistan"),
        age: 20,
        email: String::from("noman@gmail.com")
    };

    // most pain full way to do that
    let employee2: Employee = Employee {
        name: employee1.name,
        address: employee1.address,
        age: employee1.age,
        email: String::from("noman@gmail.com")
    };

    println!("{}", employee2.address);

    // as you can see employee3 and employee4 are same but we can use the spread operator to copy the values of employee3 to employee4 by the help of struct update syntax 

    let employee3: Employee = Employee {
        name: String::from("John"),
        address: String::from("NewYork, USA"),
        age: 30,
        email: String::from("john@gmail.com")
    };

    let employee4: Employee = Employee {
        email: String::from("john123@gmail.com"),
        ..employee3
    };

    print!("{}", employee4.email);
    print!("{}", employee4.age);

}


// As you can see this function create a struct with different arguments pass by user
fn generate_struct(name: String, age: i32, is_student: bool) -> Student {
    Student {
        age,
        is_student,
        name,
    }
}
