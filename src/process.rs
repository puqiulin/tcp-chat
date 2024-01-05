use crate::client_handle::ClientHandle;
use crate::server_handle::ServerHandle;
use anyhow::Result;
use futures::SinkExt;
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
// use std::sync::Mutex;
use crate::utils::get_local_time;
use chrono::prelude::*;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{error, info};

pub async fn process(
    server_handle: Arc<Mutex<ServerHandle>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("Enter your name:").await?;
    let name = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            error!("Failed to get name from {}", addr);
            return Ok(());
        }
    };

    let (tx, rx) = mpsc::unbounded_channel();
    let mut client = ClientHandle::new(lines, rx)?;

    {
        let mut s_handle = server_handle.lock();
        s_handle.clients.insert(addr, tx);
        let new_client_msg = format!(
            "new client[{}] connected: [{}], current user number: {}",
            &addr,
            name,
            s_handle.clients.len()
        );
        info!("{}", new_client_msg);
        s_handle.broadcast(addr, &new_client_msg)?;
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

    {
        let mut s_handle = server_handle.lock();
        s_handle.clients.remove(&addr);
        let remove_user_msg = format!(
            "[{}] user [{}] disconnect, current user number: {}",
            get_local_time(),
            name,
            s_handle.clients.len()
        );
        info!("{}", remove_user_msg);
        s_handle.broadcast(addr, &remove_user_msg)?;
    }

    Ok(())
}
