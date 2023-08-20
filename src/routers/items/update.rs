use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use database::structs::items::update::{DatabasePayload, DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct HttpPayload {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload {
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
        }
    }
}

#[derive(Serialize)]
pub struct Response {
    id: Uuid,
    wishlist_id: Uuid,
    selected_by_id: Option<Uuid>,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for Response {
    fn from(value: DatabaseResponse) -> Self {
        Response {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let response = state
        .repository
        .update_item(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}
