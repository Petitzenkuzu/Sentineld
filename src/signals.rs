#[cfg(target_os = "linux")]
pub mod linux {
    use tokio_util::sync::CancellationToken;
    use tokio::signal::unix::{signal, SignalKind};
    use tokio::select;

    pub fn start_signals_handler(shutdown_signal: CancellationToken) {
        tokio::spawn(async move {
            tracing::info!("Starting SIGTERM | SIGINT signal handler");
            let mut sigterm = signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal listener");
            let mut sigint = signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal listener");
        
            tokio::select! {
                _ = sigterm.recv() => {
                    tracing::info!("SIGTERM received, shutdown...");
                }
                _ = sigint.recv() => {
                    tracing::info!("SIGINT received, shutdown...");
                }
            }
        
            shutdown_signal.cancel();
        });
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use tokio_util::sync::CancellationToken;
    use tokio::signal::windows::ctrl_c;
    pub fn start_signals_handler(shutdown_signal: CancellationToken) {
        tokio::spawn(async move {
            tracing::info!("Starting CTRL+C signal handler");
            let mut ctrl_c = ctrl_c().expect("Failed to create CTRL+C signal listener");
            tokio::select! {
                _ = ctrl_c.recv() => {
                    tracing::info!("CTRL+C received, shutdown...");
                    shutdown_signal.cancel();
                }
            }
        });
    }
}