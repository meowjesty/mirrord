use std::{
    net::{SocketAddr, SocketAddrV4},
    str::FromStr,
    sync::atomic::Ordering,
};

use mirrord_intproxy_protocol::ProxyToLayerMessage;
use mirrord_protocol::{
    ClientMessage, DaemonMessage, ErrorKindInternal, RemoteError, RemoteIOError, ResponseError,
    outgoing::{
        self, DaemonConnectV2, SocketAddress,
        tcp::{self, DaemonTcpOutgoing, LayerTcpOutgoing},
    },
};

use crate::{
    background_tasks::MessageBusInner,
    main_tasks::{ProxyMessage, ToLayer},
    proxies::outgoing::OutgoingProxyMessage,
    session_monitor::ChaosRuleKindThingy,
};

impl MessageBusInner<OutgoingProxyMessage, ProxyMessage> {
    /// Attempts to send a message to this task's parent.
    pub(super) async fn send_chaos(&self, to_layer: ToLayer) {
        match &to_layer.message {
            lol => (),
        };

        let _ = self.tx.send(to_layer.into()).await;
    }

    /// Sends a message to the agent connection task.
    pub(super) async fn send_agent_chaos(&self, client_message: ClientMessage) {
        match &client_message {
            ClientMessage::TcpOutgoing(LayerTcpOutgoing::ConnectV2(v2))
                if let Some(chaos_rule) = self
                    .chaos_rx
                    .get_rule(ChaosRuleKindThingy::TcpOutgoingConnect) =>
            {
                let chaos_dunked =
                    DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::ConnectV2(DaemonConnectV2 {
                        uid: v2.uid,
                        connect: Err(ResponseError::Remote(RemoteError::ConnectTimedOut(
                            SocketAddress::Ip(SocketAddr::from_str("0.0.0.0:8000").unwrap()),
                        ))),
                    }));

                let was_stored = chaos_rule.hit_count.fetch_add(1, Ordering::Relaxed);
                tracing::info!(?was_stored, "are we hitting it only once in a while?");

                self.send(chaos_dunked).await;
                return;
            }
            lol => (),
        }

        self.agent_tx.send(client_message).await
    }
}
