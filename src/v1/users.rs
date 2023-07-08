use super::users_structs::{CreateUser, UpdateUser, User};
use crate::utils::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::postgres::PgPool;

pub async fn list_users() -> (StatusCode, Json<[u64; 1]>) {
    let users = [1337];

    (StatusCode::OK, Json(users))
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let now = Utc::now();

    let user = User {
        id: 1337,
        name: payload.name,
        surname: payload.surname,
        nickname: payload.nickname,
        created_at: now.to_string(),
        last_updated_at: now.to_string(),
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn get_user(Path(id): Path<u64>) -> (StatusCode, Json<User>) {
    let user = User {
        id,
        name: "Aboba".to_owned(),
        surname: "Foo".to_owned(),
        nickname: "Bar".to_owned(),
        created_at: "1970-01-01 00:00:00.000000000 UTC".to_owned(),
        last_updated_at: "1970-01-01 00:00:00.000000000 UTC".to_owned(),
    };

    (StatusCode::OK, Json(user))
}

pub async fn update_user(
    Path(id): Path<u64>,
    Json(payload): Json<UpdateUser>,
) -> (StatusCode, Json<User>) {
    let now = Utc::now();

    let user = User {
        id,
        name: payload.name.or(Some("Aboba".to_owned())).unwrap(),
        surname: payload.surname.or(Some("Foo".to_owned())).unwrap(),
        nickname: payload.nickname.or(Some("Bar".to_owned())).unwrap(),
        created_at: "1970-01-01 00:00:00.000000000 UTC".to_owned(),
        last_updated_at: now.to_string(),
    };

    (StatusCode::OK, Json(user))
}

pub async fn delete_user(Path(id): Path<u64>) -> StatusCode {
    StatusCode::NO_CONTENT
}

pub async fn root(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}
