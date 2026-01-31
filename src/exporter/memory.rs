use tokio::task::JoinHandle;
use prometheus::{Gauge};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
pub struct MemoryExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    memory_used_gauge: Gauge,
    memory_total_gauge: Gauge,
    memory_free_gauge: Gauge,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for MemoryExporter {
    fn start(self, shutdown_signal: CancellationToken) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut memory_enabled;
        let mut interval_secs = {
            let config = self.config.load();
            config.exporter.memory_interval
        };
        let mut delay = interval(Duration::from_secs(interval_secs));
        loop {
            let config = self.config.load();
            memory_enabled = config.metrics.memory;
            if config.exporter.memory_interval != interval_secs {
                interval_secs = config.exporter.memory_interval;
                delay = interval(Duration::from_secs(interval_secs));
            }

            tokio::select! {
                _ = delay.tick() => {
                    self.export(memory_enabled);
                }
                _ = shutdown_signal.cancelled() => {
                    tracing::info!("Shutting down memory exporter");
                    break;
                }
            }
        }
       })
    }
}

impl MemoryExporter {
    pub fn new(config: Arc<ArcSwap<Config>>, memory_used_gauge: Gauge, memory_total_gauge: Gauge, memory_free_gauge: Gauge, snapshot: Arc<ArcSwap<Snapshot>>) -> Self {
        Self {
            snapshot,
            memory_used_gauge,
            memory_total_gauge,
            memory_free_gauge,
            config,
        }
    }

    fn export(&self, memory_enabled: bool) {
        tracing::info!("Exporting memory metrics");
        if memory_enabled {
            let snapshot = self.snapshot.load();
            self.memory_used_gauge.set(snapshot.memory_used);
            self.memory_total_gauge.set(snapshot.memory_total);
            self.memory_free_gauge.set(snapshot.memory_free);
        }
        else {
            self.memory_used_gauge.set(f64::NAN);
            self.memory_total_gauge.set(f64::NAN);
            self.memory_free_gauge.set(f64::NAN);
        }
    }
}