use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use entities::users::{Entity, Model};
use sea_orm::EntityTrait;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Serialize)]
pub struct Response {
    id: Uuid,
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Model> for Response {
    fn from(value: Model) -> Self {
        Response {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let response = Entity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    Ok((StatusCode::OK, Json(response)))
}
