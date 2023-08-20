use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use database::structs::wishlists::get::DatabaseResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Serialize)]
pub struct HttpResponse {
    id: Uuid,
    name: String,
    user_id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for HttpResponse {
    fn from(value: DatabaseResponse) -> Self {
        HttpResponse {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn handler(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.get_wishlist(id).await?.into();

    Ok((StatusCode::OK, Json(response)))
}
