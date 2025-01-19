#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Team(pub crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable);

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct VecTeam(
    pub Vec<crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable>,
);
