// Let suppose we are try to connect database and also checking the status of db

// By default all functions and variables in rust are private thats mean i cant access this any where to access that we use (pub aka public) keyword

// as you can see my code looks so ugly to organize this we use mod aka modules

#![allow(dead_code, unused_variables)]

// If i want to a specific field to public i need to mention that also

// We successfully create a different file modules and import that with mod and filename why we use pub because this mod we are using in main.rs file
pub mod database;
pub mod authentication;
