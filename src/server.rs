use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;

// #[derive(Debug)]
pub struct Server {
    pub clients: HashMap<SocketAddr, mpsc::UnboundedSender<String>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }

    pub fn broadcast(&mut self, sender: SocketAddr, message: &str) -> Result<()> {
        for c in self.clients.iter_mut() {
            if *c.0 != sender {
                c.1.send(message.into())?
            }
        }
        Ok(())
    }
}
