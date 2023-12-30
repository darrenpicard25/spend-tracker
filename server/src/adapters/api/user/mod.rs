use axum::{response::IntoResponse, routing, Json, Router};
use serde::Serialize;

use crate::app_state::AppState;

mod get;

#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String,
}

impl IntoResponse for User {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<crate::domain::entities::user::User> for User {
    fn from(value: crate::domain::entities::user::User) -> Self {
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/user/:id", routing::get(get::handler))
}
