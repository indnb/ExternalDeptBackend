#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct University(
    pub crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable,
);

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct VecUniversity(
    pub Vec<crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Queryable>,
);
