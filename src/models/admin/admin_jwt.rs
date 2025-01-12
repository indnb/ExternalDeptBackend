use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminJwt {
    pub admin_password: String,
    pub admin_name: String,
    pub exp: u64,
}
