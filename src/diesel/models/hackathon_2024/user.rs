use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2024)]
pub struct HackathonUser2024Queryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub first_name: String,
    #[allow(dead_code)]
    pub last_name: String,
    #[allow(dead_code)]
    pub email: String,
    #[allow(dead_code)]
    pub phone: String,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub university: Option<i32>,
    #[allow(dead_code)]
    pub team_id: i32,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2024)]
pub struct HackathonUser2024Insertable<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub phone: &'a str,
    pub university: i32,
    pub team_id: i32,
}
