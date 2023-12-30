use axum::extract::{Path, State};

use crate::{adapters::api::error::ControllerResult, app_state::AppState};

pub async fn handler(
    State(AppState {
        user_repository, ..
    }): State<AppState>,
    Path(id): Path<i64>,
) -> ControllerResult<super::User> {
    log::info!("Get /todo/{id}");

    let user = user_repository.find_by_id(id).await?;

    Ok(user.into())
}
