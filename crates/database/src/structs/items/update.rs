use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub is_hidden: bool,
}
