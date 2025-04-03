// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hackathon_category_2025"))]
    pub struct HackathonCategory2025;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HackathonCategory2025;

    hackathon_team_2025 (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        category -> HackathonCategory2025,
        count_members -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hackathon_team_captain_2025 (team_id) {
        team_id -> Int4,
        captain_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hackathon_university_2025 (id) {
        id -> Int4,
        name -> Text,
        name_eng -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hackathon_user_2025 (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        nickname_tg -> Nullable<Varchar>,
        university_id -> Int4,
        team_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(hackathon_team_captain_2025 -> hackathon_team_2025 (team_id));
diesel::joinable!(hackathon_team_captain_2025 -> hackathon_user_2025 (captain_id));
diesel::joinable!(hackathon_user_2025 -> hackathon_team_2025 (team_id));
diesel::joinable!(hackathon_user_2025 -> hackathon_university_2025 (university_id));

diesel::allow_tables_to_appear_in_same_query!(
    hackathon_team_2025,
    hackathon_team_captain_2025,
    hackathon_university_2025,
    hackathon_user_2025,
);
