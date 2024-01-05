mod client_handle;
mod process;
mod server_handle;
mod utils;

use crate::process::process;
use crate::server_handle::ServerHandle;
use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

//TODO
// add reconnect
// add uuid

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt().init();

    let addr = "0.0.0.0:9595";
    let server = TcpListener::bind(addr).await?;
    info!("Chat server running on {}", &addr);

    let server_handle = Arc::new(Mutex::new(ServerHandle::new()));

    loop {
        let (stream, addr) = server.accept().await?;
        let server_handle = server_handle.clone();

        tokio::spawn(async move {
            info!("Accept client {}", &addr);
            if let Err(e) = process(server_handle, stream, addr).await {
                error!("Processing client[{}] error: {}", &addr, e);
            }
        });
    }
}
