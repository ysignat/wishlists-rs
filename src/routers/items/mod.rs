use axum::{
    async_trait,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
    Router,
};
use uuid::Uuid;

use crate::utils::{AppError, AppState};

mod create;
mod delete;
mod get;
mod list;
mod update;

static SUBPATH: &str = "/items";

#[async_trait]
trait ItemsRepositoryTrait {
    async fn create(
        state: State<AppState>,
        payload: Json<create::Payload>,
    ) -> Result<(StatusCode, Json<create::Response>), AppError>;

    async fn delete(
        state: State<AppState>,
        id: Path<Uuid>,
    ) -> Result<(StatusCode, String), AppError>;

    async fn get(
        state: State<AppState>,
        id: Path<Uuid>,
    ) -> Result<(StatusCode, Json<get::Response>), AppError>;

    async fn list(
        state: State<AppState>,
    ) -> Result<(StatusCode, Json<Vec<list::Response>>), AppError>;

    async fn update(
        state: State<AppState>,
        id: Path<Uuid>,
        payload: Json<update::Payload>,
    ) -> Result<(StatusCode, Json<update::Response>), AppError>;
}

struct ItemsRepository;

#[async_trait]
impl ItemsRepositoryTrait for ItemsRepository {
    async fn create(
        state: State<AppState>,
        payload: Json<create::Payload>,
    ) -> Result<(StatusCode, Json<create::Response>), AppError> {
        create::handler(state, payload).await
    }

    async fn delete(
        state: State<AppState>,
        id: Path<Uuid>,
    ) -> Result<(StatusCode, String), AppError> {
        delete::handler(state, id).await
    }

    async fn get(
        state: State<AppState>,
        id: Path<Uuid>,
    ) -> Result<(StatusCode, Json<get::Response>), AppError> {
        get::handler(state, id).await
    }

    async fn list(
        state: State<AppState>,
    ) -> Result<(StatusCode, Json<Vec<list::Response>>), AppError> {
        list::handler(state).await
    }

    async fn update(
        state: State<AppState>,
        id: Path<Uuid>,
        payload: Json<update::Payload>,
    ) -> Result<(StatusCode, Json<update::Response>), AppError> {
        update::handler(state, id, payload).await
    }
}

pub fn get_router(root_path: &str, state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{root_path}{SUBPATH}"),
            get(ItemsRepository::list).post(ItemsRepository::create),
        )
        .route(
            &format!("{root_path}{SUBPATH}/:id"),
            get(ItemsRepository::get)
                .put(ItemsRepository::update)
                .delete(ItemsRepository::delete),
        )
        .with_state(state)
}
