use crate::diesel::models::hackathon_2025::category::HackathonCategory2025Enum;
use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Insertable;
use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TeamUpdateData {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Team 2")]
    pub name: String,
    #[schema(example = "Military")]
    pub category: HackathonCategory2025Enum,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(title = "HackathonTeam2025Insertable", value_type = HackathonTeam2025Insertable, as = HackathonTeam2025Insertable)]
pub struct NewTeam(pub HackathonTeam2025Insertable);

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewTeamWithPersons {
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
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
