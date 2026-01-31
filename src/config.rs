use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub exporter: ExporterConfig,
    pub metrics: MetricsConfig,
    pub collection: CollectionConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            exporter: ExporterConfig::default(),
            metrics: MetricsConfig::default(),
            collection: CollectionConfig::default(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExporterConfig {
    pub cpu_interval: u64,
    pub memory_interval: u64,
    pub disk_interval: u64,
    pub agent_interval: u64,
}

impl Default for ExporterConfig {
    fn default() -> Self {
        Self {
            cpu_interval: 5,
            memory_interval: 5,
            disk_interval: 5,
            agent_interval: 5,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionConfig {
    pub interval: u64,
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            interval: 5,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub cpu : bool,
    pub memory : bool,
    pub disk : bool,
    pub agent : bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            cpu: true,
            memory: true,
            disk: true,
            agent: true,
        }
    }
}