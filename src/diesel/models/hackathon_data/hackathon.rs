use crate::diesel::models::hackathon_data::hackathon_category::HackathonCategoryEnum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Debug, Queryable)]
#[diesel(table_name = crate::diesel::schema::hackathon)]
pub struct HackathonQueryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub user_id: i32,
    #[allow(dead_code)]
    pub category: HackathonCategoryEnum,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::diesel::schema::hackathon)]
pub struct HackathonInsertable {
    #[allow(dead_code)]
    pub user_id: i32,
    #[allow(dead_code)]
    pub category: HackathonCategoryEnum,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
