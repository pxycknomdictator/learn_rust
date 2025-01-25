#![allow(dead_code)]

mod front_of_house {
    // if i remove pub keyword from both module and fn this will be a private module and fn
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

fn deliver_order() {} // here

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // To access deliver_order fn we use super thats mean go one step up
        super::deliver_order();
    }
    fn cook_order() {}
}

// If i create an enum and make it public all fields are also public
pub enum Angle {
  Top,
  Right,
  Bottom,
  Left,
}

// But in case of structs I need to manually mention that which field will we public or private

pub struct User {
  pub username: String, // visible
  pub email: String, // visible
  password: String // not visible
}