use crate::{api, diesel::prelude::ApiError};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(api::test::get::ping, api::hackathon_2024::university::post::create,
        api::hackathon_2024::university::get::all,
        api::hackathon_2024::university::put::by_id,
        api::hackathon_2024::university::delete::by_id,
        api::hackathon_2024::university::get::by_id,
    ),
    components(schemas(
        crate::diesel::models::hackathon_2024::university::HackathonUniversity2024Insertable,
        crate::middleware::admin_token_match::AdminAuthData,
        ApiError,
    )),
    info(
        title = "ExternalDept Hackathon 2024 API",
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
