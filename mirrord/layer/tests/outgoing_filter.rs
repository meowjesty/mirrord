#![feature(assert_matches)]
#![warn(clippy::indexing_slicing)]

use std::{net::SocketAddr, path::PathBuf, time::Duration};

use futures::{SinkExt, StreamExt, TryStreamExt};
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

    let (mut test_process, mut layer_connection) = Application::RustOutgoingFilter
        .start_process_with_layer(dylib_path, vec![], config)
        .await;

    // layer_connection
    //     .expect_xstat(Some("/etc/resolv.conf".into()), None)
    //     .await;
    let msg = layer_connection.consume_xstats().await;

    // Should we call `codec.next` or was it called outside already?
    // open file
    // let open_file_request = layer_connection.codec.next().await.unwrap().unwrap();
    let open_file_request = msg;

    assert_eq!(
        open_file_request,
        ClientMessage::FileRequest(mirrord_protocol::FileRequest::Open(
            mirrord_protocol::file::OpenFileRequest {
                path: PathBuf::from("/etc/hostname"),
                open_options: mirrord_protocol::file::OpenOptionsInternal {
                    read: true,
                    write: false,
                    append: false,
                    truncate: false,
                    create: false,
                    create_new: false
                }
            }
        ))
    );
    layer_connection.answer_file_open().await;

    // read file
    let read_request = layer_connection
        .codec
        .next()
        .await
        .expect("Read request success!")
        .expect("Read request exists!");
    assert_eq!(
        read_request,
        ClientMessage::FileRequest(mirrord_protocol::FileRequest::Read(
            mirrord_protocol::file::ReadFileRequest {
                remote_fd: 0xb16,
                buffer_size: 256,
            }
        ))
    );

    layer_connection
        .answer_file_read(b"metalbear-hostname".to_vec())
        .await;

    // TODO(alex): Add a wait time here, we can end up in the "Close request success" error.
    // close file (very rarely?).
    let close_request = layer_connection
        .codec
        .next()
        .await
        .expect("Close request success!")
        .expect("Close request exists!");

    println!("Should be a close file request: {read_request:#?}");
    assert_eq!(
        close_request,
        ClientMessage::FileRequest(mirrord_protocol::FileRequest::Close(
            mirrord_protocol::file::CloseFileRequest { fd: 0xb16 }
        ))
    );

    let mut connection = layer_connection.codec;
    let msg = connection.try_next().await.unwrap().unwrap();
    let ClientMessage::GetAddrInfoRequest(GetAddrInfoRequest { node}) = msg else {
            panic!("Invalid message received from layer: {msg:?}");
        };
    assert_eq!(node, "metalbear-hostname".to_string());

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
    assert_eq!(node, "metalbear-hostname".to_string());

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
