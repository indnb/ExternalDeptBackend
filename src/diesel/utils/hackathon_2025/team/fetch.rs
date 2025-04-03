use std::collections::HashMap;

use crate::diesel::models::hackathon_2025::team::HackathonTeam2025Queryable;
use crate::diesel::models::hackathon_2025::user::HackathonUser2025Queryable;
use crate::diesel::prelude::*;
use crate::diesel::schema::hackathon_team_2025::dsl::hackathon_team_2025;
use crate::diesel::schema::hackathon_user_2025::dsl::hackathon_user_2025;
use crate::diesel::schema::hackathon_user_2025::{id as user_id, team_id as user_team_id};
use crate::diesel::utils::hackathon_2025::{team_captain, user};
use crate::dto::response::hackathon_2025::team::FullTeam;

pub fn by_id(
    connection: &mut DbPooled,
    team_id: i32,
) -> Result<HackathonTeam2025Queryable, ApiError> {
    hackathon_team_2025
        .filter(crate::diesel::schema::hackathon_team_2025::columns::id.eq(team_id))
        .first::<HackathonTeam2025Queryable>(connection)
        .map_err(|err| ApiError::FailedToGetTeamById(err.to_string()))
}

pub fn all(connection: &mut DbPooled) -> Result<Vec<HackathonTeam2025Queryable>, ApiError> {
    hackathon_team_2025
        .load::<HackathonTeam2025Queryable>(connection)
        .map_err(|err| ApiError::FailedToGetAllTeams(err.to_string()))
}

pub fn by_id_full(connection: &mut DbPooled, team_id: i32) -> Result<FullTeam, ApiError> {
    let team = by_id(connection, team_id)?;

    let captain = team_captain::fetch::by_team_id(connection, team.id)?;

    let captain = user::fetch::by_id(connection, captain.captain_id)?;

    let members = hackathon_user_2025
        .filter(user_id.ne(captain.id))
        .filter(user_team_id.eq(team.id))
        .load::<HackathonUser2025Queryable>(connection)
        .map_err(|err| ApiError::FailedToGetUsersByTeam(err.to_string()))?;

    Ok(FullTeam {
        id: team.id,
        name: team.name,
        category: team.category,
        count_members: team.count_members,
        created_at: team.created_at,
        updated_at: team.updated_at,
        captain,
        members,
    })
}

pub fn all_full(connection: &mut DbPooled) -> Result<Vec<FullTeam>, ApiError> {
    let teams = all(connection)?;

    let captains = team_captain::fetch::all(connection)?;
    let captain_map: HashMap<i32, i32> = captains
        .into_iter()
        .map(|c| (c.team_id, c.captain_id))
        .collect();

    let users: Vec<HackathonUser2025Queryable> = user::fetch::all(connection)?;

    let mut user_by_id: HashMap<i32, HackathonUser2025Queryable> = HashMap::new();
    let mut users_by_team: HashMap<i32, Vec<HackathonUser2025Queryable>> = HashMap::new();

    for user in users {
        user_by_id.insert(user.id, user.clone());
        users_by_team.entry(user.team_id).or_default().push(user);
    }

    let result: Vec<FullTeam> = teams
        .into_iter()
        .filter_map(|team| {
            let captain_id = captain_map.get(&team.id)?;
            let captain = user_by_id.get(captain_id)?.clone();
            let mut members = users_by_team.get(&team.id).cloned().unwrap_or_default();
            members.retain(|user| user.id != captain.id);

            Some(FullTeam {
                id: team.id,
                name: team.name,
                category: team.category,
                count_members: team.count_members,
                created_at: team.created_at,
                updated_at: team.updated_at,
                captain,
                members,
            })
        })
        .collect();

    Ok(result)
}
