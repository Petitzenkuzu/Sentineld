use tokio::task::JoinHandle;
use prometheus::{Gauge};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
pub struct CpuExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    cpu_gauge: Gauge,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for CpuExporter {
    fn start(self, shutdown_signal: CancellationToken) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut cpu_enabled;
        let mut interval_secs = {
            let config = self.config.load();
            config.exporter.cpu_interval
        };
        let mut delay = interval(Duration::from_secs(interval_secs));
        loop {
            let config = self.config.load();
            cpu_enabled = config.metrics.cpu;
            if config.exporter.cpu_interval != interval_secs {
                interval_secs = config.exporter.cpu_interval;
                delay = interval(Duration::from_secs(interval_secs));
            }
            tokio::select! {
                _ = delay.tick() => {
                    self.export(cpu_enabled);
                }
                _ = shutdown_signal.cancelled() => {
                    tracing::info!("Shutting down CPU exporter");
                    break;
                }
            }
        }
       })
    }
}

impl CpuExporter {
    pub fn new(config: Arc<ArcSwap<Config>>, cpu_gauge: Gauge, snapshot: Arc<ArcSwap<Snapshot>>) -> Self {
        Self {
            snapshot,
            cpu_gauge,
            config,
        }
    }

    fn export(&self, cpu_enabled: bool) {
        tracing::info!("Exporting CPU metrics");
        if cpu_enabled {
            let snapshot = self.snapshot.load();
            self.cpu_gauge.set(snapshot.cpu_usage);
        }
        else {
            self.cpu_gauge.set(f64::NAN);
        }
    }

}