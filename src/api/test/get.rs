use rocket::get;
use rocket::serde::json::Json;

#[allow(dead_code)]
#[get("/test/ping")]
pub async fn ping() -> Json<&'static str> {
    Json("pong")
}
