mod handlers;
use handlers::cpu_handler;
use handlers::health_handler;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio::time::{Duration, interval};
use tokio::task::spawn;

use sysinfo::{
    System,
};
mod state;
use state::State;
use state::SystemMetrics;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut system = System::new_all();
    let metrics = SystemMetrics::new(system.available_memory(), system.total_memory(), system.global_cpu_usage() as f64);
    let state = Arc::new(State::new(metrics));
    let state_clone = Arc::clone(&state);

    spawn(async move {
        let mut interval = interval(Duration::from_secs(4));
        loop {
            interval.tick().await;
            system.refresh_all();
            let metrics = SystemMetrics::new(system.available_memory(), system.total_memory(), system.global_cpu_usage() as f64);
            state_clone.snapshot.store(Arc::new(metrics));
        }   
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&state)))
            .service(health_handler)
            .service(
                web::scope("/metrics")
                    .service(cpu_handler)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
