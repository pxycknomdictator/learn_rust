fn main() {
    /* so what is loop i don't need to explain
        There are 3 types of loops in Rust majorly
        ["for", "while", "loop"]
    */

    // If you hate your self use this code and break your pc
    // let mut counter: i32 = 0;

    // loop {
    //     println!("Oh no! {}", counter);
    //     counter += 1;
    // }

    // The way you exits from infinite loop
    let mut counter: i32 = 0;

    let message: &str = loop {
        println!("Go Go Go... {}", counter);
        counter += 1;

        if counter == 10 {
            break "So we cooked";
        }
    };

    println!("Final message {}", message);

    // Print something with loop in array

    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Noobs use that way
    let mut index: usize = 0;
    loop {
        println!("Noobs Dev {}: {}", index, MONTHS[index]);
        index += 1;
        if MONTHS.len() == index {
            break;
        }
    }

    // Professional use that way
    const DAYS: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let mut second_index: usize = 0;
    while DAYS.len() >= second_index {
        println!(
            "Professional Dev: {}, {}",
            second_index, MONTHS[second_index]
        );
        second_index += 1;
    }

    // Real Epic uses that way

    for day in DAYS {
        println!("Epic Dev: {}", day);
    }
}
