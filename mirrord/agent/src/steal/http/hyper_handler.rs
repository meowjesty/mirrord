use core::{future::Future, pin::Pin};
use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use dashmap::DashMap;
use fancy_regex::Regex;
use futures::TryFutureExt;
use http_body_util::{BodyExt, Full};
use hyper::{body::Incoming, client, http, service::Service, Request, Response};
use mirrord_protocol::{ConnectionId, Port, RequestId};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::Sender,
        oneshot::{self, Receiver},
    },
};
use tracing::error;

use super::error::HttpTrafficError;
use crate::{
    steal::{HandlerHttpRequest, MatchedHttpRequest},
    util::ClientId,
};

/// Used to pass data to the [`Service`] implementation of [`hyper`].
///
/// Think of this struct as if it was a bunch of function arguments that are passed down to
/// [`Service::call`] method.
///
/// Each [`TcpStream`] connection (for stealer) requires this.
#[derive(Debug)]
pub(super) struct HyperHandler {
    /// The (shared with the stealer) HTTP filter regexes that are used to filter traffic for this
    /// particular connection.
    pub(super) filters: Arc<DashMap<ClientId, Regex>>,

    /// [`Sender`] part of the channel used to communicate with the agent that we have a
    /// [`MatchedHttpRequest`], and it should be forwarded to the layer.
    pub(super) matched_tx: Sender<HandlerHttpRequest>,

    /// Identifies this [`TcpStream`] connection.
    pub(crate) connection_id: ConnectionId,

    /// The port we're filtering HTTP traffic on.
    pub(crate) port: Port,

    /// The original [`SocketAddr`] of the connection we're intercepting.
    ///
    /// Used for the case where we have an unmatched request (HTTP request did not match any of the
    /// `filters`).
    pub(crate) original_destination: SocketAddr,

    /// Keeps track of which HTTP request we're dealing with, so we don't mix up [`Request`]s.
    pub(crate) request_id: RequestId,
}

/// Sends a [`MatchedHttpRequest`] through `tx` to be handled by the stealer -> layer.
#[tracing::instrument(level = "trace", skip(matched_tx, response_rx))]
async fn matched_request(
    request: HandlerHttpRequest,
    matched_tx: Sender<HandlerHttpRequest>,
    response_rx: Receiver<Response<Full<Bytes>>>,
) -> Result<Response<Full<Bytes>>, HttpTrafficError> {
    matched_tx
        .send(request)
        .map_err(HttpTrafficError::from)
        .await?;

    let (mut parts, body) = response_rx.await?.into_parts();
    parts.headers.remove(http::header::CONTENT_LENGTH);
    parts.headers.remove(http::header::TRANSFER_ENCODING);

    Ok(Response::from_parts(parts, body))
}

/// Handles the case when no filter matches a header in the request.
///
/// 1. Creates a [`hyper::client::conn::http1::Connection`] to the `original_destination`;
/// 2. Sends the [`Request`] to it, and awaits a [`Response`];
/// 3. Sends the [`HttpResponse`] back on the connected [`TcpStream`].
#[tracing::instrument(level = "trace")]
async fn unmatched_request(
    request: Request<Incoming>,
    original_destination: SocketAddr,
) -> Result<Response<Full<Bytes>>, HttpTrafficError> {
    // TODO(alex): We need a "retry" mechanism here for the client handling part, when the server
    // closes a connection, the client could still be wanting to send a request, so we need to
    // re-connect and send.
    let tcp_stream = TcpStream::connect(original_destination)
        .await
        .inspect_err(|fail| error!("Failed connecting to original_destination with {fail:#?}"))?;

    let (mut request_sender, connection) = client::conn::http1::handshake(tcp_stream)
        .await
        .inspect_err(|fail| error!("Handshake failed with {fail:#?}"))?;

    // We need this to progress the connection forward (hyper thing).
    tokio::spawn(async move {
        if let Err(fail) = connection.await {
            error!("Connection failed in unmatched with {fail:#?}");
        }
    });

    // Send the request to the original destination.
    let (mut parts, body) = request_sender
        .send_request(request)
        .await
        .inspect_err(|fail| error!("Failed hyper request sender with {fail:#?}"))?
        .into_parts();

    // Remove headers that would be invalid due to us fiddling with the `body`.
    let body = body.collect().await?.to_bytes();
    parts.headers.remove(http::header::CONTENT_LENGTH);
    parts.headers.remove(http::header::TRANSFER_ENCODING);

    // Rebuild the `Response` after our fiddling.
    Ok(Response::from_parts(parts, body.into()))
}

impl Service<Request<Incoming>> for HyperHandler {
    type Response = Response<Full<Bytes>>;

    type Error = HttpTrafficError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    #[tracing::instrument(level = "trace", skip(self))]
    fn call(&mut self, request: Request<Incoming>) -> Self::Future {
        self.request_id += 1;

        if let Some(client_id) = request
            .headers()
            .iter()
            .map(|(header_name, header_value)| {
                header_value
                    .to_str()
                    .map(|header_value| format!("{}: {}", header_name, header_value))
            })
            .find_map(|header| {
                self.filters.iter().find_map(|filter| {
                    // TODO(alex) [low] 2022-12-23: Remove the `header` unwrap.
                    if filter.is_match(header.as_ref().unwrap()).unwrap() {
                        Some(*filter.key())
                    } else {
                        None
                    }
                })
            })
        {
            let req = MatchedHttpRequest {
                port: self.port,
                connection_id: self.connection_id,
                client_id,
                request_id: self.request_id,
                request,
            };

            let (response_tx, response_rx) = oneshot::channel();
            let handler_request = HandlerHttpRequest {
                request: req,
                response_tx,
            };

            Box::pin(matched_request(
                handler_request,
                self.matched_tx.clone(),
                response_rx,
            ))
        } else {
            Box::pin(unmatched_request(request, self.original_destination))
        }
    }
}
