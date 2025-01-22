use my_crates::connect_to_database;

fn main() {
    // Crates and modules in rust

    /*
      Crates: Rust's packages, which can be binary (executable programs) or library (reusable code).
      Modules: Used to organize code within a crate, making it reusable and easy to manage.
      Default File: Rust runs main.rs by default for binary crates and lib.rs for library crates.
      Purpose: These structures ensure clean, modular, and scalable Rust codebases.
    */

    // now i want to access database connection function so first we need to import from our service

    // my_crates::connect_to_database(connection) we can use that way but this is so ugly way to right code

    let database_string: &str = "mongodb://127.0.0.1:27017/database";
    println!("{}", connect_to_database(database_string));
}
