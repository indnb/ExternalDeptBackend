// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hackathon_category"))]
    pub struct HackathonCategory;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "type_media"))]
    pub struct TypeMedia;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
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
    use super::sql_types::HackathonCategory;

    hackathon (id) {
        id -> Int4,
        user_id -> Int4,
        category -> HackathonCategory,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        role -> UserRole,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(hackathon -> users (user_id));
diesel::joinable!(news_media -> news (news_id));

diesel::allow_tables_to_appear_in_same_query!(
    announcement_banner,
    hackathon,
    news,
    news_media,
    users,
);
