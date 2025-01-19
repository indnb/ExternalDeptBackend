#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct University(
    pub crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable,
);
