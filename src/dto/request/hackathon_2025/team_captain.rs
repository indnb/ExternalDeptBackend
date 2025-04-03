use crate::diesel::models::hackathon_2025::team_captain::HackathonTeamCaptain2025Insertable;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct TeamCaptain {
    #[schema(example = "1")]
    pub team_id: i32,
    #[schema(example = "1")]
    pub captain_id: i32,
}

impl From<TeamCaptain> for HackathonTeamCaptain2025Insertable {
    fn from(captain: TeamCaptain) -> Self {
        Self {
            team_id: captain.team_id,
            captain_id: captain.captain_id,
        }
    }
}
