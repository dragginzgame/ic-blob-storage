//! Bounded read occupancy and exact local callback identity; no paid effect permit.
pub(crate) mod record;
use super::ReadTarget;
use crate::model::gateway::registry::GatewayScope;
use candid::{CandidType, Deserialize};
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::num::{NonZeroU32, NonZeroU64};
use thiserror::Error;

/// Trusted concurrent read limits, distinct from upload bytes and lifetime history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ReadSessionLimits {
    pub(crate) sessions: u32,
    pub(crate) tenant_sessions: u32,
    pub(crate) reply_bytes: u32,
    pub(crate) bytes: u64,
    pub(crate) tenant_bytes: u64,
}
impl ReadSessionLimits {
    /// Construct an explicit envelope. At most 1024 simultaneous records are supported.
    /// Reply bytes bound one transport buffer, not total decoding or canister memory.
    /// # Errors
    /// Rejects contradictory count/byte limits or an unsupported session envelope.
    pub fn new(
        sessions: NonZeroU32,
        tenant_sessions: NonZeroU32,
        reply_bytes: NonZeroU32,
        bytes: NonZeroU64,
        tenant_bytes: NonZeroU64,
    ) -> Result<Self, ReadSessionError> {
        let value = Self {
            sessions: sessions.get(),
            tenant_sessions: tenant_sessions.get(),
            reply_bytes: reply_bytes.get(),
            bytes: bytes.get(),
            tenant_bytes: tenant_bytes.get(),
        };
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn validate(self) -> Result<(), ReadSessionError> {
        ic_blob_storage_contracts::configuration::envelope::validate_read_limits(
            ic_blob_storage_contracts::dto::configuration::ServiceReadInput {
                sessions: self.sessions,
                tenant_sessions: self.tenant_sessions,
                reply_bytes: self.reply_bytes,
                bytes: self.bytes,
                tenant_bytes: self.tenant_bytes,
            },
        )
        .map_err(|_| ReadSessionError::Limits)?;
        Ok(())
    }
    /// Conservative reservation for each admitted reply buffer.
    #[must_use]
    pub const fn max_reply_bytes(self) -> u32 {
        self.reply_bytes
    }
}
/// Exact chunk selection; authority and range validation are separate checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadChunkTarget {
    /// Tenant/object/reference and selected gateway.
    pub target: ReadTarget,
    /// Zero-based complete manifest chunk.
    pub index: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadSessionIntent {
    pub context: UploadContext,
    pub scope: GatewayScope,
    pub chunk: ReadChunkTarget,
    pub gateway_generation: u64,
    pub tenant_generation: NonZeroU64,
}
/// Host-retained exact callback identity. Never deserialize this from ingress.
/// Clones identify the same occupied slot; completion succeeds at most once.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadSessionTicket {
    pub(crate) sequence: u64,
    pub(crate) intent: ReadSessionIntent,
}
impl ReadSessionTicket {
    /// Original target for exact transport binding, not authority to dispatch.
    #[must_use]
    pub const fn chunk(&self) -> ReadChunkTarget {
        self.intent.chunk
    }
}
/// Read-only accounting, with no completion token or recovery authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadSessionUsage {
    /// Concurrent occupied records, including invalidated or abandoned calls.
    pub sessions: u32,
    /// Reserved transport buffer bytes; not upload quota or total Wasm memory.
    pub reserved_bytes: u64,
}
/// Bounded local journal rejection; binary decoding and write traps are separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ReadSessionError {
    /// Unsupported or internally contradictory trusted limits.
    #[error("invalid read session limits")]
    Limits,
    /// Installation cannot replace allocated memory.
    #[error("read session memory allocated")]
    AlreadyAllocated,
    /// Restoration cannot initialize missing memory.
    #[error("read session memory missing")]
    Missing,
    /// Original service/operator/provider/namespace or installed limits differ.
    #[error("read session binding mismatch")]
    Binding,
    /// Caller is neither the original tenant nor the requested operator observer.
    #[error("read session authority denied")]
    Denied,
    /// Global or per-tenant occupancy or byte budget is full.
    #[error("read session capacity exhausted")]
    Capacity,
    /// No safe unused local sequence remains; exact completion still works.
    #[error("read session identity exhausted")]
    Exhausted,
    /// Exact session no longer exists, or a callback differs from retained intent.
    #[error("stale read session")]
    Stale,
    /// Local stored state or accounting is inconsistent.
    #[error("invalid read session record")]
    InvalidRecord,
    /// Restoration retains occupancy under a permanent inspection-only fence.
    #[error("read sessions fenced")]
    Fenced,
    /// Selected chunk is outside the confirmed declaration.
    #[error("invalid read chunk index")]
    Chunk,
}
