#[derive(Debug)]
enum Color {
  Red,
  Green,
  Blue
}

fn main() {
  let color1 = Color::Blue;
  let color2 = Color::Red;
  let color3 = Color::Green;
  println!("Your choice is: {:?}", color1);
  println!("Your choice is: {:?}", color2);
  println!("Your choice is: {:?}", color3);
}