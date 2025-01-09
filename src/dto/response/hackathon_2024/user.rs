use crate::diesel::models::hackathon_2024::user::HackathonUser2024Queryable;

#[derive(serde::Serialize)]
pub struct User(pub HackathonUser2024Queryable);

#[derive(serde::Serialize)]
pub struct VecUser(pub Vec<HackathonUser2024Queryable>);
