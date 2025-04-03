use crate::api;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // /test/*
        api::test::get::ping,
        // /hackathon_2025/user/*
        api::hackathon_2025::user::post::create,
        api::hackathon_2025::user::get::all,
        api::hackathon_2025::user::put::by_id,
        api::hackathon_2025::user::delete::by_id,
        api::hackathon_2025::user::get::by_id,
        api::hackathon_2025::user::get::by_university,
        api::hackathon_2025::user::get::by_team,
        // /hackathon_2025/university/*
        api::hackathon_2025::university::post::create,
        api::hackathon_2025::university::post::create_by_vec,
        api::hackathon_2025::university::get::all,
        api::hackathon_2025::university::get::by_id,
        api::hackathon_2025::university::put::by_id,
        api::hackathon_2025::university::delete::by_id,
        // /hackathon_2025/team/*
        api::hackathon_2025::team::get::by_id_full,
        api::hackathon_2025::team::get::all_full,
        api::hackathon_2025::team::post::registration,
        api::hackathon_2025::team::post::create,
        api::hackathon_2025::team::get::all,
        api::hackathon_2025::team::get::by_id,
        api::hackathon_2025::team::put::by_data,
        api::hackathon_2025::team::delete::by_id,
        // /hackathon_2025/team_captain/*
        api::hackathon_2025::team_captain::post::by_data,
        api::hackathon_2025::team_captain::put::by_data,
        api::hackathon_2025::team_captain::delete::by_team_id,
        api::hackathon_2025::team_captain::get::by_team_id,
        api::hackathon_2025::team_captain::get::by_captain_id,
        api::hackathon_2025::team_captain::get::all,
        // /admin/
        api::admin::post::login,
        api::admin::get::get,
        // /other/*
    ),
    components(schemas(crate::error::api_error::ApiError)),
    info(
        title = "ExternalDept API",
        version = "1.0.0"
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
