use axum::{
    extract::{Path, Query, State as AxumState},
    http::StatusCode,
    Json,
    Router,
};
use chrono::{NaiveDateTime, Utc};
use database::interfaces::items::{Payload as DatabasePayload, Response as DatabaseResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{users, wishlists};
use crate::{errors::AppError, state::State};

type Id = Uuid;
type PictureId = Uuid;
pub type Predicate = String;

#[derive(Deserialize)]
struct CreatePayload {
    wishlist_id: wishlists::Id,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

impl From<CreatePayload> for DatabasePayload {
    fn from(val: CreatePayload) -> Self {
        DatabasePayload {
            id: Uuid::new_v4(),
            wishlist_id: val.wishlist_id,
            selected_by_id: None,
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
            picture_id: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize)]
struct UpdatePayload {
    wishlist_id: wishlists::Id,
    selected_by_id: Option<users::Id>,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

#[derive(Serialize)]
pub(crate) struct Response {
    id: Id,
    wishlist_id: wishlists::Id,
    selected_by_id: Option<users::Id>,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
    picture_id: Option<PictureId>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<DatabaseResponse> for Response {
    fn from(val: DatabaseResponse) -> Self {
        Response {
            id: val.id,
            wishlist_id: val.wishlist_id,
            selected_by_id: val.selected_by_id,
            name: val.name,
            description: val.description,
            price: val.price,
            is_hidden: val.is_hidden,
            picture_id: val.picture_id,
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
        .list_items(predicate)
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
    let response = state.repository.create_item(payload.into()).await?.into();

    Ok((StatusCode::CREATED, Json(response)))
}

async fn get(
    AxumState(state): AxumState<State>,
    Path(id): Path<Id>,
) -> Result<(StatusCode, Json<Option<Response>>), AppError> {
    let response = state.repository.get_item(id).await?.map(Into::into);

    Ok((StatusCode::OK, Json(response)))
}

async fn update(
    AxumState(state): AxumState<State>,
    Path(id): Path<Id>,
    Json(payload): Json<UpdatePayload>,
) -> Result<(StatusCode, Json<Response>), AppError> {
    match state.repository.get_item(id).await? {
        Some(object) => {
            let response = state
                .repository
                .update_item(
                    id,
                    DatabasePayload {
                        id,
                        wishlist_id: payload.wishlist_id,
                        selected_by_id: payload.selected_by_id,
                        name: payload.name,
                        description: payload.description,
                        price: payload.price,
                        is_hidden: payload.is_hidden,
                        picture_id: object.picture_id,
                        created_at: object.created_at,
                        updated_at: Utc::now().naive_local(),
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
    Path(id): Path<Id>,
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
