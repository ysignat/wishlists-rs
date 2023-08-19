use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{offset::Utc, NaiveDateTime};
use entities::items::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

#[derive(Deserialize)]
pub struct Payload {
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
    Path(id): Path<Uuid>,
    Json(payload): Json<Payload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let now = Utc::now().naive_utc();

    let mut active_model: ActiveModel = Entity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    active_model.name = Set(payload.name);
    active_model.description = Set(payload.description);
    active_model.price = Set(payload.price);
    active_model.is_hidden = Set(payload.is_hidden);
    active_model.updated_at = Set(now);

    let response = active_model
        .update(&state.postgres_connection)
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}
