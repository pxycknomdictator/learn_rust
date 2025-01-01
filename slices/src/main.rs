fn main() {
    // After a long time of learning, we are finally here This topic will be about slices and this make me sick

    let greetings: String = String::from("Hello Rust");

    // here we get a particular word from the string :)
    let result =  get_first_word(&greetings);
    println!("The string: {} world length is: {}", greetings, result);
}

fn get_first_word(input: &String) -> usize {
    // Honestly i don't know what i'm doing here
    
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    input.len()
}