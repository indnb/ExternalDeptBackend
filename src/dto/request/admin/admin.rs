use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginAdminData {
    pub admin_password: String,
}
