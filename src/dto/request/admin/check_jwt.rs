use serde::Deserialize;
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct CheckJwtAdminData {
    jwt: String,
}
