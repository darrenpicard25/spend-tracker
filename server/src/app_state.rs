use std::sync::Arc;

use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::postgres::PgPoolOptions;

use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::{domain::repositories::user_repository::UserRepositoryPort, error::ServiceError};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub user_repository: Arc<dyn UserRepositoryPort>,
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions) -> Result<Self, ServiceError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://spend_tracker:password@localhost/spend_tracker")
            .await
            .map_err(|e| ServiceError::DatabaseConnection(e.to_string()))?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| ServiceError::DatabaseMigration(e.to_string()))?;

        // Repositories
        let user_repository = Arc::new(UserRepository::new(pool));

        Ok(Self {
            user_repository,
            leptos_options,
        })
    }
}
