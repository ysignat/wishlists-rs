use axum::{
    extract::{Path, State as AxumState},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{errors::AppError, state::State};

pub async fn handler(
    AxumState(state): AxumState<State>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, String), AppError> {
    state.repository.delete_wishlist(id).await?;

    Ok((StatusCode::NO_CONTENT, "Object removed".to_owned()))
}
