use tokio_util::sync::CancellationToken;
use crate::config::Config;
use arc_swap::ArcSwap;
use std::sync::Arc;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::time::Duration;
use notify::event::EventKind;
use std::sync::mpsc::channel;
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Instant;
use prometheus::Counter;

pub fn start_config_watcher(config: Arc<ArcSwap<Config>>, config_path: PathBuf, shutdown_signal: CancellationToken, error_counter: Counter) {
    std::thread::spawn(move || {
        tracing::info!("Starting config watcher");
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())
            .expect("Failed to create watcher");

        watcher
            .watch(&config_path, RecursiveMode::NonRecursive)
            .expect("Failed to watch config file");

        let mut delay = Instant::now();
        loop {
            if shutdown_signal.is_cancelled() {
                tracing::info!("Shutting down config watcher");
                break;
            }
            match rx.recv_timeout(Duration::from_secs(200)) {
                Ok(event) => {
                    if let Ok(event) = event {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) && delay.elapsed() > Duration::from_secs(5) {
                            delay = Instant::now();
                            match std::fs::File::open(&config_path)
                                .ok()
                                .and_then(|f| serde_yml::from_reader(f).ok())
                            {
                                Some(new_cfg) => {
                                    config.store(Arc::new(new_cfg));
                                    tracing::info!("Config reloaded");
                                }
                                None => {
                                    tracing::warn!("Config reload failed, keeping previous config");
                                    error_counter.inc();
                                }
                            }
                        }
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    continue;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }
    });
}