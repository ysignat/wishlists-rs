use axum::{extract::State as AxumState, http::StatusCode, Json};
use chrono::NaiveDateTime;
use database::structs::users::create::{DatabasePayload, DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpPayload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

impl From<HttpPayload> for DatabasePayload {
    fn from(val: HttpPayload) -> Self {
        DatabasePayload {
            id: Uuid::new_v4(),
            first_name: val.first_name,
            second_name: val.second_name,
            nick_name: val.nick_name,
        }
    }
}

#[derive(Serialize)]
pub struct HttpResponse {
    id: Uuid,
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for HttpResponse {
    fn from(value: DatabaseResponse) -> Self {
        HttpResponse {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub async fn handler(
    AxumState(state): AxumState<State>,
    Json(payload): Json<HttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.create_user(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}
