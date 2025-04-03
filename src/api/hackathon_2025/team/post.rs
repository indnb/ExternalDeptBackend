use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Insertable;
use crate::diesel::models::hackathon_2025::team_captain::HackathonTeamCaptain2025Insertable;
use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::diesel::prelude::get_connection;
use crate::dto::request::hackathon_2025::team::{NewTeam, NewTeamWithPersons};
use crate::dto::response::hackathon_2025::university::UNIVERSITY_ALL_CACHED;
use crate::utils::prelude_api::*;
use crate::utils::validation::data::fields::check_name;
use crate::utils::validation::data::hackathon_2025::team::check_team_members_count;
use diesel::Connection;
use rocket::post;
use std::error::Error;

#[utoipa::path(
    post,
    path = "/api/hackathon_2025/team/registration",
    request_body = NewTeamWithPersons,
    tag = "Hackathon Team 2025",
    operation_id = "registration_team",
    responses(
        (status = 200, description = "Team registrated successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[post("/hackathon_2025/team/registration", data = "<data>")]
pub async fn registration(
    db_pool: &DbState,
    data: Json<NewTeamWithPersons>,
) -> Result<(), ApiError> {
    let new_team = data.into_inner();

    let mut connection = get_connection(db_pool)?;

    check_team_members_count(new_team.members.as_ref().map(|vev| vev.len()).unwrap_or(0))?;
    check_name(
        &new_team.team.name,
        30,
        format!("Team name greater for {} symbol", 30).as_str(),
    )?;

    async fn user_validate(user: &HackathonUser2025Insertable) -> Result<(), ApiError> {
        crate::utils::validation::data::hackathon_2025::user::field(user)?;

        let university = UNIVERSITY_ALL_CACHED.read().await;

        if university.get(&user.university_id).is_none() {
            return Err(ApiError::InvalidUniversityId(
                "University not found".to_string(),
            ));
        }
        Ok(())
    }

    for member in new_team.members.as_ref().unwrap_or(&vec![]).iter() {
        user_validate(&member.clone().into()).await?;
    }

    let mut captain = new_team.captain.into();

    user_validate(&captain).await?;

    let insert_team = new_team.team.into();

    let id = connection
        .transaction::<_, Box<dyn Error>, _>(|tx| {
            let team_id =
                crate::diesel::utils::hackathon_2025::team::insert::new_tx(tx, insert_team)?;

            captain.team_id = team_id;

            let captain_id =
                crate::diesel::utils::hackathon_2025::user::insert::new_tx(tx, captain)?;

            crate::diesel::utils::hackathon_2025::team_captain::insert::new_tx(
                tx,
                HackathonTeamCaptain2025Insertable {
                    team_id,
                    captain_id,
                },
            )?;

            new_team
                .members
                .unwrap_or_default()
                .into_iter()
                .map(|m| {
                    let mut member: HackathonUser2025Insertable = m.into();
                    member.team_id = team_id;
                    member
                })
                .try_for_each(|member| -> Result<(), ApiError> {
                    crate::diesel::utils::hackathon_2025::user::insert::new_tx(tx, member)?;
                    Ok(())
                })?;

            Ok(team_id)
        })
        .map_err(|err| {
            log::error!("Failed to create team: {}", err);
            ApiError::FailedTransaction("Failed to create team".to_string())
        })?;

    info!("Succeed insert new hackathon 2025 team with id, {}", id);

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/hackathon_2025/team/create",
    request_body = NewTeam,
    tag = "Hackathon Team 2025",
    operation_id = "create_team",
    responses(
        (status = 200, description = "Team created successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/hackathon_2025/team/create", data = "<data>")]
pub async fn create(
    pool: &DbState,
    data: Json<NewTeam>,
    admin_match: AdminAuthData,
) -> Result<(), ApiError> {
    admin_match.check_admin()?;

    let new_team: HackathonTeam2025Insertable = data.into_inner().into();

    check_name(
        &new_team.name,
        30,
        format!("Team name greater for {} symbol", 30).as_str(),
    )?;

    let id = crate::diesel::utils::hackathon_2025::team::insert::new(pool, new_team)?;

    info!("Succeed insert new hackathon 2025 team with id, {}", id);

    Ok(())
}
