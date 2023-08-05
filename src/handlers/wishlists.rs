use crate::utils::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::offset::Utc;
use entities::wishlists::ActiveModel as WishlistActiveModel;
use entities::wishlists::Entity as Wishlist;
use entities::wishlists::Model as WishlistModel;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub async fn list(
    State(connection): State<DatabaseConnection>,
) -> Result<(StatusCode, Json<Vec<WishlistModel>>), AppError> {
    let wishlists = Wishlist::find().all(&connection).await?;

    Ok((StatusCode::OK, Json(wishlists)))
}

pub async fn create(
    State(connection): State<DatabaseConnection>,
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
    .insert(&connection)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get(
    State(connection): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<WishlistModel>), AppError> {
    let wishlist = Wishlist::find_by_id(id).one(&connection).await?.unwrap();

    Ok((StatusCode::OK, Json(wishlist)))
}

pub async fn update(
    State(connection): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<WishlistModel>,
) -> Result<(StatusCode, Json<WishlistModel>), AppError> {
    let now = Utc::now().naive_utc();

    let mut wishlist: WishlistActiveModel = Wishlist::find_by_id(id)
        .one(&connection)
        .await?
        .unwrap()
        .into();

    wishlist.name = Set(payload.name);
    wishlist.updated_at = Set(now);

    let user = wishlist.update(&connection).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete(
    State(connection): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = Wishlist::delete_by_id(id).exec(&connection).await?;

    Ok((StatusCode::NO_CONTENT, "Wishlist Deleted".to_owned()))
}
