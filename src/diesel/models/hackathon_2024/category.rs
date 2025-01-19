use crate::diesel::schema::sql_types::HackathonCategory2024;
use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::{serialize, FromSqlRow};
use log::info;
use serde::{Deserialize, Serialize};
use serialize::IsNull;
use std::fmt::Debug;
use std::io::Write;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    FromSqlRow,
    AsExpression,
    Deserialize,
    Serialize,
    utoipa::ToSchema,
)]
#[diesel(sql_type = crate::diesel::schema::sql_types::HackathonCategory2024)]
pub enum HackathonCategory2024Enum {
    Education,
    Military,
    Web3,
    Cybersecurity,
}
impl ToSql<HackathonCategory2024, Pg> for HackathonCategory2024Enum {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            HackathonCategory2024Enum::Education => out.write_all(b"education")?,
            HackathonCategory2024Enum::Military => out.write_all(b"military")?,
            HackathonCategory2024Enum::Web3 => out.write_all(b"web3_0")?,
            HackathonCategory2024Enum::Cybersecurity => out.write_all(b"cybersecurity")?,
        };
        Ok(IsNull::No)
    }
}
impl FromSql<HackathonCategory2024, Pg> for HackathonCategory2024Enum {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let value = std::str::from_utf8(bytes.as_bytes())?;
        match value {
            "education" => Ok(HackathonCategory2024Enum::Education),
            "military" => Ok(HackathonCategory2024Enum::Military),
            "web3_0" => Ok(HackathonCategory2024Enum::Web3),
            "cybersecurity" => Ok(HackathonCategory2024Enum::Cybersecurity),
            _ => {
                info!(
                    "Not a valid hackathon category: {}, default set 'education'",
                    value
                );
                Ok(HackathonCategory2024Enum::Education)
            }
        }
    }
}
