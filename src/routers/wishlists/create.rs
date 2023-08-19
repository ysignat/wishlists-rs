use axum::{extract::State, http::StatusCode, Json};
use chrono::{offset::Utc, NaiveDateTime};
use entities::wishlists::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
    name: String,
    user_id: Uuid,
}

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
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let response = ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        user_id: ActiveValue::Set(payload.user_id),
        name: ActiveValue::Set(payload.name),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.postgres_connection)
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(response)))
}
