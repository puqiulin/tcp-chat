use crate::client::Client;
use crate::server::Server;
use crate::utils::get_local_time;
use anyhow::Result;
use futures::SinkExt;
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{error, info};

// #[tracing::instrument]
pub async fn process(
    server_handle: Arc<Mutex<Server>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("Enter your name").await?;
    let name = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            error!("Failed to get name from {}", addr);
            return Ok(());
        }
    };

    let (tx, rx) = mpsc::unbounded_channel();
    let mut client = Client::new(lines, rx);

    //add client by server handle
    {
        let mut sh = server_handle.lock();
        sh.clients.insert(addr, tx);
        let new_client_msg = format!(
            "new client[{}] connected: [{}], current user number: {}",
            &addr,
            name,
            sh.clients.len()
        );
        info!("{}", new_client_msg);
        sh.broadcast(addr, &new_client_msg)?;
    }

    loop {
        select! {
            Some(msg)=client.rx.recv()=>{
                client.lines.send(&msg).await?;
            }
            next=client.lines.next()=>match next{
               Some(Ok(msg))=>{
                    let msg=format!("[{}] {}: {}",get_local_time(),name,msg);
                    server_handle.lock().broadcast(addr,&msg)?;
                }
                Some(Err(e))=>{
                    error!("Failed to processing message for {}: {}",name,e);
                }
                None=>break
            }
        }
    }

    //remove client from server handle
    {
        let mut sh = server_handle.lock();
        sh.clients.remove(&addr);
        let remove_user_msg = format!(
            "[{}] user [{}] disconnected, current user number: {}",
            get_local_time(),
            name,
            sh.clients.len()
        );
        info!("{}", remove_user_msg);
        sh.broadcast(addr, &remove_user_msg)?;
    }

    Ok(())
}
