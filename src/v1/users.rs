use super::users_structs::{CreateUser, UpdateUser, User};
use crate::utils::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn list_users(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<User>>), AppError> {
    let users = sqlx::query_as!(
        User,
        r#"
        select 
          id, 
          name, 
          surname, 
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

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        insert into users (
          id, 
          name, 
          surname, 
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
        payload.name,
        payload.surname,
        payload.nickname
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
          select 
            id, 
            name, 
            surname, 
            nickname, 
            created_at, 
            updated_at 
          from users 
          where id = $1;
        "#,
        id
    )
    .fetch_one(&pool) // TODO change to getch optional
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
          update users
          set
            name = coalesce($1, name),
            surname = coalesce($2, surname),
            nickname = coalesce($3, nickname),
            updated_at = now()
          where id = $4
          returning *;
        "#,
        payload.name,
        payload.surname,
        payload.nickname,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user(
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
