use axum::{extract::State as AxumState, http::StatusCode, Json};
use chrono::NaiveDateTime;
use database::structs::items::create::{DatabasePayload, DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpPayload {
    wishlist_id: Uuid,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload {
            id: Uuid::new_v4(),
            wishlist_id: val.wishlist_id,
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
        }
    }
}

#[derive(Serialize)]
pub struct HttpResponse {
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

impl From<DatabaseResponse> for HttpResponse {
    fn from(value: DatabaseResponse) -> Self {
        HttpResponse {
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
    AxumState(state): AxumState<State>,
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.create_item(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}