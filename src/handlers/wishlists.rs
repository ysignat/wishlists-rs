use crate::utils::{AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use chrono::offset::Utc;
use entities::wishlists::ActiveModel as WishlistActiveModel;
use entities::wishlists::Entity as Wishlist;
use entities::wishlists::Model as WishlistModel;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};
use uuid::Uuid;

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

pub async fn list(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<WishlistModel>>), AppError> {
    let wishlists = Wishlist::find().all(&state.postgres_connection).await?;

    Ok((StatusCode::OK, Json(wishlists)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<WishlistModel>,
) -> Result<(StatusCode, Json<WishlistModel>), AppError> {
    let now = Utc::now().naive_utc();

    let user = WishlistActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        user_id: ActiveValue::Set(payload.user_id),
        name: ActiveValue::Set(payload.name),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.postgres_connection)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<WishlistModel>), AppError> {
    let wishlist = Wishlist::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap();

    Ok((StatusCode::OK, Json(wishlist)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<WishlistModel>,
) -> Result<(StatusCode, Json<WishlistModel>), AppError> {
    let now = Utc::now().naive_utc();

    let mut wishlist: WishlistActiveModel = Wishlist::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    wishlist.name = Set(payload.name);
    wishlist.updated_at = Set(now);

    let user = wishlist.update(&state.postgres_connection).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = Wishlist::delete_by_id(id)
        .exec(&state.postgres_connection)
        .await?;

    Ok((StatusCode::NO_CONTENT, "Wishlist Deleted".to_owned()))
}
