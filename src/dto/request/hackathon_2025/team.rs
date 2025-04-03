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
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct NewTeam {
    #[schema(example = "1")]
    pub name: String,
    #[schema(example = "Education")]
    pub category: HackathonCategory2025Enum,
}

impl Into<HackathonTeam2025Insertable> for NewTeam {
    fn into(self) -> HackathonTeam2025Insertable {
        HackathonTeam2025Insertable {
            name: self.name,
            category: self.category,
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

impl Into<HackathonUser2025Insertable> for Captain {
    fn into(self) -> HackathonUser2025Insertable {
        HackathonUser2025Insertable {
            first_name: self.first_name,
            last_name: self.last_name,
            nickname_tg: Some(self.nickname_tg),
            phone: Some(self.phone),
            university_id: self.university_id,
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

impl Into<HackathonUser2025Insertable> for Member {
    fn into(self) -> HackathonUser2025Insertable {
        HackathonUser2025Insertable {
            first_name: self.first_name,
            last_name: self.last_name,
            nickname_tg: self.nickname_tg,
            phone: self.phone,
            university_id: self.university_id,
            team_id: 0,
        }
    }
}
