use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::NaiveDateTime;
use database::crud::wishlists::{DatabaseCreatePayload, DatabaseResponse, DatabaseUpdatePayload};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct HttpCreatePayload {
    name: String,
    user_id: Uuid,
}

impl From<HttpCreatePayload> for DatabaseCreatePayload {
    fn from(val: HttpCreatePayload) -> Self {
        DatabaseCreatePayload {
            id: Uuid::new_v4(),
            name: val.name,
            user_id: val.user_id,
        }
    }
}

#[derive(Deserialize)]
pub struct HttpUpdatePayload {
    name: String,
}

impl From<HttpUpdatePayload> for DatabaseUpdatePayload {
    fn from(val: HttpUpdatePayload) -> Self {
        DatabaseUpdatePayload { name: val.name }
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

pub async fn list(
    AxumState(state): AxumState<State>,
) -> Result<(StatusCode, Json<Vec<HttpResponse>>), AppError> {
    let response = state
        .repository
        .list_wishlists()
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn create(
    AxumState(state): AxumState<State>,
    Json(payload): Json<HttpCreatePayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .create_wishlist(payload.into())
        .await?
        .into();

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<HttpResponse>>), AppError> {
    let response = state
        .repository
        .get_wishlist(id)
        .await?
        .map(std::convert::Into::into);

    Ok((StatusCode::OK, Json(response)))
}

pub async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Json(payload): Json<HttpUpdatePayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .update_wishlist(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_wishlist(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

static SUBPATH: &str = "/wishlists";

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