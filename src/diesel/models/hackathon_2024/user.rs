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
    pub nickname_tg: String,
    #[allow(dead_code)]
    pub phone: String,
    #[allow(dead_code)]
    pub university_id: Option<i32>,
    #[allow(dead_code)]
    pub team_id: Option<i32>,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = crate::diesel::schema::hackathon_user_2024)]
pub struct HackathonUser2024Insertable {
    pub first_name: String,
    pub last_name: String,
    pub nickname_tg: String,
    pub phone: String,
    pub university_id: i32,
    pub team_id: i32,
}
