fn main() {
    let is_authentication: bool = true;

    if !is_authentication {
        println!("You are not authentication");
    } else {
        println!("You are authenticate person");
    }

    let day: &str = "Friday";

    if day == "Monday" {
        println!("First day is: {}", day);
    } else if day == "Tuesday" {
        println!("Second day is: {}", day);
    } else if day == "Wednesday" {
        println!("Third day is: {}", day);
    } else if day == "Thursday" {
        println!("Fourth day is: {}", day);
    } else if day == "Friday" {
        println!("Fifth day is: {}", day);
    } else if day == "Saturday" {
        println!("Sixth day is: {}", day);
    } else {
        println!("Okay so the day is: {}", day);
    }

    // lets make a game

    let is_citizen: bool = true;
    let is_eighteen_plus: bool = true;
    let has_national_card: bool = true;

    if is_citizen {
        if is_eighteen_plus {
            if has_national_card {
                println!("You can vote!")
            } else {
                println!("You don't have NIC")
            }
        } else {
            println!("You are not a adult")
        }
    } else {
        println!("You are not a citizen")
    }

    // Number game

    let my_number: u8 = 90;

    if my_number == 80 {
        println!("The number is Eighty")
    } else if my_number == 29 {
        println!("The number is Twenty nine")
    } else if my_number == 30 {
        println!("The number is Thirty")
    } else {
        println!("The number is out of mind")
    }

    // Even & Odd Game
    let number: u8 = 89;

    if number % 2 == 0 {
        println!("The number is Even: {}", number)
    } else {
        println!("The number is Odd: {}", number)
    }
}
