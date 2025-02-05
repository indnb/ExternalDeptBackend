use serde::Deserialize;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginAdminData {
    #[schema(example = "P@SSW0RD")]
    pub admin_password: String,

    #[schema(example = "L0g1N")]
    pub admin_name: String,
}
