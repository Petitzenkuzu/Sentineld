use arc_swap::ArcSwap;
use std::sync::Arc;
pub struct State {
    pub snapshot: ArcSwap<SystemMetrics>,
}

impl State {
    pub fn new(metrics: SystemMetrics) -> Self {
        Self {
            snapshot: ArcSwap::new(Arc::new(metrics)),
        }
    }
}

pub struct SystemMetrics {
    available_memory: u64,
    total_memory: u64,
    cpu_usage: f64,
}

impl SystemMetrics {
    pub fn new(available_memory: u64, total_memory: u64, cpu_usage: f64) -> Self {
        Self {
            available_memory,
            total_memory,
            cpu_usage,
        }
    }

    pub fn cpu(&self) -> f64 {
        return self.cpu_usage;
    }

    pub fn memory(&self) -> (u64, u64) {
        return (self.available_memory, self.total_memory);
    }
}