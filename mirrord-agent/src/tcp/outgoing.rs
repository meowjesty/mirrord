use core::fmt;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf};

use futures::future::OptionFuture;
use mirrord_protocol::{
    tcp::{outgoing::*, DaemonTcp, TcpData},
    ConnectionId, RemoteResult,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    select,
    sync::mpsc::{self, Receiver, Sender},
    task,
};
use tokio_stream::{StreamExt, StreamMap};
use tokio_util::io::ReaderStream;
use tracing::{debug, error, trace, warn};

use crate::{error::AgentError, runtime::set_namespace};

type Request = TcpOutgoingRequest;
type Response = TcpOutgoingResponse;

pub(crate) struct TcpOutgoingApi {
    pub(crate) daemon_tx: Sender<DaemonTcp>,
    write_streams: HashMap<ConnectionId, OwnedWriteHalf>,
    read_streams: StreamMap<ConnectionId, ReaderStream<OwnedReadHalf>>,
}

impl TcpOutgoingApi {
    pub(crate) fn new(daemon_tx: Sender<DaemonTcp>) -> Self {
        // TODO(alex) [high] 2022-08-19: Has to be diferent from `steal`, as we don't have a port
        // here (the paramter: remote address).
        Self {
            daemon_tx,
            write_streams: HashMap::default(),
            read_streams: StreamMap::default(),
        }
    }

    pub(crate) fn start(
        pid: Option<u64>,
        request_rx: Receiver<Request>,
        daemon_tx: Sender<DaemonTcp>,
    ) {
        std::thread::spawn(|| {
            if let Some(pid) = pid {
                let namespace = PathBuf::from("/proc")
                    .join(PathBuf::from(pid.to_string()))
                    .join(PathBuf::from("ns/net"));

                set_namespace(namespace).unwrap();
            }

            let mut outgoing = TcpOutgoingApi::new(daemon_tx);
            // TODO(alex) [high] 2022-08-19: There is no `remote_stream` here yet, as we have no
            // connection requests.
            outgoing.main_task(request_rx, remote_stream);
        });
    }

    pub(crate) async fn main_task(
        &mut self,
        mut request_rx: Receiver<TcpOutgoingRequest>,
        remote_stream: TcpStream,
    ) -> Result<(), AgentError> {
        loop {
            select! {
                request = request_rx.recv() => {
                    if let Some(request) = request {
                        self.handle_request(request).await?;
                    } else {
                        debug!("main_task -> request_rx closed!");
                        break;
                    }
                },
                tcp_message = self.recv_next() => {
                    if let Some(tcp_message) = tcp_message {
                        self.daemon_tx.send(tcp_message).await?;
                    }
                }
            }
        }
        debug!("TCP Stealer exiting");
        Ok(())
    }

    async fn handle_request(&mut self, request: TcpOutgoingRequest) -> Result<(), AgentError> {
        match request {
            TcpOutgoingRequest::Connect(ConnectRequest { remote_address }) => {
                let response = TcpStream::connect(remote_address)
                    .await
                    .map_err(AgentError::from)
                    .map(|remote_stream| {
                        let connection_id = self
                            .read_streams
                            .keys()
                            .copied()
                            .last()
                            .map(|last| last + 1)
                            .unwrap_or_default();

                        let (read_half, write_half) = remote_stream.into_split();

                        self.read_streams
                            .insert(connection_id, ReaderStream::new(read_half));
                        self.write_streams.insert(connection_id, write_half);

                        ConnectResponse {
                            connection_id,
                            remote_address,
                        }
                    })?;

                Ok(self
                    .daemon_tx
                    .send(DaemonTcp::ConnectResponse(response))
                    .await?)
            }
            TcpOutgoingRequest::Write(WriteRequest {
                connection_id,
                bytes,
            }) => match self.write_streams.get_mut(&connection_id) {
                Some(stream) => Ok(stream.write_all(&bytes[..]).await?),
                None => {
                    warn!(
                        "handle_request -> Trying to send data to closed connection {:#?}",
                        connection_id
                    );

                    Ok(())
                }
            },
        }
    }

    /// Reads the mesage sent by the remote host, and prepares it to be sent back to layer.
    async fn recv_next(&mut self) -> Option<DaemonTcp> {
        let (connection_id, value) = self.read_streams.next().await?;

        value
            .inspect_err(|fail| {
                error!(
                    "next -> Failed `next` call for connection_id {:#?} with {:#?}!",
                    connection_id, fail
                )
            })
            .ok()
            .map(|bytes| {
                DaemonTcp::Data(TcpData {
                    connection_id,
                    bytes: bytes.to_vec(),
                })
            })
    }

