use crate::diesel::models::hackathon_2025::category::HackathonCategory2025Enum;
use crate::diesel::models::hackathon_2025::team::HackathonTeam2024Insertable;
use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
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
    pub category: HackathonCategory2025Enum,
    #[schema(example = "bredovschik")]
    pub nickname_tg: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(title = "HackathonTeam2024Insertable", value_type = HackathonTeam2024Insertable, as = HackathonTeam2024Insertable)]
pub struct TeamCreateData(pub HackathonTeam2024Insertable);

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewTeam {
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
    #[schema(example = "skalse_456")]
    pub lider_name: String,
    #[schema(example = "Skalse")]
    pub captain_first_name: String,
    #[schema(example = "Batya")]
    pub captain_last_name: String,
    #[schema(example = "skalse_456")]
    pub captain_nickname_tg: String,
    #[schema(example = "0123456789")]
    pub captain_phone: String,
    #[schema(example = "1")]
    pub captain_university_id: i32,
    pub hackathon_users: Vec<HackathonUser2025Insertable>,
}
