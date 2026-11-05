use super::*;

#[test]
fn nested_probe_backings_share_identity_only_for_aliases_of_the_same_grant() {
    let runtime = ic_blob_storage::ic_storage::testing::fixture_runtime(
        ProbeBackingMemory::default(),
        "probe",
        "probe.",
        &["probe.first.v1", "probe.second.v1"],
        StorageConfig::default(),
    )
    .unwrap();
    let memory = Rc::new(
        runtime
            .open_memory(&StableKey::parse("probe.first.v1").expect("valid allocation key"))
            .unwrap(),
    );
    let wrap = |memory| ProbeMemory {
        memory,
        fault: None,
        index: 0,
    };
    let first = wrap(Rc::clone(&memory));
    let alias = wrap(memory);
    let second = wrap(Rc::new(
        runtime
            .open_memory(&StableKey::parse("probe.second.v1").expect("valid allocation key"))
            .unwrap(),
    ));
    assert_eq!(first.backing_identity(), alias.backing_identity());
    assert_eq!(first.backing_identity(), first.backing_identity());
    assert_ne!(first.backing_identity(), second.backing_identity());
}
