#![feature(assert_matches)]
#![warn(clippy::indexing_slicing)]

use std::{net::SocketAddr, path::PathBuf, time::Duration};

use mirrord_protocol::{
    dns::{DnsLookup, GetAddrInfoRequest, GetAddrInfoResponse, LookupRecord},
    outgoing::{
        tcp::{DaemonTcpOutgoing, LayerTcpOutgoing},
        DaemonConnect, DaemonRead, LayerConnect, LayerWrite, SocketAddress,
    },
    ClientMessage, DaemonMessage,
};
use rstest::rstest;

mod common;

pub use common::*;
use futures::{SinkExt, TryStreamExt};
use trust_dns_resolver::lookup::Lookup;

async fn outgoing_filter_hostname_matches(
    with_config: Option<&str>,
    dylib_path: &PathBuf,
    config_dir: &PathBuf,
) {
    let config = with_config.map(|config| {
        let mut config_path = config_dir.clone();
        config_path.push(config);
        config_path
    });
    let config = config.as_ref().map(|path_buf| path_buf.to_str().unwrap());

    let (mut test_process, layer_connection) = Application::RustOutgoingTcp
        .start_process_with_layer(dylib_path, vec![], config)
        .await;
    let mut conn = layer_connection.codec;

    let peers = RUST_OUTGOING_PEERS
        .split(',')
        .map(|s| s.parse::<SocketAddr>().unwrap())
        .collect::<Vec<_>>();

    for peer in peers {
        let msg = conn.try_next().await.unwrap().unwrap();
        let ClientMessage::TcpOutgoing(LayerTcpOutgoing::Connect(LayerConnect { remote_address: SocketAddress::Ip(addr) })) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
        assert_eq!(addr, peer);
        conn.send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Connect(Ok(
            DaemonConnect {
                connection_id: 0,
                remote_address: addr.into(),
                local_address: RUST_OUTGOING_LOCAL.parse::<SocketAddr>().unwrap().into(),
            },
        ))))
        .await
        .unwrap();

        let msg = conn.try_next().await.unwrap().unwrap();
        let ClientMessage::TcpOutgoing(LayerTcpOutgoing::Write(LayerWrite { connection_id: 0, bytes })) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
        conn.send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Read(Ok(
            DaemonRead {
                connection_id: 0,
                bytes,
            },
        ))))
        .await
        .unwrap();
        conn.send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Close(0)))
            .await
            .unwrap();
    }

    test_process.wait_assert_success().await;
}

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
async fn outgoing_filter_remote_hostname_matches(
    #[values(None, Some("outgoing_filter_hostname.json"))] with_config: Option<&str>,
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
    let ClientMessage::TcpOutgoing(LayerTcpOutgoing::Write(LayerWrite { connection_id: 0, bytes })) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
    connection
        .send(DaemonMessage::TcpOutgoing(DaemonTcpOutgoing::Read(Ok(
            DaemonRead {
                connection_id: 0,
                bytes,
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

#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(10))]
#[should_panic]
async fn outgoing_tcp_from_the_local_app_broken(
    #[values(
        Some("outgoing_filter_local.json"),
        Some("outgoing_filter_remote_incomplete.json")
    )]
    with_config: Option<&str>,
    dylib_path: &PathBuf,
    config_dir: &PathBuf,
) {
    outgoing_tcp_logic(with_config, dylib_path, config_dir).await;
}
