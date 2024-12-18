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
}
