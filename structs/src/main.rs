struct User {
    username: String,
    age: i32,
    active: bool,
    jwt_token: String
}

fn main() {
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

}
