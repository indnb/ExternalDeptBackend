use crate::diesel::models::hackathon_2024::category::HackathonCategory2024Enum;
use crate::diesel::models::hackathon_2024::team::HackathonTeam2024Insertable;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct TeamRegistrationData {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "real_password123!AAA")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TeamUpdateData {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Team 2")]
    pub name: String,
    #[schema(example = "Military")]
    pub category: HackathonCategory2024Enum,
    #[schema(example = "bredovschik")]
    pub nickname_tg: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(title = "HackathonTeam2024Insertable", value_type = HackathonTeam2024Insertable, as = HackathonTeam2024Insertable)]
pub struct TeamCreateData(pub HackathonTeam2024Insertable);
