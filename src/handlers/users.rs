use crate::structs::users::{CreateUser, UpdateUser, User};
use crate::utils::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn list(State(pool): State<PgPool>) -> Result<(StatusCode, Json<Vec<User>>), AppError> {
    let users = sqlx::query_as!(
        User,
        r#"
        select 
          id, 
          first_name, 
          second_name, 
          nickname, 
          created_at, 
          updated_at 
        from users 
      "#
    )
    .fetch_all(&pool)
    .await?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        insert into users (
          id, 
          first_name, 
          second_name, 
          nickname, 
          created_at, 
          updated_at
        ) values (
          $1, 
          $2, 
          $3, 
          $4, 
          now(), 
          now()
        ) returning *;
      "#,
        Uuid::new_v4(),
        payload.first_name,
        payload.second_name,
        payload.nickname
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
          select 
            id, 
            first_name, 
            second_name, 
            nickname, 
            created_at, 
            updated_at 
          from users 
          where id = $1;
        "#,
        id
    )
    .fetch_one(&pool) // TODO change to fetch optional
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
          update users
          set
            first_name = coalesce($1, first_name),
            second_name = coalesce($2, second_name),
            nickname = coalesce($3, nickname),
            updated_at = now()
          where id = $4
          returning *;
        "#,
        payload.first_name,
        payload.second_name,
        payload.nickname,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    sqlx::query_as!(
        User,
        r#"
          delete from users
          where id = $1;
        "#,
        id
    )
    .execute(&pool)
    .await?;

    Ok((StatusCode::NO_CONTENT, "User Deleted".to_owned()))
}
