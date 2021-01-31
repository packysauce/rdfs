
use async_std::{net::TcpStream, stream::StreamExt};
use anyhow::{Result, Context};
use tracing::{info, error};
use tracing::Instrument;
use async_std::net::{TcpListener, ToSocketAddrs};

fn init_logging() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter("rdfs_namenode=TRACE")
        .init();
}

#[async_std::main]
#[paw::main]
async fn main() -> Result<()> {
    init_logging();
    let hostport = "localhost:1040";
    let sock = TcpListener::bind(hostport)
        .await
        .with_context(|| format!("Failed to listen on {}", hostport))?;
    //info!(, "Listening");
    let local = sock.local_addr().context("Failed to get local address")?;
    info!(%local, "Client listener active");
    let mut incoming = sock.incoming();
    while let Some(attempt) = incoming.next().await {
        match attempt {
            Ok(client) => {
                let peer = client.peer_addr()?;
                info!(%peer, "Client connected");
            },
            Err(e) => {
                error!(%e, "Client connection error");
            }
        }
    }
    info!("Hello, world!");
    Ok(())
}
