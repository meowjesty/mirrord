use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::Hasher,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use tokio::sync::{
    broadcast,
    watch::{self, error::SendError},
};

#[cfg(unix)]
pub mod api;

/// Wrapper around `Vec<String>` that redacts its [`Debug`] output to avoid leaking environment
/// variable names into logs, while still serializing normally for the session monitor API.
#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct RedactedVarNames(pub Vec<String>);

impl core::fmt::Debug for RedactedVarNames {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RedactedVarNames")
            .field(&"<REDACTED>")
            .finish()
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MonitorEvent {
    FileOp {
        path: Option<String>,
        operation: String,
    },
    DnsQuery {
        host: String,
    },
    IncomingRequest {
        method: String,
        path: String,
        host: String,
    },
    OutgoingConnection {
        address: String,
        port: u16,
    },
    PortSubscription {
        port: u16,
        mode: String,
    },
    EnvVar {
        vars: RedactedVarNames,
    },
    LayerConnected {
        pid: u32,
        parent_pid: Option<u32>,
        process_name: String,
        cmdline: Vec<String>,
    },
    LayerDisconnected {
        pid: u32,
    },
}

/// Wrapper around an optional broadcast sender for session monitor events.
///
/// When the session monitor is disabled, this wraps `None` and all emit calls are no-ops.
#[derive(Clone)]
pub struct MonitorTx {
    inner: Option<broadcast::Sender<MonitorEvent>>,
}

impl MonitorTx {
    pub fn disabled() -> Self {
        Self { inner: None }
    }

    pub fn emit(&self, event: MonitorEvent) {
        if let Some(tx) = &self.inner {
            let _ = tx.send(event);
        }
    }

    pub fn from_sender(tx: broadcast::Sender<MonitorEvent>) -> Self {
        Self { inner: Some(tx) }
    }

    pub fn subscribe(&self) -> Option<broadcast::Receiver<MonitorEvent>> {
        self.inner.as_ref().map(|tx| tx.subscribe())
    }
}

pub type TempChaosRuleType = ChaosRuleJsonThingy;
pub type TempChaosRules = HashSet<TempChaosRuleType>;

#[derive(Clone, Debug)]
pub struct ChaosRuleJsonThingy {
    kind: ChaosRuleKindThingy,
    pub hit_count: Arc<AtomicU32>,
}

impl PartialEq for ChaosRuleJsonThingy {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for ChaosRuleJsonThingy {}

impl core::hash::Hash for ChaosRuleJsonThingy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

impl Borrow<ChaosRuleKindThingy> for ChaosRuleJsonThingy {
    fn borrow(&self) -> &ChaosRuleKindThingy {
        &self.kind
    }
}

impl Serialize for ChaosRuleJsonThingy {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("ChaosRuleJsonThingy", 2)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("hit_count", &self.hit_count.load(Ordering::Relaxed))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ChaosRuleJsonThingy {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct TempType {
            kind: ChaosRuleKindThingy,
            #[serde(default)]
            hit_count: u32,
        }

        let TempType { kind, hit_count } = TempType::deserialize(deserializer)?;

        Ok(Self {
            kind,
            hit_count: Arc::new(AtomicU32::new(hit_count)),
        })
    }
}

#[derive(Clone, Default)]
pub struct ChaosWatcherTx(pub watch::Sender<TempChaosRules>);

impl ChaosWatcherTx {
    fn list_active_rules_for_session(&self) -> TempChaosRules {
        self.0.borrow().clone()
    }

    fn create_rule(&self, new_rule: ChaosRuleJsonThingy) {
        self.0.send_modify(|current_rules| {
            current_rules.insert(new_rule);
        });
    }

    fn clear_session_rules(&self) {
        self.0.send_replace(Default::default());
    }

    fn update_rule(&self, rule_id: String) {
        self.0.send_modify(|current_rules| {
            current_rules.replace(ChaosRuleJsonThingy {
                kind: ChaosRuleKindThingy::TcpOutgoingConnect,
                hit_count: Default::default(),
            });
        });
    }

    fn delete_rule(&self, rule_id: String) {
        self.0.send_modify(|current_rules| {
            current_rules.remove(&ChaosRuleKindThingy::TcpOutgoingConnect);
        });
    }

    fn get_rule(&self, rule_id: String) -> Option<ChaosRuleJsonThingy> {
        self.0
            .borrow()
            .get(&ChaosRuleKindThingy::TcpOutgoingConnect)
            .cloned()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum ChaosRuleKindThingy {
    TcpOutgoingConnect,
}

#[derive(Clone)]
pub struct ChaosWatcherRx(pub watch::Receiver<TempChaosRules>);

impl ChaosWatcherRx {
    pub fn get_rule(&self, rule: ChaosRuleKindThingy) -> Option<TempChaosRuleType> {
        let stored_rules = self.0.borrow();
        stored_rules
            .iter()
            .find(|r| matches!(&r.kind, rule))
            .cloned()
    }
}
