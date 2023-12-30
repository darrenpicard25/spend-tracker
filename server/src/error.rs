#[derive(Debug)]
pub enum ServiceError {
    StartUp(String),
    DatabaseConnection(String),
    DatabaseMigration(String),
}
