use colored::Colorize;
use crate::{CLUSTER_MANAGER, SERVER_CONFIG};

pub fn start_peer_discovery(port: u16) {
    log::info!("{}", "Starting peer discovery background task".magenta());
    tokio::task::spawn({
        let cluster_manager = CLUSTER_MANAGER.clone();
        let server_config = SERVER_CONFIG.clone();
        async move {
            loop {
                if let Err(e) = cluster_manager
                    .read()
                    .await
                    .discover_peers(&server_config, port)
                    .await
                {
                    log::error!("{}", format!("Failed to discover peers: {e}").red());
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
            }
        }
    });
}
