#![allow(unused_imports)]
use std::{convert::{TryFrom, TryInto}, env, fmt::Debug, net::SocketAddr, pin::Pin};

use anyhow::{anyhow, bail, Context, Error, Result};
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
use async_trait::async_trait;
use fmt::format;
use protobuf::{CodedInputStream, Message};
use rdfs_proto::{IpcConnectionContext::{IpcConnectionContextProto, UserInformationProto}, NamenodeProtocol::{GetBlocksRequestProto, GetBlocksResponseProto}, ProtocolInfo::GetProtocolVersionsRequestProto, RpcHeader::RpcRequestHeaderProto, services::NamenodeProtocolService};
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
#[derive(Debug)]
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

#[tracing::instrument(skip(stream), level="trace")]
async fn read_buf_len<T: Unpin + ReadExt>(stream: &mut T) -> Result<usize> {
    let mut len_buf = [0u8; 4];
    stream.read(&mut len_buf).in_current_span().await?;
    let maybe_len = i32::from_be_bytes(len_buf);
    match maybe_len.try_into() {
        Ok(s) => Ok(s),
        Err(error) => {
            error!(?error, ?len_buf, whoops=%String::from_utf8_lossy(&len_buf), "bad rpc length");
            bail!("Invalid u32 {:?}", len_buf)
        }
    }
}

#[tracing::instrument(skip(stream))]
async fn read_headers(mut stream: &mut TcpStream) -> Result<(RpcRequestHeaderProto, IpcConnectionContextProto)> {
    let len = read_buf_len(&mut stream).in_current_span().await?;
    let mut buf = vec![0u8; len];
    stream.read(&mut buf).await?;
    trace!(expected=len, read=%buf.len(), "bytes read");
    let mut decoder = protobuf::CodedInputStream::from_bytes(&buf);
    let header: RpcRequestHeaderProto = decoder.read_message()
        .map_err(|e| {
            error!(kind="RpcRequestHeaderProto", "failed to decode");
            hexdump::hexdump(&buf);
            e
        })
        .context("Failed to deserialize header")?;
    let context: IpcConnectionContextProto = decoder.read_message()
        .map_err(|e| {
            error!(kind="IpcConnectionContextProto", "failed to decode");
            hexdump::hexdump(&buf);
            e
        })
        .context("Failed to deserialize context")?;
    Ok((header, context))
}


struct HrpcServer {
    user: Option<UserInformationProto>,
    client_id: Option<Vec<u8>>,
    next_layer: Option<String>,
    client: SocketAddr,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    auth_mode: AuthMode,
}

impl Debug for HrpcServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hrpc")
            .field("client", &self.client)
            .field("auth_mode", &self.auth_mode)
            .finish()
    }
}

impl HrpcServer {
    fn new(stream: TcpStream) -> Self {
        HrpcServer {
            user: None,
            client_id: None,
            next_layer: None,
            client: stream.peer_addr().unwrap(),
            reader: BufReader::new(stream.clone()),
            writer: BufWriter::new(stream),
            auth_mode: AuthMode::None,
        }
    }

    #[tracing::instrument]
    async fn run(mut self) -> Result<()> {
        info!("new connection");
        self.next_layer = Some(self.handshake().await?);

        Ok(())
    }

    #[tracing::instrument]
    async fn read_message<T: Message>(&mut self) -> Result<T> {
        let len = read_buf_len(&mut self.reader).in_current_span().await?;
        let mut buf = vec![0u8; len];
        self.reader.read(&mut buf).await?;
        let mut decoder = protobuf::CodedInputStream::from_bytes(&buf);
        decoder.read_message()
            .map_err(|e| {
                error!(kind=std::any::type_name::<T>(), "failed to decode");
                hexdump::hexdump(&buf);
                e
            })
            .context("Failed to read protobuf")
    }
   
    #[tracing::instrument]
    async fn handshake(&mut self) -> Result<String> {
        let mut hdr_buf = [0u8; 7];
        self.reader.read(&mut hdr_buf).in_current_span().await?;
        match &hdr_buf[..4] {
            b"hrpc" => {}
            b"GET " => {
                //trace!("snarky http response");
                self.writer
                        .write("HTTP/1.1 301 Moved Permanently\r\n\
                                    Content-Length: 0\r\n\
                                    Location: https://google.com/?q=wrong%20port\r\n\
                                    \r\n".as_bytes())
                        .in_current_span()
                        .await?;
                self.writer.flush().await?;
                bail!("Somebody tried to cURL me");
            }
            _ => {
                debug!(?hdr_buf, whoops=%String::from_utf8_lossy(&hdr_buf), "bad header");
                bail!("Expected b'hrpc', found {:?}", hdr_buf);
            }
        }

        if let [version, svc_class, auth_mode] = hdr_buf[4..] {
            if version < CURRENT_VERSION {
                error!(client_version=%version, "client too old");
                bail!(
                    "Client version {} is less than supported {}",
                    version,
                    CURRENT_VERSION
                );
            }
            debug!(version, svc_class, ?auth_mode, "successful");
            self.auth_mode = auth_mode.try_into()?;
        } else {
            debug!(?hdr_buf, "invalid header buffer");
            bail!("invalid header buffer");
        }

        // need to manually read 2 messages out of a buffer here.... IPC context
        let buf_len = read_buf_len(&mut self.reader).await?;
        let mut buf = vec![0u8; buf_len];
        self.reader.read(&mut buf).await?;
        let mut decoder = CodedInputStream::from_bytes(&buf);
        let rpc_header = decoder.read_message::<RpcRequestHeaderProto>()
            .map_err(|e| {
                error!("failed to decode");
                hexdump::hexdump(&buf);
                e
            })
            .context("Failed to read protobuf")?;
        debug!(?rpc_header, "header received");

        match rpc_header.get_callId() {
            -3 => {
                // whatever, we'll set up all kinds of goodies!
                self.client_id = Some(rpc_header.get_clientId().to_vec());
                let ipc_context = decoder.read_message::<IpcConnectionContextProto>()
                    .context("Failed to read IPC context")?;
                    debug!(?ipc_context, "context received");
                self.user = Some(ipc_context.get_userInfo().clone());
                return Ok(ipc_context.get_protocol().to_string())
            } // need context
            call_id if call_id < 0 => {
                debug!(call_id, "unfamiliar call id");
            }
            call_id => {
                error!(call_id, "unsupported call id");
            }
        }
        bail!("fuck off")
    }
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
    info!("waiting for connections");
    while let Some(stream) = incoming.next().in_current_span().await {
        let stream = stream?;
        let client = stream.peer_addr()?;
        let join_handle = spawn({
            let rpc = HrpcServer::new(stream);
            rpc.run()
        });
        if let Err(e) = join_handle.await {
            error!(%client, error=%e, "Connection error");
        }
    }
    info!("Shutting down!");
    Ok(())
}
