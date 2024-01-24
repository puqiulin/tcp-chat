use anyhow::Result;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::codec::{Framed, LinesCodec};

// #[derive(Debug)]
pub struct Client {
    pub lines: Framed<TcpStream, LinesCodec>,
    pub rx: UnboundedReceiver<String>,
}

impl Client {
    pub fn new(lines: Framed<TcpStream, LinesCodec>, rx: UnboundedReceiver<String>) -> Self {
        Client { lines, rx }
    }
}
