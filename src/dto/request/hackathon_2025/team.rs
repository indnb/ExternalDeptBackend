use crate::diesel::models::hackathon_2025::{
    category::HackathonCategory2025Enum, team::HackathonTeam2025Insertable,
    user::HackathonUser2025Insertable,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTeam {
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "Team 2")]
    pub name: String,
    #[schema(example = "Military")]
    pub category: HackathonCategory2025Enum,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewTeamWithPersons {
    pub team: NewTeam,
    pub captain: Captain,
    pub members: Option<Vec<Member>>,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct NewTeam {
    #[schema(example = "Team 1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
}

impl From<NewTeam> for HackathonTeam2025Insertable {
    fn from(team: NewTeam) -> Self {
        Self {
            name: team.name,
            category: team.category,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct Captain {
    #[schema(example = "Skalse")]
    pub first_name: String,
    #[schema(example = "Batya")]
    pub last_name: String,
    #[schema(example = "skalse_456")]
    pub nickname_tg: String,
    #[schema(example = "0669153459")]
    pub phone: String,
    #[schema(example = "1")]
    pub university_id: i32,
}

impl From<Captain> for HackathonUser2025Insertable {
    fn from(captain: Captain) -> Self {
        Self {
            first_name: captain.first_name,
            last_name: captain.last_name,
            nickname_tg: Some(captain.nickname_tg),
            phone: Some(captain.phone),
            university_id: captain.university_id,
            team_id: 0,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct Member {
    #[schema(example = "YaCode")]
    pub first_name: String,
    #[schema(example = "YaError")]
    pub last_name: String,
    #[schema(example = "ya_code")]
    pub nickname_tg: Option<String>,
    #[schema(example = "0923456789")]
    pub phone: Option<String>,
    #[schema(example = "1")]
    pub university_id: i32,
}

impl From<Member> for HackathonUser2025Insertable {
    fn from(member: Member) -> Self {
        Self {
            first_name: member.first_name,
            last_name: member.last_name,
            nickname_tg: member.nickname_tg,
            phone: member.phone,
            university_id: member.university_id,
            team_id: 0,
        }
    }
}
