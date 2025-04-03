use serde::Serialize;
use utoipa::ToSchema;

use crate::diesel::models::hackathon_2025::team_captain::HackathonTeamCaptain2025Queryable;

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonTeamCaptain2025Queryable", value_type = HackathonTeamCaptain2025Queryable, as = HackathonTeamCaptain2025Queryable)]
pub struct TeamCaptainResponse(pub HackathonTeamCaptain2025Queryable);

#[derive(Serialize, ToSchema)]
#[schema(title = "HackathonTeamCaptain2025Queryable", value_type = Vec<HackathonTeamCaptain2025Queryable>, as = Vec<HackathonTeamCaptain2025Queryable>)]
pub struct VecTeamCaptainResponse(pub Vec<HackathonTeamCaptain2025Queryable>);
