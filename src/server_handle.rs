use anyhow::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub struct ServerHandle {
    pub clients: HashMap<SocketAddr, mpsc::UnboundedSender<String>>,
}

impl ServerHandle {
    pub fn new() -> Self {
        ServerHandle {
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
