//! Typed binary contract for the Logos daemon.
mod error;

pub use error::Error;
use rkyv::{Archive, Deserialize, Serialize};
use signal_sema_storage::{ContentHash, FixtureScope, SlotSummary, SubscriptionIdentifier};
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Request {
    ProjectRust {
        scope: FixtureScope,
        logos: ContentHash,
    },
    List {
        scope: FixtureScope,
    },
    Subscribe {
        scope: FixtureScope,
    },
}
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjectionEvent {
    pub logos: ContentHash,
    pub rust: String,
    pub source: SlotSummary,
}
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    RustProjected {
        rust: String,
        source: SlotSummary,
    },
    Listed(Vec<SlotSummary>),
    Subscribed {
        identifier: SubscriptionIdentifier,
        initial: Vec<SlotSummary>,
    },
    Event(ProjectionEvent),
    Rejected(Rejection),
}
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rejection {
    LogosNotFound,
    WrongDocumentKind,
    ProjectionFailed,
    StorageFailed,
}
pub fn encode_request(value: &Request) -> Result<Vec<u8>, Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .map(|bytes| bytes.to_vec())
        .map_err(Error::from)
}
pub fn encode_reply(value: &Reply) -> Result<Vec<u8>, Error> {
    rkyv::to_bytes::<rkyv::rancor::Error>(value)
        .map(|bytes| bytes.to_vec())
        .map_err(Error::from)
}
