use crate::utils::{AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use chrono::{offset::Utc, NaiveDateTime};
use entities::items::ActiveModel as ItemActiveModel;
use entities::items::Entity as ItemEntity;
use entities::items::Model as ItemModel;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct ItemCreate {
    wishlist_id: Uuid,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

#[derive(Deserialize)]
struct ItemUpdate {
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
}

#[derive(Serialize)]
struct Item {
    id: Uuid,
    wishlist_id: Uuid,
    selected_by_id: Option<Uuid>,
    name: String,
    description: Option<String>,
    price: Option<i32>,
    is_hidden: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<ItemModel> for Item {
    fn from(value: ItemModel) -> Self {
        Item {
            id: value.id,
            wishlist_id: value.wishlist_id,
            selected_by_id: value.selected_by_id,
            name: value.name,
            description: value.description,
            price: value.price,
            is_hidden: value.is_hidden,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{root_path}/items"),
            axum::routing::get(list).post(create),
        )
        .route(
            &format!("{root_path}/items/:id"),
            axum::routing::get(get).put(update).delete(delete),
        )
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Item>>), AppError> {
    let items = ItemEntity::find()
        .all(&state.postgres_connection)
        .await?
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    Ok((StatusCode::OK, Json(items)))
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<ItemCreate>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let now = Utc::now().naive_utc();

    let item = ItemActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        wishlist_id: ActiveValue::Set(payload.wishlist_id),
        selected_by_id: ActiveValue::Set(None),
        name: ActiveValue::Set(payload.name),
        description: ActiveValue::Set(payload.description),
        price: ActiveValue::Set(payload.price),
        is_hidden: ActiveValue::Set(payload.is_hidden),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    }
    .insert(&state.postgres_connection)
    .await?
    .into();

    Ok((StatusCode::CREATED, Json(item)))
}

async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let item = ItemEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    Ok((StatusCode::OK, Json(item)))
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ItemUpdate>,
) -> Result<(StatusCode, Json<Item>), AppError> {
    let now = Utc::now().naive_utc();

    let mut item: ItemActiveModel = ItemEntity::find_by_id(id)
        .one(&state.postgres_connection)
        .await?
        .unwrap()
        .into();

    item.name = Set(payload.name);
    item.description = Set(payload.description);
    item.price = Set(payload.price);
    item.is_hidden = Set(payload.is_hidden);
    item.updated_at = Set(now);

    let user = item.update(&state.postgres_connection).await?.into();

    Ok((StatusCode::OK, Json(user)))
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = ItemEntity::delete_by_id(id)
        .exec(&state.postgres_connection)
        .await?;

    Ok((StatusCode::NO_CONTENT, "Item Deleted".to_owned()))
}
