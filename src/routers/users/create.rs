use axum::{extract::State, http::StatusCode, Json};
use chrono::{offset::Utc, NaiveDateTime};
use entities::users::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
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
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let response = ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        first_name: ActiveValue::Set(payload.first_name),
        second_name: ActiveValue::Set(payload.second_name),
        nick_name: ActiveValue::Set(payload.nick_name),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.database_connection)
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(response)))
}
