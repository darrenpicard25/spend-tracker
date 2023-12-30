#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    Unknown(String),
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
