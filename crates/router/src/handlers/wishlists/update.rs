use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use database::structs::wishlists::update::{DatabasePayload, DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpPayload {
    name: String,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload { name: val.name }
    }
}

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
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .update_wishlist(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}
