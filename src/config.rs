use clap::Parser;

#[derive(Parser)]
#[command(
    name = "wishlists",
    bin_name = "wishlists",
    about = "Sample Rust Axum app",
    author = "ysignat"
)]
pub struct Config {
    #[arg(
        long,
        env = "WISHLISTS__MIGRATE",
        default_value = "false",
        help = "Run migrations with `--postgres-url` and exit"
    )]
    pub migrate: bool,
    #[arg(
        long,
        env = "WISHLISTS__POSTGRES_URL",
        default_value = "postgres://postgres:postgres@postgres:5432/postgres",
        help = "URL for Postgres connection (postgres://<user>:<password>@<host>:<port>/<database>)"
    )]
    pub postgres_url: String,
    #[arg(
        long,
        env = "WISHLISTS__POSTGRES_POOL_SIZE",
        default_value = "5",
        help = "Postgres connection pool size"
    )]
    pub postgres_pool_size: u32,
    #[arg(
        long,
        env = "WISHLISTS__POSTGRES_POOL_ACQUIRE_TIMEOUT",
        default_value = "3",
        help = "Postgres connection pool acquire timeout"
    )]
    pub postgres_pool_acquire_timeout: u64,
    #[arg(
        long,
        env = "WISHLISTS__APP_ROOT_PATH",
        default_value = "/api",
        help = "Root path for deployment under reverse proxy (must start with '/')"
    )]
    pub app_root_path: String,
    #[arg(
        long,
        env = "WISHLISTS__APP_BIND_ADDRESS",
        default_value = "127.0.0.1:8080",
        help = "Address where app listens to incoming connections (<host>:<port>)"
    )]
    pub app_bind_address: String,
}
