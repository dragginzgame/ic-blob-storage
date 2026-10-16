//! Descriptive source provenance; never authority or a whole-binary attestation.

/// Identify the selected complete CLI source inventory captured by its build script.
pub(super) fn runner_source() -> String {
    format!("sha256:{}", env!("BLOB_CLI_SOURCE_SHA256"))
}
