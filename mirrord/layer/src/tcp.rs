/// Tcp Traffic management, common code for stealing & mirroring
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    net::SocketAddr,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use bimap::BiMap;
use mirrord_protocol::{
    tcp::{DaemonTcp, HttpRequest, NewTcpConnection, TcpClose, TcpData},
    ClientMessage, Port, ResponseError,
};
use tokio::{net::TcpStream, sync::mpsc::Sender};
use tracing::{debug, error, info};

use crate::{
    detour::DetourGuard,
    error::LayerError,
    socket::{id::SocketId, SocketInformation, CONNECTION_QUEUE},
    LayerError::{PortAlreadyStolen, UnexpectedResponseError},
};

#[derive(Debug)]
pub(crate) enum TcpIncoming {
    Listen(Listen),
}

#[derive(Debug, Clone)]
pub(crate) struct Listen {
    /// _fd_-independent identifier for a socket.
    pub id: SocketId,

    /// Address requested by the user's program, i.e. `127.0.0.1:80`.
    pub requested_address: SocketAddr,

    /// Address of our interceptor socket, i.e. `UNSPECIFIED:{random}`.
    pub mirror_address: SocketAddr,
}

impl PartialEq for Listen {
    fn eq(&self, other: &Self) -> bool {
        self.requested_address == other.requested_address
    }
}

impl Eq for Listen {}

impl Hash for Listen {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requested_address.hash(state);
    }
}

impl From<&Listen> for SocketAddr {
    fn from(listen: &Listen) -> Self {
        let address = if listen.mirror_address.is_ipv6() {
            SocketAddr::new(
                IpAddr::V6(Ipv6Addr::LOCALHOST),
                listen.mirror_address.port(),
            )
        } else {
            SocketAddr::new(
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                listen.mirror_address.port(),
            )
        };

        debug_assert_eq!(address.port(), listen.mirror_address.port());
        address
    }
}

pub(crate) trait TcpHandler {
    fn ports(&self) -> &HashMap<Port, Listen>;
    fn ports_mut(&mut self) -> &mut HashMap<Port, Listen>;
    fn port_mapping_ref(&self) -> &BiMap<u16, u16>;

    /// Modify `Listen` to match local port to remote port based on mapping
    /// If no mapping is found, the port is not modified
    fn apply_port_mapping(&self, listen: &mut Listen) {
        if let Some(mapped_port) = self
            .port_mapping_ref()
            .get_by_left(&listen.requested_address.port())
        {
            debug!(
                "mapping address {} to port {mapped_port}",
                &listen.requested_address
            );
            listen.requested_address.set_port(*mapped_port);
        }
    }

    /// Returns true to let caller know to keep running
    #[tracing::instrument(level = "debug", skip(self))]
    async fn handle_daemon_message(&mut self, message: DaemonTcp) -> Result<(), LayerError> {
        let handled = match message {
            DaemonTcp::NewConnection(tcp_connection) => {
                self.handle_new_connection(tcp_connection).await
            }
            DaemonTcp::Data(tcp_data) => self.handle_new_data(tcp_data).await,
            DaemonTcp::Close(tcp_close) => self.handle_close(tcp_close),
            DaemonTcp::SubscribeResult(Ok(port)) => {
                // Added this so tests can know when traffic can be sent
                debug!("daemon subscribed to port {port}.");
                Ok(())
            }
            DaemonTcp::SubscribeResult(Err(ResponseError::PortAlreadyStolen(port))) => {
                error!("Port subscription failed with for port {port}.");
                Err(PortAlreadyStolen(port))
            }
            DaemonTcp::SubscribeResult(Err(other_error)) => {
                error!("Port subscription failed with unexpected error: {other_error}.");
                Err(UnexpectedResponseError(other_error))
            }
            DaemonTcp::HttpRequest(request) => {
                self.handle_http_request(request).await.map_err(From::from)
            }
        };

        debug!("handle_incoming_message -> handled {:#?}", handled);

        handled
    }

    #[tracing::instrument(level = "trace", skip(self, tx))]
    async fn handle_hook_message(
        &mut self,
        message: TcpIncoming,
        tx: &Sender<ClientMessage>,
    ) -> Result<(), LayerError> {
        match message {
            TcpIncoming::Listen(listen) => self.handle_listen(listen, tx).await,
        }
    }

    /// Handle NewConnection messages
    async fn handle_new_connection(&mut self, conn: NewTcpConnection) -> Result<(), LayerError>;

    /// Connects to the local listening socket, add it to the queue and return the stream.
    /// Find better name
    #[tracing::instrument(level = "debug", skip(self))]
    async fn create_local_stream(
        &mut self,
        tcp_connection: &NewTcpConnection,
    ) -> Result<TcpStream, LayerError> {
        let remote_destination_port = tcp_connection.destination_port;
        let local_destination_port = self
            .port_mapping_ref()
            .get_by_right(&tcp_connection.destination_port)
            .map(|p| {
                debug!("mapping port {} to {p}", &tcp_connection.destination_port);
                *p
            })
            .unwrap_or(tcp_connection.destination_port);

        info!(
            "\nget {:#?}, and ports are {:#?}\n",
            SocketAddr::new(tcp_connection.remote_address, remote_destination_port),
            self.ports(),
        );

        let listen = self
            .ports()
            .get(&remote_destination_port)
            .ok_or(LayerError::PortNotFound(remote_destination_port))?;

        let addr: SocketAddr = listen.into();
        info!("address to connect to {addr:#?}");

        let info = SocketInformation::new(
            SocketAddr::new(tcp_connection.remote_address, tcp_connection.source_port),
            // we want local so app won't know we did the mapping.
            SocketAddr::new(tcp_connection.local_address, local_destination_port),
        );

        CONNECTION_QUEUE.add(listen.id, info);

        #[allow(clippy::let_and_return)]
        let tcp_stream = {
            let _ = DetourGuard::new();
            TcpStream::connect(addr).await.map_err(From::from)
        };

        tcp_stream
    }

    /// Handle New Data messages
    async fn handle_new_data(&mut self, data: TcpData) -> Result<(), LayerError>;

    /// Handle New Data messages
    async fn handle_http_request(&mut self, request: HttpRequest) -> Result<(), LayerError>;

    /// Handle connection close
    fn handle_close(&mut self, close: TcpClose) -> Result<(), LayerError>;

    /// Handle listen request
    async fn handle_listen(
        &mut self,
        listen: Listen,
        tx: &Sender<ClientMessage>,
    ) -> Result<(), LayerError>;
}
