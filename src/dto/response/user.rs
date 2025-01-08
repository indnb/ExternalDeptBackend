use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub token: String,
}
