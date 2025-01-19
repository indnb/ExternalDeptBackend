use crate::api;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(api::test::get::ping, api::hackathon_2024::university::post::create,
        api::hackathon_2024::university::get::all,
        api::hackathon_2024::university::put::by_id,
        api::hackathon_2024::university::delete::by_id,
        api::hackathon_2024::university::get::by_id,
        api::hackathon_2024::team::post::create,
        api::hackathon_2024::team::get::all,
        api::hackathon_2024::team::get::by_id,
        api::hackathon_2024::team::put::by_id,
        api::hackathon_2024::team::delete::by_id,
        api::admin::post::login,
        api::admin::get::get
    ),
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
