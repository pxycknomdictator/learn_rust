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

    let user1 = User {
        name: String::from("Noman"),
        status: Status::Active,
    };

    let user2: User = User {
        name: String::from("John"),
        status: Status::InActive
    };

    user1.check_status();
    user2.check_status();
}
