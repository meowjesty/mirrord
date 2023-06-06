use core::fmt;
use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
};

use mirrord_protocol::{
    outgoing::{DaemonConnect, DaemonRead, LayerClose, LayerConnect, LayerWrite, SocketAddress},
    ConnectionId,
};
use socket2::SockAddr;

use crate::{common::ResponseChannel, socket::id::SocketId};

pub(crate) mod tcp;
pub(crate) mod udp;

/// Wrapper type for the (layer) socket address that intercepts the user's socket messages.
///
/// (user-app) user_app_address <--> layer_address (layer) <--> agent <--> remote-peer
#[derive(Debug)]
pub(crate) struct RemoteConnection {
    /// The socket that is held by mirrord.
    pub(crate) layer_address: SockAddr,
    /// The socket that is held by the user application.
    pub(crate) user_app_address: SockAddr,
}

pub(crate) struct RecvFromPacket {
    pub(crate) bytes: Vec<u8>,
    pub(crate) source_address: SocketAddress,
}

impl fmt::Debug for RecvFromPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceivedPacket")
            .field("source_address", &self.source_address)
            .field("bytes (length)", &self.bytes.len())
            .finish()
    }
}

#[derive(Debug)]
pub(crate) struct SendToResponse {
    pub(crate) sent_amount: usize,
}

#[derive(Debug)]
pub(crate) struct Connect {
    pub(crate) remote_address: SockAddr,
    pub(crate) channel_tx: ResponseChannel<RemoteConnection>,
}

#[derive(Debug)]
pub(crate) struct RecvFrom {
    pub(crate) socket_id: SocketId,
    pub(crate) channel_tx: ResponseChannel<RecvFromPacket>,
}

pub(crate) struct SendTo {
    pub(crate) destination: SocketAddr,
    pub(crate) bytes: Vec<u8>,
    pub(crate) channel_tx: ResponseChannel<SendToResponse>,
}

impl fmt::Debug for SendTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendTo")
            .field("destination", &self.destination)
            .field("bytes (length)", &self.bytes.len())
            .finish()
    }
}

pub(crate) struct Write {
    pub(crate) connection_id: ConnectionId,
    pub(crate) bytes: Vec<u8>,
}

impl fmt::Debug for Write {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Write")
            .field("id", &self.connection_id)
            .field("bytes (length)", &self.bytes.len())
            .finish()
    }
}

/// Wrapper type around `tokio::Sender`, used to send messages from the `agent` to our interceptor
/// socket, where they'll be written back to the user's socket.
///
/// (agent) -> (layer) -> (user)
#[derive(Debug)]
pub(crate) struct ConnectionMirror(tokio::sync::mpsc::Sender<Vec<u8>>);

impl Deref for ConnectionMirror {
    type Target = tokio::sync::mpsc::Sender<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConnectionMirror {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
