#![feature(assert_matches)]
#![warn(clippy::indexing_slicing)]

use std::{net::SocketAddr, path::PathBuf, time::Duration};

use mirrord_protocol::{
    dns::{DnsLookup, GetAddrInfoRequest, GetAddrInfoResponse, LookupRecord},
    outgoing::{
        tcp::{DaemonTcpOutgoing, LayerTcpOutgoing},
        DaemonConnect, LayerConnect, SocketAddress,
    },
    ClientMessage, DaemonMessage,
};
use rstest::rstest;

mod common;

pub use common::*;
use futures::{SinkExt, TryStreamExt};

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn outgoing_filter_remote_hostname_matches(
    #[values(Some("outgoing_filter_remote_hostname_matches.json"))] with_config: Option<&str>,
    dylib_path: &PathBuf,
    config_dir: &PathBuf,
) {
    let config = with_config.map(|config| {
        let mut config_path = config_dir.clone();
        config_path.push(config);
        config_path
    });
    let config = config.as_ref().map(|path_buf| path_buf.to_str().unwrap());

    let (mut test_process, layer_connection) = Application::RustOutgoingFilter
        .start_process_with_layer(dylib_path, vec![], config)
        .await;
    let mut connection = layer_connection.codec;

    let msg = connection.try_next().await.unwrap().unwrap();
    let ClientMessage::GetAddrInfoRequest(GetAddrInfoRequest { node}) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
    assert_eq!(node, "service.name".to_string());

    connection
        .send(DaemonMessage::GetAddrInfoResponse(GetAddrInfoResponse(Ok(
            DnsLookup(vec![LookupRecord {
                name: node,
                ip: "1.2.3.4".parse().unwrap(),
            }]),
        ))))
        .await
        .unwrap();

    let msg = connection.try_next().await.unwrap().unwrap();
    let ClientMessage::GetAddrInfoRequest(GetAddrInfoRequest { node}) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
    assert_eq!(node, "service.name".to_string());

    connection
        .send(DaemonMessage::GetAddrInfoResponse(GetAddrInfoResponse(Ok(
            DnsLookup(vec![LookupRecord {
                name: node,
                ip: "1.2.3.4".parse().unwrap(),
            }]),
        ))))
        .await
        .unwrap();

    let msg = connection.try_next().await.unwrap().unwrap();
    let ClientMessage::TcpOutgoing(LayerTcpOutgoing::Connect( LayerConnect { remote_address })) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
    assert_eq!(
        SocketAddr::try_from(remote_address.clone()).unwrap(),
        "1.2.3.4:7777".parse::<SocketAddr>().unwrap()
    );

    connection
        .send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Connect(Ok(
            DaemonConnect {
                connection_id: 0,
                remote_address,
                local_address: SocketAddress::Ip("127.0.0.1:8888".parse().unwrap()),
            },
        ))))
        .await
        .unwrap();

    connection
        .send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Close(0)))
        .await
        .unwrap();

    test_process.wait_assert_success().await;
}
