use std::error::Error;

use crate::diesel::models::hackathon_2025::team::HackathonTeam2024Insertable;
use crate::diesel::models::hackathon_2025::user::HackathonUser2025Insertable;
use crate::diesel::prelude::get_connection;
use crate::dto::request::hackathon_2025::team::{NewTeam, TeamCreateData};
use crate::dto::response::hackathon_2025::university::UNIVERSITY_ALL_CACHED;
use crate::utils::prelude_api::*;
use diesel::Connection;
use rocket::post;

#[utoipa::path(
    post,
    path = "/api/hackathon_2025/team/registration",
    request_body = TeamCreateData,
    tag = "Hackathon Team 2025",
    operation_id = "registration_team",
    responses(
        (status = 200, description = "Team created successfully"),
        (status = 422, description = "Validation error", body = ApiErrorBody),
        (status = 500, description = "Database error", body = ApiErrorBody),
    ),
)]
#[post("/hackathon_2025/team/registration", data = "<data>")]
pub async fn registration(db_pool: &DbState, data: Json<NewTeam>) -> Result<(), ApiError> {
    let new_team = data.into_inner();

    let mut connection = get_connection(db_pool)?;

    for member in new_team.hackathon_users.iter() {
        user_validate(member).await?;
    }

    let mut captain_user = HackathonUser2025Insertable {
        first_name: new_team.captain_first_name,
        last_name: new_team.captain_last_name,
        nickname_tg: Some(new_team.captain_nickname_tg.clone()),
        phone: Some(new_team.captain_phone),
        university_id: new_team.captain_university_id,
        team_id: 0,
    };

    user_validate(&captain_user).await?;

    let insert_team = HackathonTeam2024Insertable {
        name: new_team.name,
        category: new_team.category,
    };

    let id = connection
        .transaction::<_, Box<dyn Error>, _>(|tx| {
            let id = crate::diesel::utils::hackathon_2025::team::insert::new(tx, insert_team)?;

            captain_user.team_id = id;

            crate::diesel::utils::hackathon_2025::user::insert::new_tx(tx, captain_user)?;

            for member in new_team.hackathon_users.into_iter() {
                crate::diesel::utils::hackathon_2025::user::insert::new_tx(tx, member)?;
            }

            Ok(id)
        })
        .map_err(|_| ApiError::FailedTransaction("Failed to create team".to_string()))?;

    info!("Succeed insert new hackathon 2025 team with id, {}", id);

    Ok(())
}

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
