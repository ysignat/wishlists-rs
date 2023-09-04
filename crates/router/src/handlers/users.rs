use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use database::crud::users::{DatabaseCreatePayload, DatabaseResponse, DatabaseUpdatePayload};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpCreatePayload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

impl From<HttpCreatePayload> for DatabaseCreatePayload {
    fn from(val: HttpCreatePayload) -> Self {
        DatabaseCreatePayload {
            id: Uuid::new_v4(),
            first_name: val.first_name,
            second_name: val.second_name,
            nick_name: val.nick_name,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct HttpUpdatePayload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

impl From<HttpUpdatePayload> for DatabaseUpdatePayload {
    fn from(val: HttpUpdatePayload) -> Self {
        DatabaseUpdatePayload {
            first_name: val.first_name,
            second_name: val.second_name,
            nick_name: val.nick_name,
            updated_at: Utc::now().naive_utc(),
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

pub async fn create(
    AxumState(state): AxumState<State>,
    Json(payload): Json<HttpCreatePayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.create_user(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn list(
    AxumState(state): AxumState<State>,
) -> Result<(StatusCode, Json<Vec<HttpResponse>>), AppError> {
    let response = state
        .repository
        .list_users()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<HttpResponse>>), AppError> {
    let response = state.repository.get_user(id).await?.map(Into::into);

    Ok((StatusCode::OK, Json(response)))
}

pub async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Json(payload): Json<HttpUpdatePayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .update_user(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_user(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

static SUBPATH: &str = "/users";

pub fn get_router(root_path: &str, state: State) -> Router {
    Router::new()
        .route(
            &format!("{root_path}{SUBPATH}"),
            axum::routing::get(list).post(create),
        )
        .route(
            &format!("{root_path}{SUBPATH}/:id"),
            axum::routing::get(get).put(update).delete(delete),
        )
        .with_state(state)
}
