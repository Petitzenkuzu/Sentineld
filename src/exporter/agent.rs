use tokio::task::JoinHandle;
use prometheus::{Gauge, Counter};
use tokio::time::{Duration, interval};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::exporter::MetricsExporter;
use crate::snapshot::Snapshot;
use std::sync::Arc;
pub struct AgentExporter {
    snapshot: Arc<ArcSwap<Snapshot>>,
    agent_uptime_gauge: Gauge,
    agent_errors_counter: Counter,
    config: Arc<ArcSwap<Config>>,
}

impl MetricsExporter for AgentExporter {
    fn start(self) -> JoinHandle<()> {
       tokio::spawn(async move {
        let mut delay;
        let mut agent_enabled;
        loop {
            {
                let config = self.config.load();
                delay = interval(Duration::from_secs(config.exporter.agent_interval));
                agent_enabled = config.metrics.agent;
            }
            self.export(agent_enabled);
            delay.tick().await;
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
        if agent_enabled {
            let snapshot = self.snapshot.load();
            self.agent_uptime_gauge.set(snapshot.agent_uptime);
        }
        else {
            self.agent_uptime_gauge.set(f64::NAN);
        }
    }
}