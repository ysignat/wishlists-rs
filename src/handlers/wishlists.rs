use crate::structs::wishlists::{CreateWishlist, UpdateWishlist, Wishlist, WishlistQueryParams};
use crate::utils::AppError;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn list_wishlists(
    State(pool): State<PgPool>,
    Query(params): Query<WishlistQueryParams>,
) -> Result<(StatusCode, Json<Vec<Wishlist>>), AppError> {
    let wishlists = sqlx::query_as!(
        Wishlist,
        r#"
          select 
            id,
            user_id,
            name, 
            created_at, 
            updated_at 
          from wishlists
          where user_id = $1; 
        "#,
        params.user_id
    )
    .fetch_all(&pool)
    .await?;

    Ok((StatusCode::OK, Json(wishlists)))
}

pub async fn create_wishlist(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateWishlist>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let wishlist = sqlx::query_as!(
        Wishlist,
        r#"
          insert into wishlists (
            id,
            user_id, 
            name, 
            created_at, 
            updated_at
          ) values (
            $1, 
            $2, 
            $3, 
            now(), 
            now()
          ) returning *;
        "#,
        Uuid::new_v4(),
        payload.user_id,
        payload.name,
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(wishlist)))
}

pub async fn get_wishlist(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let wishlist = sqlx::query_as!(
        Wishlist,
        r#"
          select 
            id, 
            user_id,
            name, 
            created_at, 
            updated_at 
          from wishlists 
          where 
            id = $1;
        "#,
        id,
    )
    .fetch_one(&pool) // TODO change to fetch optional
    .await?;

    Ok((StatusCode::OK, Json(wishlist)))
}

pub async fn update_wishlist(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateWishlist>,
) -> Result<(StatusCode, Json<Wishlist>), AppError> {
    let wishlist = sqlx::query_as!(
        Wishlist,
        r#"
          update wishlists
          set
            name = coalesce($1, name),
            updated_at = now()
          where id = $2
          returning *;
        "#,
        payload.name,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(wishlist)))
}

pub async fn delete_wishlist(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    sqlx::query_as!(
        User,
        r#"
          delete from wishlists
          where id = $1;
        "#,
        id
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::NO_CONTENT, "Wishlist Deleted".to_owned()))
}
