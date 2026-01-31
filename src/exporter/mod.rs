use arc_swap::ArcSwap;
use crate::config::Config;
use tokio::task::JoinHandle;

pub trait MetricsExporter {
    fn start(self) -> JoinHandle<()>;
}

mod cpu;
pub use cpu::CpuExporter;

mod memory;
pub use memory::MemoryExporter;

mod disk;
pub use disk::DiskExporter;

mod agent;
pub use agent::AgentExporter;