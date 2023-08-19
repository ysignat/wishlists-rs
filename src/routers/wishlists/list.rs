use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDateTime;
use entities::wishlists::{Entity, Model};
use sea_orm::EntityTrait;
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

impl From<Model> for Response {
    fn from(value: Model) -> Self {
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
) -> Result<(StatusCode, Json<Vec<Response>>), AppError> {
    let response = Entity::find()
        .all(&state.postgres_connection)
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}
