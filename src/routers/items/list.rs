use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDateTime;
use entities::items::{Entity, Model};
use sea_orm::EntityTrait;
use serde::Serialize;
use uuid::Uuid;

use crate::utils::{AppError, AppState};

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
) -> Result<(StatusCode, Json<Vec<Response>>), AppError> {
    let response = Entity::find()
        .all(&state.postgres_connection)
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}
