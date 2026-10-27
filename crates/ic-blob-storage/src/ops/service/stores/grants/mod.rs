//! Named service grants within the host's existing `ic-memory` runtime.
//!
//! Hosts explicitly include these requests in their composed declaration snapshot,
//! grant a namespace in the allocation pool and bootstrap once before opening. Linking this module
//! registers nothing. Configuration memory and lifecycle remain host-owned.
use super::ServiceMemories;
use crate::ops::service::{funding::FundingMemories, uploads::UploadMemories};
use ic_memory::{Memory, RuntimeMemory};
use ic_memory::{MemoryDeclarationError, MemoryRequest};

// Logical identities are independent of host-selected physical IDs. Keep this
// mapping shared so an adapter cannot accidentally swap accounting and history.
const KEYS: ServiceMemories<&str> = ServiceMemories {
    uploads: UploadMemories {
        tenants: "blob.tenants.v1",
        roots: "blob.roots.v1",
        objects: "blob.root_objects.v1",
        permissions: "blob.permissions.v1",
        usage: "blob.usage.v1",
        manifests: "blob.manifests.v1",
        confirmed: "blob.confirmed.v1",
        references: "blob.references.v1",
        receipts: "blob.receipts.v1",
        root_requests: "blob.root_requests.v1",
        deletions: "blob.pending_deletions.v1",
    },
    funding: FundingMemories {
        accounting: "blob.funding_accounting.v1",
        intents: "blob.funding_intents.v1",
    },
    gateways: "blob.gateways.v1",
};

/// Build the fourteen current service requests under an explicit host authority.
/// No registration, namespace grant, runtime bootstrap or stable write occurs.
/// Hosts compose these with their configuration and other application requests
/// before sealing; the host's policy owns placement and authority validation.
/// # Errors
/// Rejects an authority unsupported by `ic-memory`.
pub fn requests(authority: &str) -> Result<Vec<MemoryRequest>, MemoryDeclarationError> {
    KEYS.all()
        .into_iter()
        .map(|key| MemoryRequest::new(authority, key))
        .collect()
}

/// Open all service grants using the host's committed-memory lookup by durable key.
/// Supply the host-owned runtime's `open_memory` after its bootstrap.
/// No second runtime, ID selection, initialization, repair or store write occurs.
/// A missing member returns no assembled set. Hosts must retain the runtime and
/// exclusively give these memories to one service owner; handles do not provide
/// independent freshness or permission to activate a restored installation.
/// # Errors
/// Preserves the host lookup's errors for an unbootstrapped runtime or missing grant.
pub fn open<M: Memory, E>(
    mut lookup: impl FnMut(&str) -> Result<RuntimeMemory<M>, E>,
) -> Result<ServiceMemories<RuntimeMemory<M>>, E> {
    Ok(ServiceMemories {
        uploads: UploadMemories {
            tenants: lookup(KEYS.uploads.tenants)?,
            roots: lookup(KEYS.uploads.roots)?,
            objects: lookup(KEYS.uploads.objects)?,
            permissions: lookup(KEYS.uploads.permissions)?,
            usage: lookup(KEYS.uploads.usage)?,
            manifests: lookup(KEYS.uploads.manifests)?,
            confirmed: lookup(KEYS.uploads.confirmed)?,
            references: lookup(KEYS.uploads.references)?,
            receipts: lookup(KEYS.uploads.receipts)?,
            root_requests: lookup(KEYS.uploads.root_requests)?,
            deletions: lookup(KEYS.uploads.deletions)?,
        },
        funding: FundingMemories {
            accounting: lookup(KEYS.funding.accounting)?,
            intents: lookup(KEYS.funding.intents)?,
        },
        gateways: lookup(KEYS.gateways)?,
    })
}

#[cfg(test)]
mod tests;
