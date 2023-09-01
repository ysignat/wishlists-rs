use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::NaiveDateTime;
use database::crud::items::{DatabaseCreatePayload, DatabaseUpdatePayload};
use entities::items::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct CreateHttpPayload {
    wishlist_id: Uuid,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

impl From<CreateHttpPayload> for DatabaseCreatePayload {
    fn from(val: CreateHttpPayload) -> Self {
        DatabaseCreatePayload {
            id: Uuid::new_v4(),
            wishlist_id: val.wishlist_id,
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateHttpPayload {
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

impl From<UpdateHttpPayload> for DatabaseUpdatePayload {
    fn from(val: UpdateHttpPayload) -> Self {
        DatabaseUpdatePayload {
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
        }
    }
}

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

impl From<Model> for HttpResponse {
    fn from(value: Model) -> Self {
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

pub async fn list(
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

pub async fn create(
    AxumState(state): AxumState<State>,
    Json(payload): Json<CreateHttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state.repository.create_item(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<HttpResponse>>), AppError> {
    let response = state
        .repository
        .get_item(id)
        .await?
        .map(std::convert::Into::into);

    Ok((StatusCode::OK, Json(response)))
}

pub async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateHttpPayload>,
) -> Result<(StatusCode, Json<HttpResponse>), AppError> {
    let response = state
        .repository
        .update_item(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_item(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

static SUBPATH: &str = "/items";

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
