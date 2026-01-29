use tokio::task::JoinHandle;
use prometheus::{Gauge, Counter};
use tokio::time::{Duration, interval};
use tokio::task;
use sysinfo::{System, Disks};
use std::time::Instant;

pub fn start_collector(
    cpu_gauge: Gauge,
    memory_used_gauge: Gauge,
    memory_total_gauge: Gauge,
    memory_free_gauge: Gauge,
    disk_total_gauge: Gauge,
    disk_free_gauge: Gauge,
    disk_used_gauge: Gauge,
    agent_uptime_counter: Counter,
) -> JoinHandle<()> {
    
    let mut system = System::new_all();
    system.refresh_all();
    let mut disks = Disks::new_with_refreshed_list();
    disks.refresh(true);
    let mut time = Instant::now();

    task::spawn(async move {
        let mut interval = interval(Duration::from_secs(4));
        loop {
            interval.tick().await;
            system.refresh_cpu_usage();
            system.refresh_memory();
            disks.refresh(true);

            cpu_gauge.set(system.global_cpu_usage() as f64);

            memory_used_gauge.set(system.available_memory() as f64);
            memory_total_gauge.set(system.total_memory() as f64);
            memory_free_gauge.set(system.free_memory() as f64);

            if !disks.is_empty() {
                let mut total_space = 0_u64;
                let mut free_space = 0_u64;
                for disk in disks.iter() {
                    total_space += disk.total_space();
                    free_space += disk.available_space();
                }
                let used_space = total_space - free_space;

                disk_total_gauge.set(total_space as f64);
                disk_free_gauge.set(free_space as f64);
                disk_used_gauge.set(used_space as f64);
            }

            let seconds = time.elapsed().as_secs();
            time = Instant::now();
            agent_uptime_counter.inc_by(seconds as f64);
        }   
    })
}