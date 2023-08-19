use axum::{extract::State, http::StatusCode, Json};
use chrono::{offset::Utc, NaiveDateTime};
use database::structs::items::create::DatabasePayload;
use entities::items::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct HttpPayload {
    pub wishlist_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload {
            wishlist_id: val.wishlist_id,
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

impl From<Model> for Response {
    fn from(value: Model) -> Self {
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
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();
    let uuid = Uuid::new_v4();

    let response = state
        .repository
        .create_item(uuid, now, payload.into())
        .await
        .unwrap()
        .into();

    Ok((StatusCode::CREATED, Json(response)))
}