    async fn interceptor_task(
        connection_id: i32,
        response_tx: Sender<Response>,
        mut write_rx: Receiver<Data>,
        stream: TcpStream,
    ) {
        trace!("intercept_task -> connection_id {:#?}", connection_id);

        let mut buffer = vec![0; 1500];
        let (mut remote_reader, mut remote_writer) = stream.into_split();

        loop {
            select! {
                biased;

                read = remote_reader.read(&mut buffer) => {
                    match read {
                        Ok(read_amount) if read_amount == 0 => {
                            warn!("intercept_task -> Read stream is closed!");
                            break;
                        }
                        Ok(read_amount) => {
                            let bytes = buffer[..read_amount].to_vec();

                            let read = ReadResponse {
                                connection_id,
                                bytes,
                            };

                            let response = TcpOutgoingResponse::Read(Ok(read));
                            debug!("interceptor_task -> read response {:#?}", response);

                            if let Err(fail) = response_tx.send(response).await {
                                error!("intercept_task -> Failed sending response with {:#?}", fail);
                                break;
                            }
                        }
                        Err(ref fail) if fail.kind() == std::io::ErrorKind::WouldBlock => continue,
                        Err(fail) => {
                            error!("Failed reading stream with {:#?}", fail);
                            break;
                        }
                    }
                }

                write = write_rx.recv() => {
                    match write {
                        Some(data) => {
                            let result = remote_writer.write_all(&data.bytes).await;

                            match result {
                                Err(fail) if fail.kind() == std::io::ErrorKind::WouldBlock => continue,
                                Err(fail) => {
                                    error!("Failed writing stream with {:#?}", fail);
                                    break;
                                }
                                Ok(()) => {
                                    let write = WriteResponse {
                                        connection_id,
                                    };
                                    let response = TcpOutgoingResponse::Write(Ok(write));
                                    debug!("interceptor_task -> write response {:#?}", response);

                                    if let Err(fail) = response_tx.send(response).await {
                                        error!("Failed sending read message with {:#?}!", fail);
                                        break;
                                    }
                                }
                            }
                        }
                        None => {
                            warn!("intercept_task-> write_rx closed {:#?}!", connection_id);
                            break;
                        }
                    }
                }
            }
        }

        trace!("intercept_task -> Finished id {:#?}", connection_id);
    }

    async fn request_task(
        pid: Option<u64>,
        mut request_rx: Receiver<Request>,
        response_tx: Sender<Response>,
    ) -> Result<(), AgentError> {
        if let Some(pid) = pid {
            let namespace = PathBuf::from("/proc")
                .join(PathBuf::from(pid.to_string()))
                .join(PathBuf::from("ns/net"));

            set_namespace(namespace).unwrap();
        }

        let mut senders: HashMap<i32, Sender<Data>> = HashMap::with_capacity(4);

        loop {
            // [layer] -> [agent]
            match request_rx.recv().await {
                Some(request) => {
                    trace!("inner_request_handler -> request {:?}", request);

                    match request {
                        TcpOutgoingRequest::Connect(ConnectRequest { remote_address }) => {
                            let connect_response: RemoteResult<_> =
                                TcpStream::connect(remote_address)
                                    .await
                                    .map_err(From::from)
                                    .map(|remote_stream| {
                                        let (write_tx, write_rx) = mpsc::channel(1000);

                                        let connection_id = senders
                                            .keys()
                                            .copied()
                                            .last()
                                            .map(|last| last + 1)
                                            .unwrap_or_default();

                                        senders.insert(connection_id, write_tx.clone());

                                        task::spawn(Self::interceptor_task(
                                            connection_id,
                                            response_tx.clone(),
                                            write_rx,
                                            remote_stream,
                                        ));

                                        ConnectResponse {
                                            connection_id,
                                            remote_address,
                                        }
                                    });

                            trace!("Connect -> response {:#?}", connect_response);

                            let response = TcpOutgoingResponse::Connect(connect_response);
                            response_tx.send(response).await?
                        }
                        TcpOutgoingRequest::Write(WriteRequest {
                            connection_id,
                            bytes,
                        }) => {
                            trace!("Write -> request {:#?}", connection_id);

                            let write = Data {
                                connection_id,
                                bytes,
                            };

                            senders.get(&connection_id).unwrap().send(write).await?
                        }
                    }
                }
                None => {
                    warn!("run -> Disconnected!");
                    break;
                }
            }
        }

        Ok(())
    }

    pub(crate) async fn request(&mut self, request: TcpOutgoingRequest) -> Result<(), AgentError> {
        Ok(self.request_channel_tx.send(request).await?)
    }

    pub(crate) async fn response(&mut self) -> Result<TcpOutgoingResponse, AgentError> {
        self.response_channel_rx
            .recv()
            .await
            .ok_or(AgentError::ReceiverClosed)
    }
}
