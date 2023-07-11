mod handlers;
mod structs;
mod utils;

use axum::{routing::get, Router};
use handlers::{
    users::{create_user, delete_user, get_user, list_users, update_user},
    wishlists::{create_wishlist, delete_wishlist, get_wishlist, list_wishlists, update_wishlist},
};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};

// TODO Constraints checks https://github.com/launchbadge/realworld-axum-sqlx/blob/f1b25654773228297e35c292f357d33b7121a101/src/http/users.rs#L80
// TODO Tests
// TODO Dockerfile
// TODO Helm Chart
// TODO SwaggerUI
// TODO Prometheus
// TODO Oauth2
// TODO Migrations Automation
// TODO CI
// TODO Configuration from env
// TODO Logging

#[tokio::main]
async fn main() {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/wishlists", get(list_wishlists).post(create_wishlist))
        .route(
            "/wishlists/:id",
            get(get_wishlist)
                .put(update_wishlist)
                .delete(delete_wishlist),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
