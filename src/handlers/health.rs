use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().body("Healthy")
}