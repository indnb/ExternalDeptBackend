use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserLoginRequest {
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub password: String,
}
