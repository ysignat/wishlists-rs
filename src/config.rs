use std::net::SocketAddr;

use clap::{Args, Parser, Subcommand};
// use tracing_subscriber::filter::LevelFilter; // TODO: Preparation for logging

const ENV_PREFIX: &str = "WISHLISTS__";

const DATABASE_ENV_PREFIX: &str = "DATABASE__";
const RUN_ENV_PREFIX: &str = "RUN__";
// const LOG_ENV_PREFIX: &str = "LOG__"; // TODO: Preparation for logging

fn add_prefix(values: &[&str]) -> String {
    let concatenated = values.concat();
    format!("{ENV_PREFIX}{concatenated}")
}

#[derive(Parser)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub database: DatabaseArgs,
    // #[command(flatten)] // TODO: Preparation for logging
    // logging: LoggingArgs,
}

#[derive(Subcommand, PartialEq, Eq)]
pub enum Commands {
    #[command(about = "Run HTTP server")]
    Run(RunArgs),
    #[command(about = "Run database migrations and exit")]
    Migrate,
}

#[derive(Args)]
pub struct DatabaseArgs {
    #[arg(
        long = "database-url",
        env = add_prefix(&[DATABASE_ENV_PREFIX,"URL"]),
        default_value = "postgres://postgres:postgres@postgres:5432/postgres",
        help = "URL for database connection (postgres://<user>:<password>@<host>:<port>/<database>)",
        global = true
    )]
    pub url: String,
    #[arg(
        long = "database-pool-size",
        env = add_prefix(&[DATABASE_ENV_PREFIX,"SIZE"]),
        default_value = "5",
        help = "Connection pool size",
        global = true
    )]
    pub pool_size: u32,
    #[arg(
        long = "database-acquire-timeout",
        env = add_prefix(&[DATABASE_ENV_PREFIX,"ACQUIRE_TIMEOUT"]),
        default_value = "3",
        help = "TODO: write description",
        global = true
    )]
    pub acquire_timeout: u64,
}

// #[derive(Args)] // TODO: Preparation for logging
// struct LoggingArgs {
// #[command(long, default_value = "INFO", env = add_prefix(&[LOG_ENV_PREFIX,"LEVEL"]), global = true)]
// log_level: LevelFilter,
// #[clap(long, env = add_prefix(&[LOG_ENV_PREFIX,"FORMAT"]), global = true)]
// log_format: LogFormat,
// }

// #[derive(Clone, ValueEnum)]  // TODO: Preparation for logging
// enum LogFormat {
//     Raw,
//     Json,
// }

#[derive(Args, PartialEq, Eq)]
pub struct RunArgs {
    #[arg(
        long = "run-root-path",
        env = add_prefix(&[RUN_ENV_PREFIX,"ROOT_PATH"]),
        default_value = "/api",
        help = "Root path for deployment under reverse proxy (must start with '/')"
    )]
    pub root_path: RootPath,
    #[arg(
        long = "run-bind-address",
        env = add_prefix(&[RUN_ENV_PREFIX,"BIND_ADDRESS"]),
        default_value = "127.0.0.1:8080",
        help = "Address where app listens to incoming connections (<host>:<port>)"
    )]
    pub bind_address: SocketAddr,
}

#[derive(Args, Clone, PartialEq, Eq)]
pub struct RootPath {
    root_path: String,
}

impl From<String> for RootPath {
    fn from(value: String) -> Self {
        RootPath {
            root_path: if value == "/" { String::new() } else { value },
        }
    }
}

impl From<RootPath> for String {
    fn from(val: RootPath) -> Self {
        val.root_path
    }
}
