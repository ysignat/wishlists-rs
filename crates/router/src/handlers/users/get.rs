use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use database::structs::users::get::DatabaseResponse;
use serde::Serialize;
use uuid::Uuid;

use crate::{errors::AppError, state::State};

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
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.get_user(id).await?.into();

    Ok((StatusCode::OK, Json(response)))
}
