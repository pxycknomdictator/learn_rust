#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum IpAddressTypes {
    V4(String),
    V6(String),
}

fn main() {
    let color1 = Color::Blue;
    let color2 = Color::Red;
    let color3 = Color::Green;

    println!("Your choice is: {:?}", color1);
    println!("Your choice is: {:?}", color2);
    println!("Your choice is: {:?}", color3);

    let version4 = IpAddressTypes::V4(String::from("192.168.1.1"));
    let version6 = IpAddressTypes::V6(String::from("::1"));

    // just like switch statement but its not so easy to read and understand
    match version4 {
        IpAddressTypes::V4(addr) => println!("Ip version is: {}", addr),
        IpAddressTypes::V6(addr) => println!("Ip version is: {}", addr),
    }

    match version6 {
        IpAddressTypes::V4(addr) => println!("{}", addr),
        IpAddressTypes::V6(addr) => println!("{}", addr),
    }

    #[derive(Debug)]
    struct User {
        name: String,
        status: Status,
    }

    #[derive(Debug)]
    enum Status {
        Active,
        InActive,
    }

    impl User {
        fn check_status(&self) {
            let result: &str = match self.status {
                Status::Active => "Active",
                Status::InActive => "Inactive",
            };
            println!("Name: {}, Status: {}", self.name, result);
        }
    }

    /* here i created two users and set name and status will be active or inactive with Status enum
    than i call check_status method where i am simply matching self field comparison with match just like switch case btw */

    let user1 = User {
        name: String::from("Noman"),
        status: Status::Active,
    };

    let user2: User = User {
        name: String::from("John"),
        status: Status::InActive,
    };

    user1.check_status();
    user2.check_status();

    #[derive(Debug)]
    enum ProfileStatus {
        ACTIVE,
        INACTIVE
    }

    #[derive(Debug)]
    struct Profile {
        name: String,
        message: String,
        status: ProfileStatus
    }

    impl Profile {
        fn check_status(&self) {
            let user_information: &str = match self.status {
                ProfileStatus::ACTIVE => "Active",
                ProfileStatus::INACTIVE => "InActive"
            };
            println!("My name is: {:?}\nI am {:?}\nuser & {:?}", self.name, user_information, self.message);
        }
    }

    let noman: Profile = Profile {
        name: String::from("Noman"),
        message: String::from("Hello Rust"),
        status: ProfileStatus::ACTIVE
    };

    let john: Profile = Profile {
        message: String::from("Hello Word"),
        name: String::from("John"),
        status: ProfileStatus::INACTIVE
    };

    noman.check_status();
    john.check_status();

    #[derive(Debug)]
    enum UserRole {
        ADMIN,
        USER
    }

    #[derive(Debug)]
    struct UserModel {
        name: String,
        role: UserRole
    }

    let name: String = String::from("Jane Doe");
    let role: UserRole = UserRole::ADMIN;

    let name2: String = String::from("Kevin Doe");
    let role2: UserRole = UserRole::USER;

    let user1: UserModel = user_generator(name2, role2);
    println!("User: {:?}, Role: {:?}", user1.name, user1.role);

    let user2 = user_generator(name, role);
    println!("User: {:?}, Role: {:?}", user2.name, user2.role);

    fn user_generator(name: String, role: UserRole) -> UserModel {
        UserModel { name, role }
    }

}

