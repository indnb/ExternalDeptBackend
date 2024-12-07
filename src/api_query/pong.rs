use rocket::get;
use rocket::serde::json::Json;

#[allow(dead_code)]
#[get("/ping")]
pub async fn get_pong() -> Json<&'static str> {
    Json("pong")
}
