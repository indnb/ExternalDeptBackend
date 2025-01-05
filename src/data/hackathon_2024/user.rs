#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserJwt {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub team_id: i32,
    pub university: i32,
    pub exp: usize,
}
