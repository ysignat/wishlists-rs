use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabasePayload {
    pub first_name: Option<String>,
    pub second_name: Option<String>,
    pub nick_name: String,
}
