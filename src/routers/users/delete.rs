use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_user(id).await.unwrap();

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}
