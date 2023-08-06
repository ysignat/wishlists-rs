use crate::utils::{AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use chrono::{offset::Utc, NaiveDateTime};
use entities::wishlists::ActiveModel as WishlistActiveModel;
use entities::wishlists::Entity as WishlistEntity;
use entities::wishlists::Model as WishlistModel;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct WishlistCreate {
    name: String,
    user_id: Uuid,
}

#[derive(Deserialize)]
struct WishlistUpdate {
    name: String,
}

#[derive(Serialize)]
struct Wishlist {
    id: Uuid,
    name: String,
    user_id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<WishlistModel> for Wishlist {
    fn from(value: WishlistModel) -> Self {
        Wishlist {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{root_path}/wishlists"),
            axum::routing::get(list).post(create),
        )
        .route(
            &format!("{root_path}/wishlists/:id"),
            axum::routing::get(get).put(update).delete(delete),
        )
        .with_state(state)
}

async fn list(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Wishlist>>), AppError> {
    let wishlists = WishlistEntity::find()
        .all(&state.postgres_connection)
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(wishlists)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<WishlistCreate>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let now = Utc::now().naive_utc();

    let user = WishlistActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        user_id: ActiveValue::Set(payload.user_id),
        name: ActiveValue::Set(payload.name),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.postgres_connection)
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(user)))
}

async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let wishlist = WishlistEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    Ok((StatusCode::OK, Json(wishlist)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<WishlistUpdate>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let now = Utc::now().naive_utc();

    let mut wishlist: WishlistActiveModel = WishlistEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    wishlist.name = Set(payload.name);
    wishlist.updated_at = Set(now);

    let user = wishlist.update(&state.postgres_connection).await?.into();

    Ok((StatusCode::OK, Json(user)))
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = WishlistEntity::delete_by_id(id)
        .exec(&state.postgres_connection)
        .await?;

    Ok((StatusCode::NO_CONTENT, "Wishlist Deleted".to_owned()))
}
