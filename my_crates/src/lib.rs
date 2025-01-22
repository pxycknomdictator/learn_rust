// Let suppose we are try to connect database and also checking the status of db

// By default all functions and variables in rust are private thats mean i cant access this any where to access that we use (pub aka public) keyword

#![allow(dead_code, unused_variables)]
pub enum Status {
    CONNECTED(bool),
    INTERRUPTED(bool),
}

// If i want to a specific field to public i need to mention that also
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn connect_to_database(connection: &str) -> &str {
    if connection.len() > 0 {
        return "Database connected successfully";
    }
    "Database connection failed"
}

pub fn database_status(status: &Status) -> &str {
    match status {
        Status::CONNECTED(true) => "Status is connected",
        _ => "Status is not connected",
    }
}

fn login() {}

pub fn get_user(cred: &Credentials) {
    // database query
    login();
}
