use tokio::task::JoinHandle;
use prometheus::{Gauge};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
pub struct DiskExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    disk_total_gauge: Gauge,
    disk_free_gauge: Gauge,
    disk_used_gauge: Gauge,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for DiskExporter {
    fn start(self) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut delay;
        let mut disk_enabled;
        loop {
            {
                let config = self.config.load();
                delay = interval(Duration::from_secs(config.exporter.disk_interval));
                disk_enabled = config.metrics.disk;
            }
            self.export(disk_enabled);
            delay.tick().await;
        }
       })
    }
}

impl DiskExporter {
    pub fn new(config: Arc<ArcSwap<Config>>, disk_total_gauge: Gauge, disk_free_gauge: Gauge, disk_used_gauge: Gauge, snapshot: Arc<ArcSwap<Snapshot>>) -> Self {
        Self {
            snapshot,
            disk_total_gauge,
            disk_free_gauge,
            disk_used_gauge,
            config,
        }
    }

    fn export(&self, disk_enabled: bool) {
        if disk_enabled {
            let snapshot = self.snapshot.load();
            self.disk_total_gauge.set(snapshot.disk_total);
            self.disk_free_gauge.set(snapshot.disk_free);
            self.disk_used_gauge.set(snapshot.disk_used);
        }
        else {
            self.disk_total_gauge.set(f64::NAN);
            self.disk_free_gauge.set(f64::NAN);
            self.disk_used_gauge.set(f64::NAN);
        }
    }
}