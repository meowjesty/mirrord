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
    pub name: String,
    pub id: String,
    pub port: u16,
}
