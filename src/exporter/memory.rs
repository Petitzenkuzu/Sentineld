use tokio::task::JoinHandle;
use prometheus::{Gauge};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
pub struct MemoryExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    memory_used_gauge: Gauge,
    memory_total_gauge: Gauge,
    memory_free_gauge: Gauge,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for MemoryExporter {
    fn start(self) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut delay;
        let mut memory_enabled;
        loop {
            {
                let config = self.config.load();
                delay = interval(Duration::from_secs(config.exporter.memory_interval));
                memory_enabled = config.metrics.memory;
            }
            self.export(memory_enabled);
            delay.tick().await;
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