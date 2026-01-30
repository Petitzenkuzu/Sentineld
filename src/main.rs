mod handlers;
use handlers::cpu_handler;
use handlers::health_handler;

use actix_web::{web, App, HttpServer};

mod state;
use state::State;

use prometheus::{Gauge, Registry, Opts, Counter};

use serde_yml;

mod config;
use config::Config;

use std::fs::File;

mod collector;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut config = Config::default();
    if let Ok(config_file) = File::open("config.yml") {
        if let Ok(config_data) = serde_yml::from_reader(config_file) {
            config = config_data;
        }
    }

    let cpu_gauge = Gauge::new("sentinel_cpu_usage_percent", "CPU usage in percentage").unwrap();
    
    let memory_used_gauge = Gauge::new("sentinel_memory_used_bytes", "Memory used in bytes").unwrap();
    let memory_total_gauge = Gauge::new("sentinel_memory_total_bytes", "Memory total in bytes").unwrap();
    let memory_free_gauge = Gauge::new("sentinel_memory_free_bytes", "Memory free in bytes").unwrap();
    
    let disk_total_gauge = Gauge::new("sentinel_disk_total_bytes", "Disk total in bytes").unwrap();
    let disk_free_gauge = Gauge::new("sentinel_disk_free_bytes", "Disk free in bytes").unwrap();
    let disk_used_gauge = Gauge::new("sentinel_disk_used_bytes", "Disk used in bytes").unwrap();

    let agent_uptime_counter_opts = Opts::new("sentinel_agent_uptime_seconds", "Agent uptime in seconds");
    let agent_uptime_counter = Counter::with_opts(agent_uptime_counter_opts).unwrap();

    let agent_errors_counter_opts = Opts::new("sentinel_agent_errors_count", "Number of errors during collection");
    let agent_errors_counter = Counter::with_opts(agent_errors_counter_opts).unwrap();

    let collector_task = collector::start_collector(
        config.collection_interval,
        cpu_gauge.clone(),
        memory_used_gauge.clone(),
        memory_total_gauge.clone(),
        memory_free_gauge.clone(),
        disk_total_gauge.clone(),
        disk_free_gauge.clone(),
        disk_used_gauge.clone(),
        agent_uptime_counter.clone(),
    );

    let registry = Registry::default();
    registry.register(Box::new(cpu_gauge)).unwrap();
    registry.register(Box::new(memory_used_gauge)).unwrap();
    registry.register(Box::new(memory_total_gauge)).unwrap();
    registry.register(Box::new(memory_free_gauge)).unwrap();
    registry.register(Box::new(disk_total_gauge)).unwrap();
    registry.register(Box::new(disk_free_gauge)).unwrap();
    registry.register(Box::new(disk_used_gauge)).unwrap();
    registry.register(Box::new(agent_uptime_counter)).unwrap();
    registry.register(Box::new(agent_errors_counter)).unwrap();

    let state = web::Data::new(State::new(registry));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health_handler)
            .service(cpu_handler)
    })
    .bind((config.host.as_str(), config.port))?
    .run()
    .await?;
    Ok(())
}
