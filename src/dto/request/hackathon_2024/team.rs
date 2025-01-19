use crate::diesel::models::hackathon_2024::category::HackathonCategory2024Enum;
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use serde::Deserialize;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct TeamRegistrationData {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "password")]
    pub password: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct TeamUpdateData {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Innovation University")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2024Enum,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct TeamCreateData(pub HackathonTeam2024Insertable);
