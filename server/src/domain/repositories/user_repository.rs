use async_trait::async_trait;

use crate::domain::entities::user::User;

use super::error::RepositoryResult;

#[derive(Debug)]
pub struct UpdateInput {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait UserRepositoryPort: Sync + Send {
    async fn find_by_id(&self, id: i64) -> RepositoryResult<User>;
    async fn update_one(&self, input: UpdateInput) -> RepositoryResult<User>;
}
