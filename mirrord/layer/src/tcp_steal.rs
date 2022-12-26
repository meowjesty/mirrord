use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::client::conn::http1::{handshake, SendRequest};
use mirrord_protocol::{
    tcp::{
        HttpRequest, HttpResponse, LayerTcpSteal, NewTcpConnection,
        StealType::{All, FilteredHttp},
        TcpClose, TcpData,
    },
    ClientMessage, ConnectionId, Port,
};
use streammap_ext::StreamMap;
use tokio::{
    io::{AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc::{channel, Receiver, Sender},
};
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
use tracing::{debug, error, trace};

use crate::{
    error::LayerError,
    tcp::{Listen, TcpHandler},
};

pub(crate) mod http_forwarding;

use crate::{detour::DetourGuard, tcp_steal::http_forwarding::HttpForwarderError};

pub struct TcpStealHandler {
    ports: HashSet<Listen>,
    write_streams: HashMap<ConnectionId, WriteHalf<TcpStream>>,
    read_streams: StreamMap<ConnectionId, ReaderStream<ReadHalf<TcpStream>>>,

    /// Mapping of a ConnectionId to a sender that sends HTTP requests over to a task that is
    /// running an http client for this connection.
    http_request_senders: HashMap<ConnectionId, Sender<HttpRequest>>,

    /// Sender of responses from within an http client task back to the main layer task.
    /// This sender is cloned and moved into those tasks.
    http_response_sender: Sender<HttpResponse>,

    /// A string with a header regex to filter HTTP requests by.
    http_filter: Option<String>,
}

// TODO: let user specify http ports.
const HTTP_PORTS: [Port; 2] = [80, 8080];

#[async_trait]
impl TcpHandler for TcpStealHandler {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn handle_new_connection(
        &mut self,
        tcp_connection: NewTcpConnection,
    ) -> Result<(), LayerError> {
        let stream = self.create_local_stream(&tcp_connection).await?;

        let (read_half, write_half) = tokio::io::split(stream);
        self.write_streams
            .insert(tcp_connection.connection_id, write_half);
        self.read_streams
            .insert(tcp_connection.connection_id, ReaderStream::new(read_half));

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self), fields(data = data.connection_id))]
    async fn handle_new_data(&mut self, data: TcpData) -> Result<(), LayerError> {
        // TODO: "remove -> op -> insert" pattern here, maybe we could improve the overlying
        // abstraction to use something that has mutable access.
        let mut connection = self
            .write_streams
            .remove(&data.connection_id)
            .ok_or(LayerError::NoConnectionId(data.connection_id))?;

        trace!(
            "handle_new_data -> writing {:#?} bytes to id {:#?}",
            data.bytes.len(),
            data.connection_id
        );
        // TODO: Due to the above, if we fail here this connection is leaked (-agent won't be told
        // that we just removed it).
        connection.write_all(&data.bytes[..]).await?;

        self.write_streams.insert(data.connection_id, connection);

        Ok(())
    }

    /// An http request was stolen by the http filter. Pass it to the local application.
    #[tracing::instrument(level = "debug", skip(self))] // TODO: trace
    async fn handle_http_request(&mut self, request: HttpRequest) -> Result<(), LayerError> {
        self.forward_request(request).await
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn handle_close(&mut self, close: TcpClose) -> Result<(), LayerError> {
        let TcpClose { connection_id } = close;

        // Dropping the connection -> Sender drops -> Receiver disconnects -> tcp_tunnel ends
        let _ = self.read_streams.remove(&connection_id);
        let _ = self.write_streams.remove(&connection_id);
        let _ = self.http_request_senders.remove(&connection_id);

        Ok(())
    }

    fn ports(&self) -> &HashSet<Listen> {
        &self.ports
    }

    fn ports_mut(&mut self) -> &mut HashSet<Listen> {
        &mut self.ports
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn handle_listen(
        &mut self,
        listen: Listen,
        tx: &Sender<ClientMessage>,
    ) -> Result<(), LayerError> {
        let port = listen.requested_port;

        self.ports_mut()
            .insert(listen)
            .then_some(())
            .ok_or(LayerError::ListenAlreadyExists)?;

        let steal_type = if HTTP_PORTS.contains(&port) && let Some(filter) = &self.http_filter {
            FilteredHttp(port, filter.clone())
        } else {
            All(port)
        };
        tx.send(ClientMessage::TcpSteal(LayerTcpSteal::PortSubscribe(
            steal_type,
        )))
        .await
        .map_err(From::from)
    }
}

impl TcpStealHandler {
    pub(crate) fn new(
        http_filter: Option<String>,
        http_response_sender: Sender<HttpResponse>,
    ) -> Self {
        Self {
            ports: Default::default(),
            write_streams: Default::default(),
            read_streams: Default::default(),
            http_request_senders: Default::default(),
            http_response_sender,
            http_filter,
        }
    }

    #[tracing::instrument(level = "debug", skip(self))] // TODO: trace.
    pub async fn next(&mut self) -> Option<ClientMessage> {
        let (connection_id, value) = self.read_streams.next().await?;
        match value {
            Some(Ok(bytes)) => Some(ClientMessage::TcpSteal(LayerTcpSteal::Data(TcpData {
                connection_id,
                bytes: bytes.to_vec(),
            }))),
            Some(Err(err)) => {
                error!("connection id {connection_id:?} read error: {err:?}");
                None
            }
            None => Some(ClientMessage::TcpSteal(
                LayerTcpSteal::ConnectionUnsubscribe(connection_id),
            )),
        }
    }

    /// Send a filtered HTTP request to the application in the appropriate port.
    /// If this is the first filtered HTTP from its remote connection to arrive at this layer, a new
    /// local connection will be started for it, otherwise it will be sent in the existing local
    /// connection.
    #[tracing::instrument(level = "debug", skip(self))] // TODO: trace
    pub(crate) async fn forward_request(&mut self, request: HttpRequest) -> Result<(), LayerError> {
        if let Some(sender) = self.http_request_senders.get(&request.connection_id) {
            debug!(
                "Got an HTTP request from an existing connection, sending it to the client task \
                to be forwarded to the application."
            ); // TODO: trace.
            sender
                .send(request)
                .await
                .map_err::<HttpForwarderError, _>(From::from)?
        } else {
            self.create_http_connection(request).await?
        }
        Ok(())
    }

    /// Manage a single tcp connection, forward requests, wait for responses, send responses back.
    async fn connection_task(
        mut request_receiver: Receiver<HttpRequest>,
        mut http_request_sender: SendRequest<Full<Bytes>>,
        response_sender: Sender<HttpResponse>,
        port: Port,
        connection_id: ConnectionId,
    ) -> Result<(), HttpForwarderError> {
        // Listen for more requests in this connection and forward them to app.
        while let Some(req) = request_receiver.recv().await {
            debug!("HTTP client task received a new request to send: {req:?}."); // TODO: trace.
            let request_id = req.request_id;
            // Send to application.
            let res = http_request_sender.send_request(req.request.into()).await;
            // TODO: trace.
            debug!("HTTP client task sent request to local app and got response: {res:#?}.");
            let res =
                HttpResponse::from_hyper_response(res?, port, connection_id, request_id).await?;
            debug!("HTTP client task sending converted response to main task: {res:?}.");
            // Send response back to forwarder.
            response_sender.send(res).await?;
            debug!("HTTP client task done sending converted response to main task.");
        }
        Ok(())
    }

    /// Create a new TCP connection with the application to send all the filtered HTTP requests
    /// from this connection in.
    /// Spawn a task that receives requests on a channel and sends them to the application on that
    /// new TCP connection. The sender of that channel is stored in [`self.request_senders`].
    /// The responses from all the http client tasks will arrive together at
    /// [`self.response_receiver`].
    #[tracing::instrument(level = "debug", skip(self))] // TODO: trace.
    async fn create_http_connection(
        &mut self,
        http_request: HttpRequest,
    ) -> Result<(), LayerError> {
        let listen = self
            .ports()
            .get(&http_request.port)
            .ok_or(LayerError::PortNotFound(http_request.port))?;
        let addr: SocketAddr = listen.into();
        let target_stream = {
            let _ = DetourGuard::new();
            TcpStream::connect(addr).await?
        };
        let (sender, connection): (SendRequest<Full<Bytes>>, _) =
            handshake(target_stream)
                .await
                .map_err::<HttpForwarderError, _>(From::from)?;
        let connection_id = http_request.connection_id;
        let port = http_request.port;

        // spawn a task to poll the connection and drive the HTTP state
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!(
                    "Error in http connection {} on port {}: {}",
                    connection_id, port, e
                );
            }
        });

        let (request_sender, request_receiver) = channel(1024);

        let response_sender = self.http_response_sender.clone();

        tokio::spawn(async move {
            debug!("HTTP client task started."); // TODO: trace.
            if let Err(e) = Self::connection_task(
                request_receiver,
                sender,
                response_sender,
                port,
                connection_id,
            )
            .await
            {
                error!(
                    "Error while forwarding http connection {connection_id} (port {port}): {e:?}."
                )
            } else {
                debug!(
                    // TODO: trace.
                    "Filtered http connection {connection_id} (port {port}) closed without errors."
                )
            }
        });

        request_sender
            .send(http_request)
            .await
            .map_err::<HttpForwarderError, _>(From::from)?;
        // Give the forwarder a channel to send the task new requests from the same connection.
        self.http_request_senders
            .insert(connection_id, request_sender);

        debug!("main task done creating http connection."); // TODO: done.
        Ok(())
    }
}
