use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{offset::Utc, NaiveDateTime};
use database::structs::wishlists::update::DatabasePayload;
use entities::wishlists::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
    pub name: String,
}

impl From<Payload> for DatabasePayload {
    fn from(val: Payload) -> Self {
        DatabasePayload { name: val.name }
    }
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
    Path(id): Path<Uuid>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let response = state
        .repository
        .update_wishlist(now, id, payload.into())
        .await
        .unwrap()
        .into();

    Ok((StatusCode::OK, Json(response)))
}
