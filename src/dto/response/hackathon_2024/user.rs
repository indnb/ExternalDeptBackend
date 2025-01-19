use crate::diesel::models::hackathon_2024::user::HackathonUser2024Queryable;
use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct User(pub HackathonUser2024Queryable);

#[derive(serde::Serialize, ToSchema)]
pub struct VecUser(pub Vec<HackathonUser2024Queryable>);
