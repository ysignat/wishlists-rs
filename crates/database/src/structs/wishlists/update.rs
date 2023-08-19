use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub name: String,
}
