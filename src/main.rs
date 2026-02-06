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

mod exporter;
use exporter::CpuExporter;
use exporter::MemoryExporter;
use exporter::DiskExporter;
use exporter::MetricsExporter;
use exporter::AgentExporter;

mod snapshot;
use snapshot::Snapshot;

mod collector;
use collector::SystemCollector;

use arc_swap::ArcSwap;

use std::sync::Arc;

mod signals;
#[cfg(target_os = "linux")]
use signals::linux::start_signals_handler;
#[cfg(target_os = "windows")]
use signals::windows::start_signals_handler;

use tracing_subscriber::FmtSubscriber;
use tracing::Level;
use tokio_util::sync::CancellationToken;
use std::path::PathBuf;
mod watcher;
use watcher::start_config_watcher;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::info!("Starting Sentineld");

    let config_path = if let Ok(path) = std::env::var("SENTINELD_CONFIG") {
        PathBuf::from(path)
    } else {
        PathBuf::from("./config.yml")
    };

    let config = Arc::new(ArcSwap::new(Arc::new(Config::default())));
    if let Ok(config_file) = File::open(&config_path) {
        if let Ok(config_data) = serde_yml::from_reader(&config_file) {
            config.store(Arc::new(config_data));
        } else {
            tracing::error!("Failed to parse config file");
        }
    } else {
        tracing::error!("Failed to open config file");
    }

    let shutdown_signal = CancellationToken::new();
    start_signals_handler(shutdown_signal.clone());

    let cpu_gauge = Gauge::new("sentinel_cpu_usage_percent", "CPU usage in percentage").unwrap();
    
    let memory_used_gauge = Gauge::new("sentinel_memory_used_bytes", "Memory used in bytes").unwrap();
    let memory_total_gauge = Gauge::new("sentinel_memory_total_bytes", "Memory total in bytes").unwrap();
    let memory_free_gauge = Gauge::new("sentinel_memory_free_bytes", "Memory free in bytes").unwrap();
    
    let disk_total_gauge = Gauge::new("sentinel_disk_total_bytes", "Disk total in bytes").unwrap();
    let disk_free_gauge = Gauge::new("sentinel_disk_free_bytes", "Disk free in bytes").unwrap();
    let disk_used_gauge = Gauge::new("sentinel_disk_used_bytes", "Disk used in bytes").unwrap();

    let agent_uptime_gauge = Gauge::new("sentinel_agent_uptime_seconds", "Agent uptime in seconds").unwrap();

    let agent_errors_counter_opts = Opts::new("sentinel_agent_errors_count", "Number of errors during collection");
    let agent_errors_counter = Counter::with_opts(agent_errors_counter_opts).unwrap();

    start_config_watcher(config.clone(), config_path, shutdown_signal.clone(), agent_errors_counter.clone());

    let snapshot = Arc::new(ArcSwap::new(Arc::new(Snapshot::default())));
    let system_collector = SystemCollector::new(config.clone(), snapshot.clone());
    let cpu_exporter = CpuExporter::new(config.clone(), cpu_gauge.clone(), snapshot.clone());
    let memory_exporter = MemoryExporter::new(config.clone(), memory_used_gauge.clone(), memory_total_gauge.clone(), memory_free_gauge.clone(), snapshot.clone());
    let disk_exporter = DiskExporter::new(config.clone(), disk_total_gauge.clone(), disk_free_gauge.clone(), disk_used_gauge.clone(), snapshot.clone());
    let agent_exporter = AgentExporter::new(config.clone(), agent_uptime_gauge.clone(), agent_errors_counter.clone(), snapshot.clone());

    let _tasks = vec![
        system_collector.start(shutdown_signal.clone()),
        cpu_exporter.start(shutdown_signal.clone()),
        memory_exporter.start(shutdown_signal.clone()),
        disk_exporter.start(shutdown_signal.clone()),
        agent_exporter.start(shutdown_signal.clone()),
    ];

    let registry = Registry::default();
    registry.register(Box::new(cpu_gauge)).expect("Failed to register CPU gauge");
    registry.register(Box::new(memory_used_gauge)).expect("Failed to register Memory used gauge");
    registry.register(Box::new(memory_total_gauge)).expect("Failed to register Memory total gauge");
    registry.register(Box::new(memory_free_gauge)).expect("Failed to register Memory free gauge");
    registry.register(Box::new(disk_total_gauge)).expect("Failed to register Disk total gauge");
    registry.register(Box::new(disk_free_gauge)).expect("Failed to register Disk free gauge");
    registry.register(Box::new(disk_used_gauge)).expect("Failed to register Disk used gauge");
    registry.register(Box::new(agent_uptime_gauge)).expect("Failed to register Agent uptime gauge");
    registry.register(Box::new(agent_errors_counter)).expect("Failed to register Agent errors counter");
    
    let state = web::Data::new(State::new(registry));
    let host = config.load().host.clone();
    let port = config.load().port;
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health_handler)
            .service(cpu_handler)
    })
    .workers(2)
    .bind((host.as_str(), port))?
    .run()
    .await?;
    Ok(())
}
