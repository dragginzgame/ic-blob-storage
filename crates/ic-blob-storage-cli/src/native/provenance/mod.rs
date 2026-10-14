//! Descriptive source provenance; never authority or a whole-binary attestation.
use ic_blob_storage_contracts::identity::ContentDigest;

const COMMON: &str = concat!(
    include_str!("mod.rs"),
    include_str!("../mod.rs"),
    include_str!("../arguments/mod.rs"),
    include_str!("../parsing/mod.rs"),
    include_str!("../exact_candid/mod.rs"),
    include_str!("../http/mod.rs"),
    include_str!("../artifacts/mod.rs"),
    include_str!("../signed_update/mod.rs"),
    include_str!("../../../../../Cargo.toml"),
    include_str!("../../../../../Cargo.lock"),
);

/// Bind common implementation and explicit command sources as separate digests.
pub(super) fn runner_source(command: &str) -> String {
    let common = ContentDigest::compute(COMMON.as_bytes());
    let command = ContentDigest::compute(command.as_bytes());
    ContentDigest::compute(
        format!("ic-blob-storage/native-source\n{common}\n{command}\n").as_bytes(),
    )
    .to_string()
}
