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
}
