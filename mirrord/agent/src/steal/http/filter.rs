use core::time::Duration;
use std::{net::SocketAddr, sync::Arc};

use dashmap::DashMap;
use fancy_regex::Regex;
use hyper::server::conn::http1;
use mirrord_protocol::ConnectionId;
use tokio::{
    io::{copy_bidirectional, duplex, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
    sync::mpsc::Sender,
    time::timeout,
};
use tracing::{error, info};

use super::{error::HttpTrafficError, hyper_handler::HyperHandler, HttpVersion};
use crate::{
    steal::{http::error, HandlerHttpRequest},
    util::ClientId,
};

const H2_PREFACE: &[u8] = b"PRI * HTTP/2.0";
const DEFAULT_HTTP_VERSION_DETECTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Controls the amount of data we read when trying to detect if the stream's first message contains
/// an HTTP request.
///
/// **WARNING**: Can't be too small, otherwise we end up accepting things like "Foo " as valid HTTP
/// requests.
pub(super) const MINIMAL_HEADER_SIZE: usize = 10;

/// Used to set up the creation of a [`HyperHandler`] task for the HTTP traffic stealer.
#[derive(Debug)]
pub(super) struct HttpFilterTask {
    http_version: HttpVersion,
    original_destination: SocketAddr,
    connection_id: ConnectionId,
    client_filters: Arc<DashMap<ClientId, Regex>>,
    matched_tx: Sender<HandlerHttpRequest>,

    /// For informing the stealer task that the connection was closed.
    connection_close_sender: Sender<ConnectionId>,
    stolen_connection: TcpStream,
    stolen_bytes: Vec<u8>,
}

impl HttpFilterTask {
    // TODO(alex) [high] 2023-01-13: We need to keep the bytes that we initially send to this
    // `DuplexStream`, otherwise hyper would get a messed up packet (as we remove data from the
    // stream here with read).
    //
    // And we need to re-send this data to the `hyper_stream` or the `passthrough_stream` somehow.
    //
    // Probably need a bit more fiddling to make this whole thing work.
    //
    /// Does not consume bytes from the stream.
    ///
    /// Checks if the first available bytes in a stream could be of an http request.
    ///
    /// This is a best effort classification, not a guarantee that the stream is HTTP.
    // #[tracing::instrument(level = "trace", skip_all)]
    pub(super) async fn new(
        stolen_connection: TcpStream,
        original_destination: SocketAddr,
        connection_id: ConnectionId,
        filters: Arc<DashMap<ClientId, Regex>>,
        matched_tx: Sender<HandlerHttpRequest>,
        connection_close_sender: Sender<ConnectionId>,
    ) -> Result<Self, HttpTrafficError> {
        let (read_stream, write_stream) = tokio::io::split(stolen_connection);

        let (read_stream, read_buffer) =
            timeout(DEFAULT_HTTP_VERSION_DETECTION_TIMEOUT, async move {
                let mut limited_read_stream = read_stream.take(MINIMAL_HEADER_SIZE as u64);
                let mut minimal_read_buffer = [0; MINIMAL_HEADER_SIZE];

                let mut total_read = 0;
                while total_read < MINIMAL_HEADER_SIZE {
                    let amount = limited_read_stream
                        .read(&mut minimal_read_buffer[total_read..])
                        .await?;
                    total_read += amount;
                }

                let read_stream = limited_read_stream.into_inner();

                Ok::<_, HttpTrafficError>((read_stream, minimal_read_buffer[..total_read].to_vec()))
            })
            .await??;

        let http_version = HttpVersion::new(&read_buffer, &H2_PREFACE[..MINIMAL_HEADER_SIZE]);

        let stolen_connection = read_stream.unsplit(write_stream);

        // TODO(alex) [high] 2023-01-16: Send the bytes we have to read to the `hyper_stream` after
        // hyper handler is set up.

        Ok(Self {
            stolen_connection,
            stolen_bytes: read_buffer,
            http_version,
            client_filters: filters,
            original_destination,
            connection_id,
            matched_tx,
            connection_close_sender,
        })
    }

    /// Creates the hyper task, and returns an [`HttpFilter`] that contains the channels we use to
    /// pass the requests to the layer.
    #[tracing::instrument(
        level = "debug",
        skip(self),
        fields(
            self.http_version = ?self.http_version,
            self.client_filters = ?self.client_filters,
            self.connection_id = ?self.connection_id,
            self.original_destination = ?self.original_destination,
        )
    )]
    pub(super) fn start(self) -> Result<(), HttpTrafficError> {
        match self.http_version {
            HttpVersion::V1 => {
                tokio::task::spawn(async move { self.steal_http1().await });
                Ok(())
            }
            // TODO(alex): hyper handling of HTTP/2 requires a bit more work, as it takes an
            // "executor" (just `tokio::spawn` in the `Builder::new` function is good enough), and
            // some more effort to chase some missing implementations.
            HttpVersion::V2 | HttpVersion::NotHttp => {
                let _passhtrough_task = tokio::task::spawn(async move {
                    self.steal_passthrough().await.inspect_err(|fail| {
                        error!(
                            "Something went wrong in the passthrough traffic handler!\n{fail:#?}"
                        )
                    })
                });

                Ok(())
            }
        }
    }

    async fn steal_http1(self) -> Result<(), HttpTrafficError> {
        let Self {
            mut stolen_connection,
            stolen_bytes,
            http_version: _,
            client_filters,
            matched_tx,
            connection_id,
            original_destination,
            connection_close_sender,
        } = self;

        let port = original_destination.port();
        let (mut stealer_stream, hyper_stream) = duplex(15000);

        // TODO: do we need to do something with this result?
        let _hyper_task = tokio::spawn(async move {
            let _ = http1::Builder::new()
                .preserve_header_case(true)
                .serve_connection(
                    hyper_stream,
                    HyperHandler {
                        filters: client_filters,
                        matched_tx,
                        connection_id,
                        port,
                        original_destination,
                        request_id: 0,
                    },
                )
                .with_upgrades()
                .await;

            connection_close_sender
                            .send(connection_id)
                            .await
                            .inspect_err(|connection_id| {
                                error!("Main TcpConnectionStealer dropped connection close channel while HTTP filter is still running. \
                                Cannot report the closing of connection {connection_id}.");
                            })?;

            Ok::<_, HttpTrafficError>(())
        });

        {
            {
                {}
            }
        }

        // Send the bytes we took when checking for HTTP traffic.
        stealer_stream.write(&stolen_bytes).await?;

        let mut hyper_buffer = vec![0; 15000];
        let mut remote_buffer = vec![0; 15000];

        // TODO(alex) [high] 2023-01-16: Now we need to keep track of where we are in these buffers,
        // so that we can do the upgrade->passthrough case properly, as we need to re-send the bytes
        // that hyper ate to detect the upgrade request.

        loop {
            select! {
                read_from_hyper = stealer_stream.read(&mut hyper_buffer) => {
                    let read_amount = read_from_hyper?;
                    stolen_connection.write(&hyper_buffer[..read_amount]).await?;
                }

                read_from_remote = stolen_connection.read(&mut remote_buffer) => {
                    let read_amount = read_from_remote?;
                    stealer_stream.write(&remote_buffer[..read_amount]).await?;
                }

                else => {
                    info!("How do we even get here?");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn steal_passthrough(self) -> Result<(), HttpTrafficError> {
        let Self {
            mut stolen_connection,
            stolen_bytes,
            original_destination,
            ..
        } = self;

        let mut interceptor_to_original = TcpStream::connect(original_destination).await?;

        // Send the bytes we took when checking for HTTP traffic.
        interceptor_to_original.write(&stolen_bytes).await?;

        copy_bidirectional(&mut stolen_connection, &mut interceptor_to_original).await?;
        Ok::<_, error::HttpTrafficError>(())
    }
}
