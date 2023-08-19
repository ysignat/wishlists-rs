use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{offset::Utc, NaiveDateTime};
use entities::users::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
    first_name: Option<String>,
    second_name: Option<String>,
}

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
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let mut active_model: ActiveModel = Entity::find_by_id(id)
        .one(&state.database_connection)
        .await?
        .unwrap()
        .into();

    active_model.first_name = Set(payload.first_name);
    active_model.second_name = Set(payload.second_name);
    active_model.updated_at = Set(now);

    let response = active_model
        .update(&state.database_connection)
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}
