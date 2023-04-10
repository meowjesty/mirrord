use std::net::SocketAddr;

use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct BindSocketRequest {
    pub address: SocketAddr,
    pub domain: i32,
    pub type_: i32,
    pub protocol: i32,
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct BindSocketResponse;
