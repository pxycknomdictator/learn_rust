pub struct Credentials {
  pub username: String,
  pub password: String,
}

fn login() {}

// let suppose i need to use status in this module so we need to import this module
// use super::database::Status;

pub fn get_user(cred: &Credentials) {
  // database query
  login();
}