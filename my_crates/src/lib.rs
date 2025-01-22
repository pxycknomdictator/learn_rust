// Let suppose we are try to connect database and also checking the status of db

// By default all functions and variables in rust are private thats mean i cant access this any where to access that we use (pub aka public) keyword

pub fn connect_to_database(connection: &str) -> &str {
  if connection.len() > 0  {
      return "Database connected successfully";
  };
  "Database connection failed"
}