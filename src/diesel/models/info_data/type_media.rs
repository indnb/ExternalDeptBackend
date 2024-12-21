use crate::diesel::schema::sql_types::TypeMedia;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{serialize, FromSqlRow};
use log::info;
use serde::Deserialize;
use serialize::IsNull;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromSqlRow, AsExpression, Deserialize)]
#[diesel(sql_type = crate::diesel::schema::sql_types::TypeMedia)]
pub enum TypeMediaEnum {
    Video,
    Photo,
}
impl ToSql<TypeMedia, Pg> for TypeMediaEnum {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            TypeMediaEnum::Video => out.write_all(b"video")?,
            TypeMediaEnum::Photo => out.write_all(b"photo")?,
        };
        Ok(IsNull::No)
    }
}
impl FromSql<TypeMedia, Pg> for TypeMediaEnum {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = std::str::from_utf8(bytes.as_bytes())?;
        match value {
            "video" => Ok(TypeMediaEnum::Video),
            "photo" => Ok(TypeMediaEnum::Photo),
            _ => {
                info!("Not a valid type media: {}, default set 'photo'", value);
                Ok(TypeMediaEnum::Photo)
            }
        }
    }
}
