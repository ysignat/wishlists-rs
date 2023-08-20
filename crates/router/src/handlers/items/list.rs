use axum::{extract::State as AxumState, http::StatusCode, Json};
use chrono::NaiveDateTime;
use database::structs::items::list::DatabaseResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::{errors::AppError, state::State};

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
) -> Result<(StatusCode, Json<Vec<HttpResponse>>), AppError> {
    let response = state
        .repository
        .list_items()
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}
