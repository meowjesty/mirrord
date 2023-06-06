use mirrord_protocol::dns::RecvFromResponse;

use crate::common::ResponseChannel;

#[derive(Debug)]
pub(crate) struct RecvFrom {
    channel_tx: ResponseChannel<RecvFromResponse>,
}

#[derive(Debug)]
pub(crate) enum UdpIncoming {
    RecvFrom(RecvFrom),
}
