use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use entities::items::Entity;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::utils::{AppError, AppState};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    let _ = Entity::delete_by_id(id)
        .exec(&state.postgres_connection)
        .await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}
