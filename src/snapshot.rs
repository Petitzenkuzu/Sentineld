#[derive(Default)]
pub struct Snapshot {
    pub cpu_usage: f64,
    pub memory_used: f64,
    pub memory_total: f64,
    pub memory_free: f64,
    pub disk_total: f64,
    pub disk_free: f64,
    pub disk_used: f64,
    pub agent_uptime: f64,
    pub agent_errors: u64,
}

impl Snapshot {
    pub fn new(cpu_usage: f64, memory_used: f64, memory_total: f64, memory_free: f64, disk_total: f64, disk_free: f64, disk_used: f64, agent_uptime: f64, agent_errors: u64) -> Self {
        Self {
            cpu_usage,
            memory_used,
            memory_total,
            memory_free,
            disk_total,
            disk_free,
            disk_used,
            agent_uptime,
            agent_errors,
        }
    }
}