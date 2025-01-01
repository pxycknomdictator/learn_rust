// structs are used to create custom data types in Rust that can help us to create multiple instances of a struct with different values. just like classes in other programming languages. This is look like interface in TypeScript and create multiple instances  with different values.

struct User {
    username: String,
    age: i32,
    active: bool,
    jwt_token: String
}

fn main() {
    // Custom data types han? remind me of interfaces in TypeScript :D
    let first_user: User = User {
        jwt_token: String::from("9922iks9o293901"),
        age: 92,
        username: String::from("Noman"),
        active: true 
    };

    print!("{} \n", first_user.active);
    print!("{} \n", first_user.username);
    print!("{} \n", first_user.age);
    print!("{} \n", first_user.jwt_token);

    let second_user: User = User {
        age: 92,
        jwt_token: String::from("992kd82992929"),
        username: String::from("John Doe"),
        active: false
    };

    print!("{} \n", second_user.active);
    print!("{} \n", second_user.username);
    print!("{} \n", second_user.age);
    print!("{} \n", second_user.jwt_token);

}
