use axum::{extract::State, http::StatusCode, Json};
use chrono::{offset::Utc, NaiveDateTime};
use entities::items::{ActiveModel, Model};
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
    wishlist_id: Uuid,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
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
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let response = ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        wishlist_id: ActiveValue::Set(payload.wishlist_id),
        selected_by_id: ActiveValue::Set(None),
        name: ActiveValue::Set(payload.name),
        description: ActiveValue::Set(payload.description),
        price: ActiveValue::Set(payload.price),
        is_hidden: ActiveValue::Set(payload.is_hidden),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.database_connection)
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(response)))
}
