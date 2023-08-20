use axum::{extract::State as AxumState, http::StatusCode, Json};
use chrono::NaiveDateTime;
use database::structs::wishlists::create::{DatabasePayload, DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpPayload {
    name: String,
    user_id: Uuid,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload {
            id: Uuid::new_v4(),
            name: val.name,
            user_id: val.user_id,
        }
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
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .create_wishlist(payload.into())
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(response)))
}
