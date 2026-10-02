//! Named service grants within the host's existing `ic-memory` runtime.
//!
//! Hosts explicitly include these requests in their composed declaration snapshot,
//! grant an authority range and bootstrap once before opening. Linking this module
//! registers nothing. Configuration memory and lifecycle remain host-owned.
use super::ServiceMemories;
use crate::ops::service::{
    funding::FundingMemories, reads::ReadSessionMemories, uploads::UploadMemories,
};
use ic_memory::{
    MemoryRequest, RuntimeMemory, RuntimeOpenError, SchemaMetadata, StaticMemoryDeclarationError,
    ic_stable_structures::{DefaultMemoryImpl, Memory},
};

// Logical identities are independent of host-selected physical IDs. Keep this
// mapping shared so an adapter cannot accidentally swap accounting and history.
const KEYS: [&str; 16] = [
    "blob.tenants.v1",
    "blob.roots.v1",
    "blob.root_objects.v1",
    "blob.permissions.v1",
    "blob.usage.v1",
    "blob.manifests.v1",
    "blob.confirmed.v1",
    "blob.references.v1",
    "blob.receipts.v1",
    "blob.root_requests.v1",
    "blob.funding_accounting.v1",
    "blob.funding_intents.v1",
    "blob.gateways.v1",
    "blob.read_journal.v1",
    "blob.read_sessions.v1",
    "blob.read_tenants.v1",
];

/// Build the sixteen current service requests under an explicit host authority.
/// No registration, range grant, runtime bootstrap or stable write occurs.
/// Hosts compose these with their configuration and other application requests
/// before sealing; the host's policy owns placement and authority validation.
/// # Errors
/// Rejects an authority unsupported by `ic-memory`.
pub fn requests(authority: &str) -> Result<Vec<MemoryRequest>, StaticMemoryDeclarationError> {
    KEYS.into_iter()
        .map(|key| MemoryRequest::new(authority, key, SchemaMetadata::default()))
        .collect()
}

/// Open all service grants using the host's committed-memory lookup by durable key.
/// Supply an existing runtime's `open_memory_by_key` or `ic-memory`'s default
/// committed lookup after checking bootstrap when a framework owns that runtime.
/// No second runtime, ID selection, initialization, repair or store write occurs.
/// A missing member returns no assembled set. Hosts must retain the runtime and
/// exclusively give these memories to one service owner; handles do not provide
/// independent freshness or permission to activate a restored installation.
/// # Errors
/// Preserves the host lookup's errors for an unbootstrapped runtime or missing grant.
/// # Panics
/// Only an inconsistent internal key count can panic.
pub fn open<M: Memory, E>(
    mut lookup: impl FnMut(&str) -> Result<M, E>,
) -> Result<ServiceMemories<M>, E> {
    let memories = KEYS
        .into_iter()
        .map(&mut lookup)
        .collect::<Result<Vec<_>, _>>()?;
    let mut memories = memories.into_iter();
    let [
        tenants,
        roots,
        objects,
        permissions,
        usage,
        manifests,
        confirmed,
        references,
        receipts,
        root_requests,
        accounting,
        intents,
        gateways,
        journal,
        sessions,
        read_tenants,
    ] = std::array::from_fn::<_, 16, _>(|_| memories.next().expect("fixed service grant count"));
    Ok(ServiceMemories {
        uploads: UploadMemories {
            tenants,
            roots,
            objects,
            permissions,
            usage,
            manifests,
            confirmed,
            references,
            receipts,
            root_requests,
        },
        funding: FundingMemories {
            accounting,
            intents,
        },
        gateways,
        reads: ReadSessionMemories {
            journal,
            sessions,
            tenants: read_tenants,
        },
    })
}

/// Open service grants from the already bootstrapped default runtime, for a host
/// that owns it. Checks the existing committed capability first;
/// absence cannot construct a manager or silently choose its bucket policy.
/// No declarations, lifecycle hooks, store initialization or recovery activation
/// are implied. The host must grant exclusive service ownership as for [`open`].
/// # Errors
/// Rejects an absent/unbootstrapped runtime, missing grant or runtime access failure.
pub fn open_default() -> Result<ServiceMemories<RuntimeMemory<DefaultMemoryImpl>>, RuntimeOpenError>
{
    ic_memory::committed_allocations()?;
    open(ic_memory::open_default_memory_manager_memory_by_key)
}

#[cfg(test)]
mod tests;
