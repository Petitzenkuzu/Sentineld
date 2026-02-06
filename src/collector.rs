use tokio::task::JoinHandle;
use std::sync::Arc;
use tokio::time::{Duration, interval};
use tokio::task;
use std::time::Instant;
use sysinfo::{System, Disks};
use crate::config::Config;
use arc_swap::ArcSwap;
use crate::snapshot::Snapshot;
use tokio_util::sync::CancellationToken;


pub struct SystemCollector {
    config: Arc<ArcSwap<Config>>,
    snapshot: Arc<ArcSwap<Snapshot>>,
    system: System,
    disks: Disks,
    start_time: Instant,
}

impl SystemCollector {
    pub fn new(config: Arc<ArcSwap<Config>>, snapshot: Arc<ArcSwap<Snapshot>>) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let mut disks = Disks::new_with_refreshed_list();
        disks.refresh(true);
        Self {
            config,
            snapshot,
            system,
            disks,
            start_time: Instant::now(),
        }
    }

    pub fn start(mut self, shutdown_signal: CancellationToken) -> JoinHandle<()> {
        task::spawn(async move {
            self.start_time = Instant::now();
            let mut interval_secs = {
                let config = self.config.load();
                config.collection.interval
            };
            let mut delay = interval(Duration::from_secs(interval_secs));
            self.system.refresh_all();
            loop {
                let config = self.config.load();
                if config.collection.interval != interval_secs {
                    interval_secs = config.collection.interval;
                    delay = interval(Duration::from_secs(interval_secs));
                }
                tokio::select! {
                    _ = delay.tick() => {
                        self.collect();
                    }
                    _ = shutdown_signal.cancelled() => {
                        tracing::info!("Shutting down system collector");
                        break;
                    }
                }
            }
        })
    }

    fn collect(&mut self) {
        tracing::info!("Collecting system metrics");
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.disks.refresh(true);
        let mut total_space = 0_u64;
        let mut free_space = 0_u64;
        for disk in self.disks.iter() {
            total_space += disk.total_space();
            free_space += disk.available_space();
        }
        let used_space = total_space.saturating_sub(free_space);
        let cpu_usage = self.system.global_cpu_usage() as f64;
        let memory_used = self.system.available_memory() as f64;
        let memory_total = self.system.total_memory() as f64;
        let memory_free = self.system.free_memory() as f64;
        let agent_uptime = self.start_time.elapsed().as_secs() as f64;
                
        self.snapshot.store(Arc::new(Snapshot::new(
            cpu_usage,
            memory_used,
            memory_total,
            memory_free,
            total_space as f64,
            free_space as f64,
            used_space as f64,
            agent_uptime,
            self.snapshot.load().agent_errors,
        )));
        
    }
}
