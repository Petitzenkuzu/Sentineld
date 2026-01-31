use tokio::task::JoinHandle;
use prometheus::{Gauge, Counter};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
pub struct AgentExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    agent_uptime_gauge: Gauge,
    agent_errors_counter: Counter,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for AgentExporter {
    fn start(self, shutdown_signal: CancellationToken) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut agent_enabled;
        let mut interval_secs = {
            let config = self.config.load();
            config.exporter.agent_interval
        };
        let mut delay = interval(Duration::from_secs(interval_secs));
        loop {
            let config = self.config.load();
            agent_enabled = config.metrics.agent;
            if config.exporter.agent_interval != interval_secs {
                interval_secs = config.exporter.agent_interval;
                delay = interval(Duration::from_secs(interval_secs));
            }
            tokio::select! {
                _ = delay.tick() => {
                    self.export(agent_enabled);
                }
                _ = shutdown_signal.cancelled() => {
                    tracing::info!("Shutting down agent exporter");
                    break;
                }
            }
        }
       })
    }
}

impl AgentExporter {
    pub fn new(config: Arc<ArcSwap<Config>>, agent_uptime_gauge: Gauge, agent_errors_counter: Counter, snapshot: Arc<ArcSwap<Snapshot>>) -> Self {
        Self {
            snapshot,
            agent_uptime_gauge,
            agent_errors_counter,
            config,
        }
    }

    fn export(&self, agent_enabled: bool) {
        tracing::info!("Exporting agent metrics");
        if agent_enabled {
            let snapshot = self.snapshot.load();
            self.agent_uptime_gauge.set(snapshot.agent_uptime);
        }
        else {
            self.agent_uptime_gauge.set(f64::NAN);
        }
    }
}