//! Component-only store assembly; no installation record or lifecycle authority.
use crate::ops::service::stores::{
    ServiceMemories, ServiceStoreConfiguration, ServiceStoreError, ServiceStores,
};
use ic_memory::ic_stable_structures::Memory;

/// Construct fixture owners in fresh exclusive memories.
/// # Errors
/// Preserves all ordinary fresh-store admission errors.
/// # Panics
/// Stable-write or aliased-memory traps must propagate for IC rollback.
pub fn install<M: Memory>(
    memory: ServiceMemories<M>,
    config: ServiceStoreConfiguration,
) -> Result<ServiceStores<M>, ServiceStoreError> {
    ServiceStores::install(memory, config)
}
/// Restore fixture owners synchronously, retaining their recovery fences.
/// # Errors
/// Preserves all ordinary missing, binding and retained-store errors.
/// # Panics
/// Corrupt records or collection headers trap.
pub fn open<M: Memory>(
    memory: ServiceMemories<M>,
    config: ServiceStoreConfiguration,
) -> Result<ServiceStores<M>, ServiceStoreError> {
    ServiceStores::open(memory, config)
}
