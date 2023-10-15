use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

pub type Id = Uuid;
type AvatarId = Uuid;
type Predicate = &'static str;

#[derive(Deserialize)]
struct CreatePayload {
    name: String,
    avatar_id: Option<AvatarId>,
}

#[derive(Deserialize)]
struct UpdatePayload {
    name: String,
    avatar_id: Option<AvatarId>,
}

#[derive(Serialize)]
struct Response {
    id: Uuid,
    name: String,
    avatar_id: Option<AvatarId>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

async fn create(
    AxumState(state): AxumState<State>,
    Json(payload): Json<CreatePayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let response = state.repository.create_user(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    AxumState(state): AxumState<State>,
) -> Result<(StatusCode, Json<Vec<Response>>), AppError> {
    let response = state
        .repository
        .list_users()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Option<Response>>), AppError> {
    let response = state.repository.get_user(id).await?.map(Into::into);

    Ok((StatusCode::OK, Json(response)))
}

async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    let response = state
        .repository
        .update_user(id, payload.into())
        .await?
        .into();

    Ok((StatusCode::OK, Json(response)))
}

async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_user(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

static SUBPATH: &str = "/users";

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
