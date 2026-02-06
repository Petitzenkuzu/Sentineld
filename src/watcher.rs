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
use std::fs::File;

pub fn start_config_watcher(config: Arc<ArcSwap<Config>>, config_path: PathBuf, shutdown_signal: CancellationToken, error_counter: Counter) {
    std::thread::spawn(move || {
        tracing::info!("Starting config watcher");
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())
            .expect("Failed to create watcher");

        match watcher.watch(&config_path, RecursiveMode::NonRecursive) {
            Ok(_) => {
                tracing::info!("Config file watched successfully started");
            }
            Err(e) => {
                tracing::error!("Failed to watch config file: {}, creating default config..", e.to_string());

                // we try to create the config file
                let file = match File::create(&config_path) {
                    Ok(file) => file,
                    Err(e) => {
                        // if we fail to create the config file, we can't start the watcher
                        tracing::error!("Failed to create config file: {}", e.to_string());
                        error_counter.inc();
                        return;
                    }
                };
                
                let default_config = Config::default();
                if let Err(e) = serde_yml::to_writer(&file, &default_config) {
                    // if we fail to write the default config, we can't start the watcher
                    tracing::error!("Failed to write default config: {}", e.to_string());
                    error_counter.inc();
                    return;
                }

                tracing::info!("Default config created");
                // storing the default config in the config swap
                config.store(Arc::new(default_config));

                if let Ok(()) = watcher.watch(&config_path, RecursiveMode::NonRecursive) {
                    // watcher successfully started
                    tracing::info!("Config file watcher successfully started");
                } else {
                    tracing::error!("Failed to watch newly created config file: {}", e.to_string());
                    // if we fail to watch the newly created config file, we can't start the watcher
                    error_counter.inc();
                    return;
                }
            }
        }

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