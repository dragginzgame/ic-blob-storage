use super::*;
use ic_blob_storage::ic_memory::{
    self, GenericRangePolicy, MemoryManagerConfig, MemoryManagerRangeMode,
    ic_stable_structures::Memory,
};
fn bootstrap(case: &str) {
    let authority = if case == "authority" { "other" } else { "blob" };
    let mut declared = requests(authority).unwrap();
    if case == "missing" {
        declared.pop();
    }
    if case == "metadata" {
        declared[0] = MemoryRequest::new(
            authority,
            INSTALLATION_MEMORY_KEY,
            SchemaMetadata::new(Some(1)).unwrap(),
        )
        .unwrap();
    }
    for request in declared {
        ic_memory::register_memory_request(request).unwrap();
    }
    ic_memory::register_memory_request(
        MemoryRequest::new(authority, "neighbor.data.v1", SchemaMetadata::default()).unwrap(),
    )
    .unwrap();
    ic_memory::register_static_memory_manager_range(
        100,
        117,
        authority,
        MemoryManagerRangeMode::Allowed,
        None,
    )
    .unwrap();
    ic_memory::bootstrap_default_memory_manager_with_config(
        MemoryManagerConfig::new(16).unwrap(),
        &GenericRangePolicy,
    )
    .unwrap();
    let neighbor =
        ic_memory::open_default_memory_manager_memory_by_key("neighbor.data.v1").unwrap();
    assert_eq!(neighbor.grow(1), Ok(0));
    neighbor.write(0, b"neighbor");
}
#[test]
fn framework_adoption_checks_authority_metadata_and_absence_without_effects() {
    // The linked registry is process-global with no reset. Each host runs independently.
    let Ok(case) = std::env::var("BLOB_MEMORY_ADOPTION_CASE") else {
        for case in ["absent", "authority", "missing", "metadata", "matching"] {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "memory::tests::framework_adoption_checks_authority_metadata_and_absence_without_effects", "--nocapture"])
                .env("BLOB_MEMORY_ADOPTION_CASE", case).status().unwrap();
            assert!(status.success(), "adoption case {case}");
        }
        return;
    };
    if case == "absent" {
        assert!(matches!(
            open("blob"),
            Err(MemoryAdoptionFailure::Adoption(RuntimeAdoptionError::Open(
                RuntimeOpenError::NotBootstrapped
            )))
        ));
        assert!(matches!(
            ic_memory::default_memory_manager_memory_allocation_summary(),
            Err(ic_memory::RuntimeDiagnosticError::NotBootstrapped)
        ));
        assert!(
            ic_memory::sealed_declaration_snapshot()
                .unwrap()
                .requests()
                .is_empty()
        );
        return;
    }
    bootstrap(&case);
    let before = ic_memory::default_memory_manager_memory_allocations().unwrap();
    match case.as_str() {
        "authority" => assert!(matches!(
            open("blob"),
            Err(MemoryAdoptionFailure::Adoption(
                RuntimeAdoptionError::AuthorityMismatch { .. }
            ))
        )),
        "missing" => assert!(matches!(
            open("blob"),
            Err(MemoryAdoptionFailure::Adoption(RuntimeAdoptionError::Open(
                RuntimeOpenError::StableKeyNotCommitted(_)
            )))
        )),
        "metadata" => assert!(matches!(
            open("blob"),
            Err(MemoryAdoptionFailure::Adoption(
                RuntimeAdoptionError::DeclarationMetadataMismatch { .. }
            ))
        )),
        "matching" => {
            let summary = ic_memory::default_memory_manager_memory_allocation_summary().unwrap();
            let memories = open("blob").unwrap();
            assert_eq!(memories.configuration.size(), 0);
            assert_eq!(
                ic_memory::default_memory_manager_memory_allocation_summary().unwrap(),
                summary
            );
            assert_eq!(summary.bucket_size_pages, 16);
        }
        _ => panic!("unknown adoption fixture"),
    }
    assert_eq!(
        ic_memory::default_memory_manager_memory_allocations().unwrap(),
        before
    );
    let neighbor =
        ic_memory::open_default_memory_manager_memory_by_key("neighbor.data.v1").unwrap();
    let mut bytes = [0; 8];
    neighbor.read(0, &mut bytes);
    assert_eq!(&bytes, b"neighbor");
}
