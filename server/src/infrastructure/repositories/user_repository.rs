use async_trait::async_trait;
use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Postgres};

use crate::domain::{
    entities::user::User,
    repositories::{
        error::{RepositoryError, RepositoryResult},
        user_repository::{UpdateInput, UserRepositoryPort},
    },
};

#[derive(FromRow, Debug)]
struct UserDocument {
    id: i64,
    first_name: String,
    last_name: String,
    nickname: Option<String>,
    #[allow(dead_code)]
    created_at: PrimitiveDateTime,
    #[allow(dead_code)]
    updated_at: PrimitiveDateTime,
}

impl From<UserDocument> for User {
    fn from(value: UserDocument) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            nickname: value.nickname,
        }
    }
}

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn find_by_id(&self, id: i64) -> RepositoryResult<User> {
        log::debug!("UserRepository.find_by_id | {id}");

        let document = sqlx::query_as::<_, UserDocument>("SELECT * FROM todos WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => RepositoryError::NotFound,
                e => RepositoryError::Unknown(e.to_string()),
            })?;

        Ok(document.into())
    }
    async fn update_one(&self, _input: UpdateInput) -> RepositoryResult<User> {
        todo!();
    }
}
