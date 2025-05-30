use mirrord_protocol::{
    tcp::{StealType, TcpData},
    ConnectionId, Port,
};
use tokio::sync::mpsc::Sender;

use crate::util::ClientId;

mod api;
mod connection;
mod connections;
mod http;
mod subscriptions;
mod tls;

pub(crate) use api::{StealerMessage, TcpStealerApi};
pub(crate) use connection::TcpConnectionStealer;
pub(crate) use tls::StealTlsHandlerStore;

use self::http::HttpResponseFallback;

/// Commands from the agent that are passed down to the stealer worker, through [`TcpStealerApi`].
///
/// These are the operations that the agent receives from the layer to make the _steal_ feature
/// work.
#[derive(Debug)]
enum Command {
    /// Contains the channel that's used by the stealer worker to respond back to the agent
    /// (stealer -> agent -> layer).
    NewClient(Sender<StealerMessage>),

    /// A layer wants to subscribe to this [`Port`].
    ///
    /// The agent starts stealing traffic on this [`Port`].
    PortSubscribe(StealType),

    /// A layer wants to unsubscribe from this [`Port`].
    ///
    /// The agent stops stealing traffic from this [`Port`].
    PortUnsubscribe(Port),

    /// Unsubscribes the layer from the connection.
    ///
    /// The agent stops sending incoming traffic.
    ConnectionUnsubscribe(ConnectionId),

    /// There is new data in the direction going from the local process to the end-user (Going
    /// via the layer and the agent  local-process -> layer --> agent --> end-user).
    ///
    /// Agent forwards this data to the other side of original connection.
    ResponseData(TcpData),

    /// Response from local app to stolen HTTP request.
    ///
    /// Should be forwarded back to the connection it was stolen from.
    HttpResponse(HttpResponseFallback),

    SwitchProtocolVersion(semver::Version),
}

/// Association between a client (identified by the `client_id`) and a [`Command`].
///
/// The (agent -> worker) channel uses this, instead of naked [`Command`]s when communicating.
#[derive(Debug)]
pub struct StealerCommand {
    /// Identifies which layer instance is sending the [`Command`].
    client_id: ClientId,

    /// The command message sent from (layer -> agent) to be handled by the stealer worker.
    command: Command,
}
