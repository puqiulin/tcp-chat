use anyhow::Result;
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_util::codec::{Framed, LinesCodec};

pub struct ClientHandle {
    pub lines: Framed<TcpStream, LinesCodec>,
    pub rx: UnboundedReceiver<String>,
}

impl ClientHandle {
    pub fn new(
        lines: Framed<TcpStream, LinesCodec>,
        rx: UnboundedReceiver<String>,
    ) -> Result<Self> {
        Ok(ClientHandle { lines, rx })
    }
}
