use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// TODO(alex) [high] 2026-04-07 4:
// MultiContainerThingy {
//  name: "catzor",
//  id: "docker://6a9cb1a79ae4e32182b2873b79281004e739ddf28903e11d338200b6ff8236a4",
//  port: 24000
// },
#[derive(
    Clone,
    Encode,
    Decode,
    Debug,
    Default,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct MultiContainerThingy {
    name: String,
    id: String,
    ephemeral: bool,
    port: u16,
}

impl MultiContainerThingy {
    pub fn new(name: String, id: String, ephemeral: bool, port: u16) -> Self {
        Self {
            name,
            id,
            ephemeral,
            port,
        }
    }

    pub fn id(&self) -> &str {
        &self
            .id
            .split_once(":")
            .expect("Should be in the format `[runtime]://[id]`")
            .1
    }

    pub fn runtime(&self) -> &str {
        &self
            .id
            .split_once(":")
            .expect("Should be in the format `[runtime]://[id]`")
            .0
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }
}
