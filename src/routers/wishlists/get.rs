use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use database::structs::wishlists::get::DatabaseResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Serialize)]
pub struct Response {
    id: Uuid,
    name: String,
    user_id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for Response {
    fn from(value: DatabaseResponse) -> Self {
        Response {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let response = state.repository.get_wishlist(id).await?.into();

    Ok((StatusCode::OK, Json(response)))
}
