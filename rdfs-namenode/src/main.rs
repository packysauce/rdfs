#![allow(unused_imports)]
use std::{
    convert::{TryFrom, TryInto},
    env,
    pin::Pin,
};

use anyhow::{bail, Context, Error, Result};
use async_std::{
    future,
    io::{prelude::WriteExt, BufRead, Read, ReadExt},
    net::TcpListener,
    task::spawn,
};
use async_std::{
    io::{prelude::BufReadExt, BufReader, BufWriter},
    net::TcpStream,
    stream::StreamExt,
};
use fmt::format;
use rdfs_proto::{
    services::NamenodeProtocolService,
    NamenodeProtocol::{GetBlocksRequestProto, GetBlocksResponseProto},
};
use tracing::{debug, debug_span, error, info, trace, trace_span, Value};
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

const CURRENT_VERSION: u8 = 9;

#[repr(i8)]
enum AuthMode {
    None = 0,
    SASL = -33,
}

impl TryFrom<u8> for AuthMode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value as i8 {
            0 => AuthMode::None,
            -33 => AuthMode::SASL,
            _ => bail!("Invalid AuthMode {}", value),
        })
    }
}

#[tracing::instrument(skip(stream))]
async fn handshake(stream: &mut TcpStream) -> Result<AuthMode> {
    let mut hdr_buf = [0u8; 4];
    let mut writer = stream.clone();
    let reader = stream;
    reader.read(&mut hdr_buf).in_current_span().await?;
    match &hdr_buf {
        b"hrpc" => {}
        b"GET " => {
            trace!("snarky http response");
            writer
                    .write("HTTP/1.1 301 Moved Permanently\r\nContent-Length: 0\r\nLocation: https://google.com/q=wrong%20port\r\n\r\n".as_bytes())
                    .in_current_span()
                    .await?;
            bail!("Somebody tried to cURL me");
        }
        _ => {
            error!(?hdr_buf, whoops=?String::from_utf8_lossy(&hdr_buf), "bad header");
            bail!("Expected b'hrpc', found {:?}", hdr_buf);
        }
    }
    let mut header_bytes = [0u8; 3];
    reader
        .read(&mut header_bytes)
        .in_current_span()
        .await
        .context("Expected version, svc_class, auth_mode")?;
    let [version, svc_class, auth_mode] = header_bytes;
    if version < CURRENT_VERSION {
        error!(client_version=%version, "client too old");
        bail!(
            "Client version {} is less than supported {}",
            version,
            CURRENT_VERSION
        );
    }
    debug!(version, svc_class, auth_mode, "successful");
    Ok(auth_mode.try_into()?)
}

#[tracing::instrument(skip(stream), fields(client = %stream.peer_addr()?))]
async fn handle_client(mut stream: TcpStream) -> Result<()> {
    info!("connected");
    let _auth = handshake(&mut stream).in_current_span().await?;
    debug!("goodbye");
    Ok(())
}

#[async_std::main]
#[paw::main]
#[tracing::instrument]
async fn main() -> Result<()> {
    init_logging();
    let hostport = "[::]:8020";
    let sock = TcpListener::bind(hostport)
        .await
        .with_context(|| format!("Failed to listen on {}", hostport))?;
    let local = sock.local_addr().context("Failed to get local address")?;
    let mut incoming = sock.incoming();
    let listen_span = info_span!("listening", %local);
    let _listen_guard = listen_span.enter();
    while let Some(stream) = incoming.next().in_current_span().await {
        let stream = stream?;
        if let Err(e) = spawn(handle_client(stream).in_current_span()).await {
            error!(error=%e, "Connection error");
        }
        //let _handle = spawn(handle_client(stream).in_current_span());
    }
    info!("Shutting down!");
    Ok(())
}
