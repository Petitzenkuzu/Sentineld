use tokio::task::JoinHandle;
use prometheus::{Gauge};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
pub struct CpuExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    cpu_gauge: Gauge,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for CpuExporter {
    fn start(self) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut delay;
        let mut cpu_enabled;
        loop {
            {
                let config = self.config.load();
                delay = interval(Duration::from_secs(config.exporter.cpu_interval));
                cpu_enabled = config.metrics.cpu;
            }
            self.export(cpu_enabled);
            delay.tick().await;
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
        if cpu_enabled {
            let snapshot = self.snapshot.load();
            self.cpu_gauge.set(snapshot.cpu_usage);
        }
        else {
            self.cpu_gauge.set(f64::NAN);
        }
    }

}