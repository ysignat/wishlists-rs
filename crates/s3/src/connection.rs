use aws_sdk_s3::{
    config::{Credentials, Region},
    Client,
    Config,
};

pub struct S3ConnectOptions {
    access_key_id: String,
    secret_access_key: String,
    endpoint_url: String,
    region: String,
    force_path_style: bool,
}

impl S3ConnectOptions {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        endpoint_url: String,
        region: String,
        force_path_style: bool,
    ) -> Self {
        S3ConnectOptions {
            access_key_id,
            secret_access_key,
            endpoint_url,
            region,
            force_path_style,
        }
    }
}

pub struct Connection;

impl Connection {
    pub fn client(connect_options: S3ConnectOptions) -> Client {
        let creds = Credentials::new(
            connect_options.access_key_id,
            connect_options.secret_access_key,
            None,
            None,
            "",
        );
        let config = Config::builder()
            .endpoint_url(connect_options.endpoint_url)
            .credentials_provider(creds)
            .region(Region::new(connect_options.region))
            .force_path_style(connect_options.force_path_style)
            .build();

        Client::from_conf(config)
    }
}
