use crate::diesel::{
    models::hackathon_2024::university::HackathonUniversity2024Queryable,
    prelude::{ApiError, DbState},
};
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::RwLock;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Clone)]
#[schema(title = "HackathonUniversity2024Queryable", value_type = HackathonUniversity2024Queryable, as = HackathonUniversity2024Queryable)]
pub struct University(pub HackathonUniversity2024Queryable);

#[derive(Serialize, ToSchema, Clone)]
#[schema(title = "VecHackathonUniversity2024Queryable", value_type = Vec<HackathonUniversity2024Queryable>, as = Vec<HackathonUniversity2024Queryable>)]
pub struct VecUniversity(pub Vec<HackathonUniversity2024Queryable>);

lazy_static::lazy_static! {
    pub static ref UNIVERSITY_ALL_CACHED: RwLock<HashMap<i32, HackathonUniversity2024Queryable>> = RwLock::new(HashMap::new());
}

pub async fn update_university_cached(db_pool: &DbState) -> Result<(), ApiError> {
    let mut university = UNIVERSITY_ALL_CACHED.write().await;

    *university = crate::diesel::utils::hackathon_2024::university::fetch::all(db_pool)?;

    Ok(())
}
