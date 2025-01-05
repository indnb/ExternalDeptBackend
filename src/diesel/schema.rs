// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hackathon_category_2024"))]
    pub struct HackathonCategory2024;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "type_media"))]
    pub struct TypeMedia;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TypeMedia;

    announcement_banner (id) {
        id -> Int4,
        src_url -> Text,
        type_media -> TypeMedia,
        #[max_length = 255]
        description -> Varchar,
        showing -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HackathonCategory2024;

    hackathon_team_2024 (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        category -> HackathonCategory2024,
        #[max_length = 255]
        password_registration -> Varchar,
        count_members -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hackathon_university_2024 (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hackathon_user_2024 (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        university -> Nullable<Int4>,
        team_id -> Int4,
    }
}

diesel::table! {
    news (id) {
        id -> Int4,
        description -> Text,
        preview_id -> Nullable<Int4>,
        #[max_length = 255]
        header -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TypeMedia;

    news_media (id) {
        id -> Int4,
        src_url -> Text,
        news_id -> Nullable<Int4>,
        type_media -> TypeMedia,
        position -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(hackathon_user_2024 -> hackathon_team_2024 (team_id));
diesel::joinable!(hackathon_user_2024 -> hackathon_university_2024 (university));
diesel::joinable!(news_media -> news (news_id));

diesel::allow_tables_to_appear_in_same_query!(
    announcement_banner,
    hackathon_team_2024,
    hackathon_university_2024,
    hackathon_user_2024,
    news,
    news_media,
);
