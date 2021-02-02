#![allow(unused_imports)]
use std::env;

use anyhow::{Context, Result};
use async_std::{future, io::{BufRead, Read, prelude::WriteExt}, net::TcpListener, task::spawn};
use async_std::{
    io::{prelude::BufReadExt, BufReader, BufWriter},
    net::TcpStream,
    stream::StreamExt,
};
use fmt::format;
use rdfs_proto::{NamenodeProtocol::{GetBlocksResponseProto, GetBlocksRequestProto}, services::NamenodeProtocolService};
use tracing::{Value, debug, error, info, trace_span};
use tracing::{info_span, Instrument};
use tracing_futures::WithSubscriber;
use tracing_subscriber::{
    fmt::{self, format::Pretty},
    prelude::*,
    EnvFilter, FmtSubscriber,
};

fn init_logging() {
    let filter = env::var("RUST_LOG").unwrap_or_else(|_| "rdfs_namenode=TRACE".to_string());
    tracing_subscriber::fmt()
        .event_format(format().pretty())
        .with_env_filter(filter)
        .init();
}

#[tracing::instrument(skip(stream), fields(client = %stream.peer_addr()?))]
async fn handle_client(stream: TcpStream) -> Result<()> {
    info!("connected");
    let mut lines = BufReader::new(stream.clone()).lines();

    while let Some(result) = lines.next().instrument(info_span!("reading")).await {
        let line = result?;
        debug!("got new line");
        let mut writer = BufWriter::new(stream.clone());
        async {
            debug!(len = line.len(), "sending bytes");
            writer
                .write(format!("you said {}\n", line).as_bytes())
                .await?;
            writer.flush().instrument(info_span!("flushing")).await
        }
        .instrument(info_span!("writing"))
        .await?;
    }
    info!("goodbye");
    Ok(())
}

#[async_std::main]
#[paw::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    init_logging();
    let hostport = "[::]:1040";
    let sock = TcpListener::bind(hostport)
        .await
        .with_context(|| format!("Failed to listen on {}", hostport))?;
    let local = sock.local_addr().context("Failed to get local address")?;
    let mut incoming = sock.incoming();
    info!(%local, "listener active");
    while let Some(stream) = incoming.next().in_current_span().await {
        let stream = stream?;
        let _handle = spawn(handle_client(stream).in_current_span());
    }
    info!("Hello, world!");
    Ok(())
}
