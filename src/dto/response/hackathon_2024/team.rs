#[derive(serde::Serialize)]
pub struct Team(pub crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable);

#[derive(serde::Serialize)]
pub struct VecTeam(
    pub Vec<crate::diesel::models::hackathon_2024::team::HackathonTeam2024Queryable>,
);
