use std::{net::SocketAddr, time::Duration};

use clap::{Args, Parser, Subcommand};
use database::connection::DatabaseConnectOptions;
// use tracing_subscriber::filter::LevelFilter; // TODO: Preparation for logging

const ENV_PREFIX: &str = "WISHLISTS";
const ENV_SEPARATOR: &str = "__";

const DATABASE_ENV_PREFIX: &str = "DATABASE";
const RUN_ENV_PREFIX: &str = "RUN";
// const LOG_ENV_PREFIX: &str = "LOG"; // TODO: Preparation for logging

const LONG_SEPARATOR: &str = "-";
const DATABASE_LONG_PREFIX: &str = "database";
const RUN_LONG_PREFIX: &str = "run";
// const LOG_LONG_PREFIX: &str = "log"; // TODO: Preparation for logging

struct ArgMetadata {
    prefix: Option<String>,
    separator: String,
}

trait Metadata {
    fn metadata() -> ArgMetadata;
}

struct EnvArg;

impl Metadata for EnvArg {
    fn metadata() -> ArgMetadata {
        ArgMetadata {
            prefix: Some(ENV_PREFIX.to_owned()),
            separator: ENV_SEPARATOR.to_owned(),
        }
    }
}

struct LongArg;

impl Metadata for LongArg {
    fn metadata() -> ArgMetadata {
        ArgMetadata {
            prefix: None,
            separator: LONG_SEPARATOR.to_owned(),
        }
    }
}

trait Construct
where
    Self: Metadata,
{
    fn construct(values: &[&str]) -> String {
        let mut res = String::new();
        let metadata = Self::metadata();
        if let Some(prefix) = metadata.prefix {
            res.push_str(&prefix);
            res.push_str(&metadata.separator);
        };
        res.push_str(&values.join(&metadata.separator));
        res
    }
}

impl Construct for EnvArg {}
impl Construct for LongArg {}

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
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"url"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"URL"]),
        default_value = "postgres://postgres:postgres@postgres:5432/postgres",
        help = "The URI of the database",
        global = true
    )]
    url: String,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"max-connections"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"MAX_CONNECTIONS"]),
        help = "Maximum number of connections for a pool",
        global = true
    )]
    max_connections: Option<u32>,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"min-connections"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"MIN_CONNECTIONS"]),
        help = "Minimum number of connections for a pool",
        global = true
    )]
    min_connections: Option<u32>,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"connect-timeout"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"CONNECT_TIMEOUT"]),
        help = "The connection timeout for a packet connection",
        global = true
    )]
    connect_timeout: Option<u64>,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"idle-timeout"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"IDLE_TIMEOUT"]),
        help = "Maximum idle time for a particular connection to prevent network resource exhaustion",
        global = true
    )]
    idle_timeout: Option<u64>,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"acquire-timeout"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"ACQUIRE_TIMEOUT"]),
        help = "Maximum amount of time to spend waiting for acquiring a connection",
        global = true
    )]
    acquire_timeout: Option<u64>,
    #[arg(
        long = LongArg::construct(&[DATABASE_LONG_PREFIX,"max-lifetime"]),
        env = EnvArg::construct(&[DATABASE_ENV_PREFIX,"MAX_LIFETIME"]),
        help = "Maximum lifetime of individual connections",
        global = true
    )]
    max_lifetime: Option<u64>,
}

impl From<DatabaseArgs> for DatabaseConnectOptions {
    fn from(value: DatabaseArgs) -> Self {
        DatabaseConnectOptions {
            url: value.url,
            max_connections: value.max_connections,
            min_connections: value.min_connections,
            connect_timeout: value.connect_timeout.map(Duration::from_secs),
            idle_timeout: value.idle_timeout.map(Duration::from_secs),
            acquire_timeout: value.acquire_timeout.map(Duration::from_secs),
            max_lifetime: value.max_lifetime.map(Duration::from_secs),
        }
    }
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
        long = LongArg::construct(&[RUN_LONG_PREFIX,"root-path"]),
        env = EnvArg::construct(&[RUN_ENV_PREFIX,"ROOT_PATH"]),
        default_value = "/api",
        help = "Root path for deployment under reverse proxy (must start with '/')"
    )]
    pub root_path: RootPath,
    #[arg(
        long = LongArg::construct(&[RUN_LONG_PREFIX,"bind-address"]),
        env = EnvArg::construct(&[RUN_ENV_PREFIX,"BIND_ADDRESS"]),
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
        if value.starts_with('/') {
            RootPath {
                root_path: if value == "/" { String::new() } else { value },
            }
        } else {
            panic!("Root path '{value}' doesn't start with '/'")
        }
    }
}

impl From<RootPath> for String {
    fn from(val: RootPath) -> Self {
        val.root_path
    }
}
