use mirrord_intproxy_protocol::ProxyToLayerMessage;
use mirrord_protocol::{
    ClientMessage, DaemonMessage, ResponseError,
    outgoing::{
        self,
        tcp::{self, DaemonTcpOutgoing, LayerTcpOutgoing},
    },
};

use crate::{
    background_tasks::MessageBusInner,
    main_tasks::{ProxyMessage, ToLayer},
    proxies::outgoing::OutgoingProxyMessage,
    session_monitor::ChaosRule,
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
                if let Some(chaos_rule) = self.chaos_rx.get_rule(ChaosRule::TcpOutgoingConnect) =>
            {
                let chaos_dunked = DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Connect(Err(
                    ResponseError::NotImplemented,
                )));

                self.send(chaos_dunked).await;
                return;
            }
            lol => (),
        }

        self.agent_tx.send(client_message).await
    }
}
