use crate::diesel::models::info_data::type_media::TypeMediaEnum;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Debug, Queryable)]
#[diesel(table_name = crate::diesel::schema::announcement_banner)]
pub struct BannerQueryable {
    #[allow(dead_code)]
    pub id: i32,
    #[allow(dead_code)]
    pub src_url: String,
    #[allow(dead_code)]
    pub type_media: TypeMediaEnum,
    #[allow(dead_code)]
    pub description: String,
    #[allow(dead_code)]
    pub showing: bool,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::diesel::schema::announcement_banner)]
pub struct BannerInsertable<'a> {
    #[allow(dead_code)]
    pub src_url: &'a str,
    #[allow(dead_code)]
    pub type_media: TypeMediaEnum,
    #[allow(dead_code)]
    pub description: &'a str,
    #[allow(dead_code)]
    pub showing: bool,
    #[allow(dead_code)]
    pub created_at: Option<NaiveDateTime>,
    #[allow(dead_code)]
    pub updated_at: Option<NaiveDateTime>,
}
