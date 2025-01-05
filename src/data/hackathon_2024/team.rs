use serde::Deserialize;

#[derive(Deserialize)]
pub struct TeamRegistrationData {
    pub id: i32,
    pub password: String,
}
