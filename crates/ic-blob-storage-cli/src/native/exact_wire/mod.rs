//! Exact bounded passive CBOR documents for offline native inputs.
use super::Failure;
pub(super) const INSTALLATION_BYTES: usize = 16 * 1024;
/// Callers retain their command-specific byte ceilings; extra values always refuse.
pub(super) fn decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
    max: usize,
) -> Result<T, Failure> {
    ic_blob_storage_contracts::document::decode(bytes, max).map_err(|_| Failure::Arguments)
}
