use rocket::get;
use rocket::serde::json::Json;

#[utoipa::path(
    get,
    tag = "Test",
    path = "/api/test/ping",
    responses(
        (status = 200, description = "Ping the server to check if it's running", body = String)
    )
)]
#[get("/test/ping")]
pub async fn ping() -> Json<&'static str> {
    Json("pong")
}
