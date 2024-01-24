mod client;
mod process;
mod server;
mod utils;

use crate::process::process;
use crate::server::Server;
use anyhow::Result;
// use console_subscriber::ConsoleLayer;
use parking_lot::Mutex;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::prelude::*;

// TODO
// add reconnect
// add uuid

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::registry()
    //     .with(ConsoleLayer::builder().spawn())
    //     .init();

    tracing_subscriber::fmt().init();

    let addr = "127.0.0.1:8888";
    let server = TcpListener::bind(addr).await?;
    info!("Chat server running on {}", &addr);

    let server_handle = Arc::new(Mutex::new(Server::new()));

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
