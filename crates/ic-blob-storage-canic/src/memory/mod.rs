//! Reuse the host's committed runtime; never bootstrap or choose a bucket policy.
use ic_blob_storage::{
    ic_memory::{
        MemoryRequest, RuntimeMemory, RuntimeOpenError, SchemaMetadata,
        StaticMemoryDeclarationError, ic_stable_structures::DefaultMemoryImpl,
    },
    ops::service::{
        installation::{INSTALLATION_MEMORY_KEY, ServiceInstallationMemories},
        stores::grants,
    },
};

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
/// Missing authority/grants do not initialize a manager or any service store.
/// The artifact must give these handles exclusively to one installation owner.
/// # Errors
/// Preserves upstream absence, missing-key and runtime inspection errors.
pub fn open() -> Result<ServiceInstallationMemories<ManagedMemory>, RuntimeOpenError> {
    ic_blob_storage::ic_memory::committed_allocations()?;
    Ok(ServiceInstallationMemories {
        configuration: ic_blob_storage::ic_memory::open_default_memory_manager_memory_by_key(
            INSTALLATION_MEMORY_KEY,
        )?,
        stores: grants::open_default()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_framework_runtime_is_not_constructed_or_registered_by_linkage() {
        std::thread::spawn(|| {
            assert!(matches!(open(), Err(RuntimeOpenError::NotBootstrapped)));
            assert!(matches!(
                ic_blob_storage::ic_memory::default_memory_manager_memory_allocations(),
                Err(ic_blob_storage::ic_memory::RuntimeDiagnosticError::NotBootstrapped)
            ));
            let snapshot = ic_blob_storage::ic_memory::sealed_declaration_snapshot().unwrap();
            assert!(
                !snapshot
                    .requests()
                    .iter()
                    .any(|r| r.stable_key().as_str().starts_with("blob."))
            );
        })
        .join()
        .unwrap();
    }
}
