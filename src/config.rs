use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub collection_interval: u64,
    pub metrics: MetricsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            collection_interval: 5,
            metrics: MetricsConfig::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetricsConfig {
    pub cpu : bool,
    pub memory : bool,
    pub disk : bool
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            cpu: true,
            memory: true,
            disk: true
        }
    }
}