use actix_web::{get, HttpResponse};
use crate::state::{State, SystemMetrics};
use std::sync::Arc;
use actix_web::web;

#[get("/cpu")]
pub async fn cpu_handler(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().body(format!("CPU usage: {}", state.snapshot.load().cpu()))
}