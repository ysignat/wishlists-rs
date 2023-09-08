use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use database::{
    WishlistsDatabaseCreatePayload,
    WishlistsDatabaseResponse,
    WishlistsDatabaseUpdatePayload,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
struct HttpCreatePayload {
    name: String,
    user_id: Uuid,
}

impl From<HttpCreatePayload> for WishlistsDatabaseCreatePayload {
    fn from(val: HttpCreatePayload) -> Self {
        WishlistsDatabaseCreatePayload {
            id: Uuid::new_v4(),
            name: val.name,
            user_id: val.user_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
struct HttpUpdatePayload {
    name: String,
}

impl From<HttpUpdatePayload> for WishlistsDatabaseUpdatePayload {
    fn from(val: HttpUpdatePayload) -> Self {
        WishlistsDatabaseUpdatePayload {
            name: val.name,
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize)]
struct HttpResponse {
    id: Uuid,
    name: String,
    user_id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<WishlistsDatabaseResponse> for HttpResponse {
    fn from(value: WishlistsDatabaseResponse) -> Self {
        HttpResponse {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

async fn list(
    AxumState(state): AxumState<State>,
) -> Result<(StatusCode, Json<Vec<HttpResponse>>), AppError> {
    let response = state
        .repository
        .list_wishlists()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

async fn create(
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

async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<HttpResponse>>), AppError> {
    let response = state.repository.get_wishlist(id).await?.map(Into::into);

    Ok((StatusCode::OK, Json(response)))
}

async fn update(
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

async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_wishlist(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

static SUBPATH: &str = "/wishlists";

pub(crate) fn get_router(root_path: &str, state: State) -> Router {
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
