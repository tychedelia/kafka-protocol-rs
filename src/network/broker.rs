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

use protocol_base::{VersionRange, Request, Encodable, Decodable, EncodeError, DecodeError, HeaderVersion};

use super::messages::{RequestHeader, ResponseHeader, ApiVersionsRequest};
use super::service::{Service, ServiceBase, ServiceExt, BoxService, RecvFuture, Pipeline, Shutdown, EagerService};

const MAX_FRAME_LENGTH: usize = std::i32::MAX as usize;

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
            buffer, api_version, correlation_id
        } = self.encode_request(req)?;

        println!("Sending: {:?}", buffer);

        // Send the request
        let res = self.inner.send(buffer).await?;

        Ok(async move {
            // Wait for a response
            let mut buf: Bytes = res.await?.into();

            println!("Received: {:?}", buf);

            // Decode the response
            let response_header_version = Req::Response::header_version(api_version);
            let header = ResponseHeader::decode(&mut buf, response_header_version)?;
            if header.correlation_id != correlation_id {
                return Err(ClientError::CorrelationMismatch);
            }

            let body = Req::Response::decode(&mut buf, api_version)?;

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
    Encode(EncodeError),
    Decode(DecodeError),
    Io(Error),
}

impl From<EncodeError> for ClientError {
    fn from(other: EncodeError) -> Self {
        ClientError::Encode(other)
    }
}

impl From<DecodeError> for ClientError {
    fn from(other: DecodeError) -> Self {
        ClientError::Decode(other)
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

struct EncodedRequest {
    buffer: Bytes,
    api_version: i16,
    correlation_id: i32,
}

impl ClientState {
    fn encode_request<Req: Request>(&mut self, req: Req) -> Result<EncodedRequest, EncodeError> {
        let mut buf = BytesMut::new();

        // Determine versions
        let api_key = Req::KEY;
        let server_range = self.supported_versions.get(&api_key).ok_or(EncodeError)?;
        let supported_range = server_range.intersect(&Req::VERSIONS);
        if supported_range.is_empty() {
            return Err(EncodeError);
        }
        let api_version = supported_range.max;

        let correlation_id = self.correlation_id;
        self.correlation_id += 1;

        // Construct header
        let mut header = RequestHeader::default();
        header.request_api_key = api_key;
        header.request_api_version = api_version;
        header.correlation_id = correlation_id;

        let request_header_version = Req::header_version(api_version);
        header.encode(&mut buf, request_header_version)?;

        // Encode body
        req.encode(&mut buf, api_version)?;

        Ok(EncodedRequest {
            buffer: buf.into(),
            api_version,
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
    supported_versions.insert(ApiVersionsRequest::KEY, VersionRange {
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
    let supported_versions = client.call(ApiVersionsRequest::default()).await?;
    client.supported_versions = supported_versions.api_keys
        .into_iter()
        .map(|(k, v)| (k, VersionRange { min: v.min_version, max: v.max_version }))
        .collect();
    println!("Supported API versions:\n{:?}", client.supported_versions);

    println!("Client ready.");
    Ok(client)
}

pub fn connect(addr: SocketAddr, config: ClientConfig) -> EagerService<ClientState, ClientError> {
    EagerService::new(connect_and_setup(addr, config))
}
