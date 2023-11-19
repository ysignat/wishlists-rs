use axum::{
    extract::{Path, Query, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use database::traits::wishlists::{Payload as DatabasePayload, Response as DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::items;
use crate::router::{errors::AppError, state::State};

pub type Id = Uuid;
pub(crate) type Predicate = String;

#[derive(Deserialize)]
struct CreatePayload {
    name: String,
    user_id: Uuid,
}

impl From<CreatePayload> for DatabasePayload {
    fn from(val: CreatePayload) -> Self {
        DatabasePayload {
            id: Uuid::new_v4(),
            name: val.name,
            user_id: val.user_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
struct UpdatePayload {
    name: String,
}

#[derive(Serialize)]
pub(crate) struct Response {
    id: Uuid,
    name: String,
    user_id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for Response {
    fn from(val: DatabaseResponse) -> Self {
        Response {
            id: val.id,
            name: val.name,
            user_id: val.user_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

async fn list(
    AxumState(state): AxumState<State>,
    Query(predicate): Query<Option<Predicate>>,
) -> Result<(StatusCode, Json<Vec<Response>>), AppError> {
    let response = state
        .repository
        .list_wishlists(predicate)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

async fn create(
    AxumState(state): AxumState<State>,
    Json(payload): Json<CreatePayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
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
) -> Result<(StatusCode, Json<Option<Response>>), AppError> {
    let response = state.repository.get_wishlist(id).await?.map(Into::into);

    Ok((StatusCode::OK, Json(response)))
}

async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    match state.repository.get_wishlist(id).await? {
        Some(object) => {
            let response = state
                .repository
                .update_wishlist(
                    id,
                    DatabasePayload {
                        id,
                        name: payload.name,
                        user_id: object.user_id,
                        created_at: object.created_at,
                        updated_at: Utc::now().naive_utc(),
                    },
                )
                .await?
                .into();

            Ok((StatusCode::OK, Json(response)))
        }
        None => todo!(),
    }
}

async fn delete(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_wishlist(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}

async fn list_items(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
    Query(predicate): Query<Option<items::Predicate>>,
) -> Result<(StatusCode, Json<Vec<items::Response>>), AppError> {
    let response = state
        .repository
        .list_wishlist_items(id, predicate)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok((StatusCode::OK, Json(response)))
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
