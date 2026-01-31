use arc_swap::ArcSwap;
use crate::config::Config;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub trait MetricsExporter {
    fn start(self, shutdown_signal: CancellationToken) -> JoinHandle<()>;
}

mod cpu;
pub use cpu::CpuExporter;

mod memory;
pub use memory::MemoryExporter;

mod disk;
pub use disk::DiskExporter;

mod agent;
pub use agent::AgentExporter;