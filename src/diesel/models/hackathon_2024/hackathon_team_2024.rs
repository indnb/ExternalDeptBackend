use crate::diesel::models::hackathon_2024::hackathon_category_2024::HackathonCategory2024Enum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2024)]
pub struct HackathonTeam2024Queryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub category: HackathonCategory2024Enum,
    #[allow(dead_code)]
    pub password_registration: String,
    #[allow(dead_code)]
    pub count_members: i32,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_team_2024)]
pub struct HackathonTeam2024Insertable<'a> {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub name: &'a str,
    #[allow(dead_code)]
    pub category: HackathonCategory2024Enum,
    #[allow(dead_code)]
    pub password_registration: &'a str,
    #[allow(dead_code)]
    pub count_members: i32,
}
