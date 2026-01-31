use actix_web::{get, HttpResponse};
use crate::state::State;
use actix_web::web;
use prometheus::{TextEncoder, Encoder};

#[get("/metrics")]
pub async fn cpu_handler(state: web::Data<State>) -> HttpResponse {

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = state.registry.gather();
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        return HttpResponse::InternalServerError().body(e.to_string());
    }
    if let Ok(s)     = String::from_utf8(buffer) {
    println!("{}", s);
    HttpResponse::Ok().body(s)
    } else {
        HttpResponse::InternalServerError().body("Failed to convert encoded buffer to string")
    }
}