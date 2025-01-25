#![allow(dead_code)]

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

fn eat_at_restaurant() {
  // There are two ways to access add_to_waitlist function

  // 1. absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // 2. relative path
  front_of_house::hosting::add_to_waitlist();
}