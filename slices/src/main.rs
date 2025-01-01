fn main() {
    // After a long time of learning, we are finally here This topic will be about slices and this make me sick

    // So whats happening here is that i have a string and i want to get the first word of it and i'm using a function to do that and i'm passing the string as a reference to the function after returning the function i clear original string and print it and the result of the function I just calculate the length and result is the calculated length of the first word of the string

    let mut greetings: String = String::from("Hello Rust");
    let result =  get_first_word(&greetings);

    greetings.clear(); // here i clear the string and make some unexpected result

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