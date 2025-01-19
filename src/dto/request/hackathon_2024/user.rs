use crate::diesel::models::hackathon_2024::user::HackathonUser2024Insertable;
use crate::dto::request::hackathon_2024::team::TeamRegistrationData;
use serde::Deserialize;
use utoipa::ToSchema;

#[non_exhaustive]
#[derive(Deserialize, ToSchema)]
pub struct RegistrationData {
    pub user_data: HackathonUser2024Insertable,
    pub team_data: TeamRegistrationData,
}

#[derive(serde::Deserialize, ToSchema)]
#[schema(title = "HackathonUser2024Insertable", value_type = HackathonUser2024Insertable, as = HackathonUser2024Insertable)]
pub struct User(pub HackathonUser2024Insertable);

#[derive(serde::Deserialize, ToSchema)]
#[schema(title = "VecHackathonUser2024Insertable", value_type = Vec<HackathonUser2024Insertable>, as = Vec<HackathonUser2024Insertable>)]
pub struct VecUser(#[allow(dead_code)] pub Vec<HackathonUser2024Insertable>);
