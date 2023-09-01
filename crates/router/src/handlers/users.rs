use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::NaiveDateTime;
use database::structs::users::{CreatePayload, UpdatePayload};
use entities::users::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

#[derive(Deserialize)]
pub struct CreateHttpPayload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

impl From<CreateHttpPayload> for CreatePayload {
    fn from(val: CreateHttpPayload) -> Self {
        CreatePayload {
            id: Uuid::new_v4(),
            first_name: val.first_name,
            second_name: val.second_name,
            nick_name: val.nick_name,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateHttpPayload {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

impl From<UpdateHttpPayload> for UpdatePayload {
    fn from(val: UpdateHttpPayload) -> Self {
        UpdatePayload {
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

impl From<Model> for HttpResponse {
    fn from(value: Model) -> Self {
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
    Json(payload): Json<CreateHttpPayload>,
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
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<HttpResponse>>), AppError> {
    let response = state
        .repository
        .get_user(id)
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
