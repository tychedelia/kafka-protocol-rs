use std::net::SocketAddr;
use std::collections::HashMap;
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::io::Error;
use tokio_util::codec::LengthDelimitedCodec;
use bytes::{Bytes, BytesMut};
use futures::future::FutureExt;
use async_trait::async_trait;
use tokio::time::Elapsed;

use franz_protocol::{RequestType, Request, CompatibilityError, DecodeCompatible};

use super::messages::api_versions::{ApiVersions, ApiVersions1Request};
use super::messages::header::RequestHeader0;
use super::service::{Service, ServiceBase, ServiceExt, BoxService, RecvFuture, Pipeline, Shutdown, EagerService};

const MAX_FRAME_LENGTH: usize = std::i32::MAX as usize;

type RequestHeader = RequestHeader0;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    max_message_size: Option<usize>,
    max_concurrency: usize,
    request_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            max_message_size: None,
            max_concurrency: 10,
            request_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct VersionRange {
    min: i16,
    max: i16,
}

pub struct ClientState {
    supported_versions: HashMap<i16, VersionRange>,
    correlation_id: i32,
    inner: BoxService<'static, Bytes, BytesMut, ClientError>,
}

impl ServiceBase for ClientState {}

#[async_trait]
impl<Req> Service<Req> for ClientState
where
    Req: Request + Send + 'static,
    Req::Response: Send + 'static,
{
    type Response = Req::Response;
    type Error = ClientError;

    async fn send(&'async_trait mut self, req: Req) -> Result<RecvFuture<Self::Response, Self::Error>, Self::Error> {
        // Encode the request
        let EncodedRequest {
            buffer, header_decoder, body_decoder, correlation_id
        } = self.encode_request(req)?;

        println!("Sending: {:?}", buffer);

        // Send the request
        let res = self.inner.send(buffer).await?;

        Ok(async move {
            // Wait for a response
            let mut buf = res.await?.into();

            println!("Received: {:?}", buf);

            // Decode the response
            let header = header_decoder.decode_compatible(&mut buf)?;
            if header.correlation_id != correlation_id {
                return Err(ClientError::CorrelationMismatch);
            }

            let body = body_decoder.decode_compatible(&mut buf)?;

            Ok(body)
        }.boxed())
    }
}

#[derive(Debug)]
pub enum ClientError {
    Shutdown,
    Timeout,
    CorrelationMismatch,
    Setup,
    Compatibility(CompatibilityError),
    Io(Error),
}

impl From<CompatibilityError> for ClientError {
    fn from(other: CompatibilityError) -> Self {
        ClientError::Compatibility(other)
    }
}

impl From<Error> for ClientError {
    fn from(other: Error) -> Self {
        ClientError::Io(other)
    }
}

impl From<Shutdown> for ClientError {
    fn from(_: Shutdown) -> Self {
        ClientError::Shutdown
    }
}

impl From<Elapsed> for ClientError {
    fn from(_: Elapsed) -> Self {
        ClientError::Timeout
    }
}

struct EncodedRequest<Req: Request> {
    buffer: Bytes,
    header_decoder: DecodeCompatible<RequestHeader>,
    body_decoder: DecodeCompatible<Req>,
    correlation_id: i32,
}

impl ClientState {
    fn encode_request<Req: Request>(&mut self, req: Req) -> Result<EncodedRequest<Req>, CompatibilityError> {
        let mut buf = BytesMut::new();

        // Determine versions
        let api_key = Req::Type::KEY;
        let ver_range = self.supported_versions.get(&api_key).ok_or(CompatibilityError::Downgrade)?;
        let api_version = Req::find_compatible_version(ver_range.min, ver_range.max)?;
        let correlation_id = self.correlation_id;
        self.correlation_id += 1;

        // Encode header
        let header = RequestHeader {
            api_key,
            api_version,
            correlation_id,
            client_id: None,
        };
        let header_decoder = header.encode_compatible(&mut buf, api_version)?;

        // Encode body
        let body_decoder = req.encode_compatible(&mut buf, api_version)?;

        Ok(EncodedRequest {
            buffer: buf.into(),
            header_decoder,
            body_decoder,
            correlation_id,
        })
    }
}

async fn connect_internal(addr: SocketAddr, config: ClientConfig) -> Result<ClientState, ClientError> {
    let tcp_stream = TcpStream::connect(addr).await?;
    let (tcp_read, tcp_write) = tokio::io::split(tcp_stream);

    let mut codec = LengthDelimitedCodec::builder();
    codec.max_frame_length(config.max_message_size.unwrap_or(MAX_FRAME_LENGTH));

    let framed_read = codec.new_read(tcp_read);
    let framed_write = codec.new_write(tcp_write);

    let result = Pipeline::new(framed_read, framed_write, config.max_concurrency)
        .timeout(config.request_timeout);

    let mut supported_versions = HashMap::new();
    supported_versions.insert(ApiVersions::KEY, VersionRange {
        min: 0,
        max: 0,
    });

    Ok(ClientState {
        inner: Box::new(result),
        correlation_id: 0,
        supported_versions,
    })
}

async fn connect_and_setup(addr: SocketAddr, config: ClientConfig) -> Result<ClientState, ClientError> {
    println!("Connecting to {}...", addr);
    let mut client = connect_internal(addr, config).await?;
    println!("Connected.");

    // Find out the supported API versions
    println!("Discovering supported API versions...");
    let supported_versions = client.call(ApiVersions1Request {}).await?;
    client.supported_versions = supported_versions.api_keys
        .ok_or(ClientError::Setup)?
        .into_iter()
        .map(|x| (x.api_key, VersionRange { min: x.min_version, max: x.max_version }))
        .collect();
    println!("Supported API versions:\n{:?}", client.supported_versions);

    println!("Client ready.");
    Ok(client)
}

pub fn connect(addr: SocketAddr, config: ClientConfig) -> EagerService<ClientState, ClientError> {
    EagerService::new(connect_and_setup(addr, config))
}
