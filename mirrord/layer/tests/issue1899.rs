#![feature(assert_matches)]
#![warn(clippy::indexing_slicing)]
use std::{path::PathBuf, time::Duration};

use futures::{stream::StreamExt, SinkExt};
use mirrord_protocol::{file::*, *};
use rstest::rstest;

mod common;

pub use common::*;

/// Verify that issue [#1899](https://github.com/metalbear-co/mirrord/issues/1899) is fixed.
///
/// Test the `opendir` hook by calling it and asserting that `*DIR` is not `null`.
#[rstest]
#[tokio::test]
#[timeout(Duration::from_secs(60))]
async fn test_issue1899(
    #[values(Application::RustIssue1899)] application: Application,
    dylib_path: &PathBuf,
) {
    let (mut test_process, mut layer_connection) = application
        .start_process_with_layer(
            dylib_path,
            vec![("MIRRORD_FILE_READ_WRITE_PATTERN", "/tmp")],
            None,
        )
        .await;

    let fd = 1;

    layer_connection
        .expect_file_open_with_options(
            "/tmp",
            fd,
            OpenOptionsInternal {
                read: true,
                write: false,
                append: false,
                truncate: false,
                create: false,
                create_new: false,
            },
        )
        .await;

    // Rust compiles with newer libc on Linux that uses statx
    #[cfg(target_os = "macos")]
    {
        // lstat test
        assert_eq!(
            layer_connection.codec.next().await.unwrap().unwrap(),
            ClientMessage::FileRequest(FileRequest::Xstat(XstatRequest {
                path: Some("/tmp".to_string().into()),
                fd: None,
                follow_symlink: false
            }))
        );

        let metadata = MetadataInternal {
            device_id: 0,
            size: 1,
            user_id: 2,
            blocks: 3,
            ..Default::default()
        };
        layer_connection
            .codec
            .send(DaemonMessage::File(FileResponse::Xstat(Ok(
                XstatResponse { metadata },
            ))))
            .await
            .unwrap();

        // fstat test
        assert_eq!(
            layer_connection.codec.next().await.unwrap().unwrap(),
            ClientMessage::FileRequest(FileRequest::Xstat(XstatRequest {
                path: Some("/tmp".to_string().into()),
                fd: None,
                follow_symlink: true
            }))
        );

        let metadata = MetadataInternal {
            device_id: 4,
            size: 5,
            user_id: 6,
            blocks: 7,
            ..Default::default()
        };
        layer_connection
            .codec
            .send(DaemonMessage::File(FileResponse::Xstat(Ok(
                XstatResponse { metadata },
            ))))
            .await
            .unwrap();
    }

    assert_eq!(
        layer_connection.codec.next().await.unwrap().unwrap(),
        ClientMessage::FileRequest(FileRequest::FdOpenDir(FdOpenDirRequest { remote_fd: 1 }))
    );

    layer_connection
        .codec
        .send(DaemonMessage::File(FileResponse::OpenDir(Ok(
            OpenDirResponse { fd: 2 },
        ))))
        .await
        .unwrap();

    test_process.wait_assert_success().await;
    test_process
        .assert_stdout_contains("test issue 1899: START")
        .await;
    test_process
        .assert_stdout_contains("test issue 1899: SUCCESS")
        .await;
}