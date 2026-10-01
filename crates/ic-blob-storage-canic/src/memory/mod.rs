//! Reuse the host's committed runtime; never bootstrap or choose a bucket policy.
use ic_blob_storage::{
    ic_memory::{
        MemoryRequest, RuntimeAdoptionError, RuntimeMemory, RuntimeOpenError, SchemaMetadata,
        SealedDeclarationSnapshot, StaticMemoryDeclarationError,
        ic_stable_structures::DefaultMemoryImpl,
    },
    ops::service::{
        installation::{INSTALLATION_MEMORY_KEY, ServiceInstallationMemories},
        stores::grants,
    },
};
use thiserror::Error;

/// Failure to adopt the host's existing grants; no bootstrap or repair is attempted.
#[derive(Debug, Error)]
pub enum MemoryAdoptionFailure {
    /// The explicitly requested authority or declarations are invalid.
    #[error(transparent)]
    Declarations(#[from] StaticMemoryDeclarationError),
    /// Current committed authority, key or declaration metadata differs.
    #[error(transparent)]
    Adoption(#[from] RuntimeAdoptionError),
    /// A verified grant could not be opened.
    #[error(transparent)]
    Open(#[from] RuntimeOpenError),
}

/// Memory handle supplied by Canic's sole default runtime.
pub type ManagedMemory = RuntimeMemory<DefaultMemoryImpl>;

/// Build all seventeen shared requests, without registration or allocation.
/// # Errors
/// Rejects an invalid host-selected allocation authority.
pub fn requests(authority: &str) -> Result<Vec<MemoryRequest>, StaticMemoryDeclarationError> {
    let mut requests = vec![MemoryRequest::new(
        authority,
        INSTALLATION_MEMORY_KEY,
        SchemaMetadata::default(),
    )?];
    requests.extend(grants::requests(authority)?);
    Ok(requests)
}

/// Open the complete installation only after Canic has committed its memory grants.
/// Verify all seventeen declarations against the explicit expected authority before
/// opening any handle. Missing authority/grants do not initialize a manager or store.
/// The artifact must give these handles exclusively to one installation owner.
/// # Errors
/// Preserves upstream absence, missing-key and runtime inspection errors.
pub fn open(
    authority: &str,
) -> Result<ServiceInstallationMemories<ManagedMemory>, MemoryAdoptionFailure> {
    let requirements = SealedDeclarationSnapshot::new(&[], &[], &requests(authority)?)?;
    ic_blob_storage::ic_memory::verify_default_memory_manager_authority(&requirements, authority)?;
    Ok(ServiceInstallationMemories {
        configuration: ic_blob_storage::ic_memory::open_default_memory_manager_memory_by_key(
            INSTALLATION_MEMORY_KEY,
        )?,
        stores: grants::open_default()?,
    })
}

#[cfg(test)]
mod tests;
