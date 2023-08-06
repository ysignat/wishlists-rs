use crate::utils::{AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use chrono::{offset::Utc, NaiveDateTime};
use entities::users::ActiveModel as UserActiveModel;
use entities::users::Entity as UserEntity;
use entities::users::Model as UserModel;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct UserCreate {
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
}

#[derive(Deserialize)]
struct UserUpdate {
    first_name: Option<String>,
    second_name: Option<String>,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    first_name: Option<String>,
    second_name: Option<String>,
    nick_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        User {
            id: value.id,
            first_name: value.first_name,
            second_name: value.second_name,
            nick_name: value.nick_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{root_path}/users"),
            axum::routing::get(list).post(create),
        )
        .route(
            &format!("{root_path}/users/:id"),
            axum::routing::get(get).put(update).delete(delete),
        )
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<User>>), AppError> {
    let users = UserEntity::find()
        .all(&state.postgres_connection)
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(users)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let now = Utc::now().naive_utc();

    let user = UserActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        first_name: ActiveValue::Set(payload.first_name),
        second_name: ActiveValue::Set(payload.second_name),
        nick_name: ActiveValue::Set(payload.nick_name),
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
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = UserEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    Ok((StatusCode::OK, Json(user)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserUpdate>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let now = Utc::now().naive_utc();

    let mut user: UserActiveModel = UserEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    user.first_name = Set(payload.first_name);
    user.second_name = Set(payload.second_name);
    user.updated_at = Set(now);

    let user = user.update(&state.postgres_connection).await?.into();

    Ok((StatusCode::OK, Json(user)))
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = UserEntity::delete_by_id(id)
        .exec(&state.postgres_connection)
        .await?;

    Ok((StatusCode::NO_CONTENT, "User Deleted".to_owned()))
}
