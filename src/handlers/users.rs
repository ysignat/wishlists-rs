use crate::utils::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::offset::Utc;
use entities::users::ActiveModel as UserActiveModel;
use entities::users::Entity as User;
use entities::users::Model as UserModel;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub async fn list(
    State(connection): State<DatabaseConnection>,
) -> Result<(StatusCode, Json<Vec<UserModel>>), AppError> {
    let users = User::find().all(&connection).await?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn create(
    State(connection): State<DatabaseConnection>,
    Json(payload): Json<UserModel>,
) -> Result<(StatusCode, Json<UserModel>), AppError> {
    let now = Utc::now().naive_utc();

    let user = UserActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        first_name: ActiveValue::Set(payload.first_name),
        second_name: ActiveValue::Set(payload.second_name),
        nick_name: ActiveValue::Set(payload.nick_name),
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
) -> Result<(StatusCode, Json<UserModel>), AppError> {
    let user = User::find_by_id(id).one(&connection).await?.unwrap();

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update(
    State(connection): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserModel>,
) -> Result<(StatusCode, Json<UserModel>), AppError> {
    let now = Utc::now().naive_utc();

    let mut user: UserActiveModel = User::find_by_id(id).one(&connection).await?.unwrap().into();

    user.first_name = Set(payload.first_name);
    user.second_name = Set(payload.second_name);
    user.nick_name = Set(payload.nick_name);
    user.updated_at = Set(now);

    let user = user.update(&connection).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete(
    State(connection): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = User::delete_by_id(id).exec(&connection).await?;

    Ok((StatusCode::NO_CONTENT, "User Deleted".to_owned()))
}
