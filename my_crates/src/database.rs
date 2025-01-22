pub enum Status {
  CONNECTED(bool),
  INTERRUPTED(bool),
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