use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Debug, Queryable)]
#[diesel(table_name = crate::diesel::schema::news)]
pub struct NewsQueryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub preview_id: Option<i32>,
    #[allow(dead_code)]
    pub header: String,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::diesel::schema::news)]
pub struct NewsInsertable<'a> {
    #[allow(dead_code)]
    pub description: &'a str,
    #[allow(dead_code)]
    pub header: &'a str,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
