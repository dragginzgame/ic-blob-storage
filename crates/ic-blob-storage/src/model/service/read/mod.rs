//! Exact read identity; this value alone grants no authority or session capacity.
use crate::model::{identity::ProviderRootHash, lifecycle::binding::ReferenceKey};
use candid::Principal;

/// Exact tenant-owned content/reference and selected current gateway.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadTarget {
    /// Root must resolve to the reference's full object lifetime.
    pub root: ProviderRootHash,
    /// This exact reference must remain live; another reference cannot replace it.
    pub reference: ReferenceKey,
    /// Host-selected provider member, not the tenant caller or an inferred source.
    pub gateway: Principal,
}
