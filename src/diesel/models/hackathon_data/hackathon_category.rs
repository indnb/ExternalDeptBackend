use crate::diesel::schema::sql_types::HackathonCategory;
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
#[diesel(sql_type = crate::diesel::schema::sql_types::HackathonCategory)]
pub enum HackathonCategoryEnum {
    Education,
    Military,
    Web3,
    Cybersecurity,
}
impl ToSql<HackathonCategory, Pg> for HackathonCategoryEnum {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            HackathonCategoryEnum::Education => out.write_all(b"education")?,
            HackathonCategoryEnum::Military => out.write_all(b"military")?,
            HackathonCategoryEnum::Web3 => out.write_all(b"web3_0")?,
            HackathonCategoryEnum::Cybersecurity => out.write_all(b"cybersecurity")?,
        };
        Ok(IsNull::No)
    }
}
impl FromSql<HackathonCategory, Pg> for HackathonCategoryEnum {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = std::str::from_utf8(bytes.as_bytes())?;
        match value {
            "education" => Ok(HackathonCategoryEnum::Education),
            "military" => Ok(HackathonCategoryEnum::Military),
            "web3_0" => Ok(HackathonCategoryEnum::Web3),
            "cybersecurity" => Ok(HackathonCategoryEnum::Cybersecurity),
            _ => {
                info!(
                    "Not a valid hackathon category: {}, default set 'education'",
                    value
                );
                Ok(HackathonCategoryEnum::Education)
            }
        }
    }
}
