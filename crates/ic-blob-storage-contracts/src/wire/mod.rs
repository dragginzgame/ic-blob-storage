//! One method- and direction-bound Wire frame per owned service call.
//! Native Serde field/variant order is the frozen Blob schema. Provider codecs,
//! passive offline documents and persistent records have independent owners.
pub(crate) mod decimal;
/// Canonical host-side schemas and client generation.
#[cfg(all(feature = "wire-schema", not(target_arch = "wasm32")))]
pub mod schema;
/// Bounded tenant projections use the same native codec as other methods.
pub mod tenant;
pub use ic_wire::{BytesRef, Error, TextRef};
use ic_wire::{Limits, native};
use serde::{Deserialize, Serialize};

/// Absolute frame ceiling; receiving owners may impose a narrower bound.
pub const MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;
/// Native traversal ceiling, independent of the initial platform argument buffer.
pub const MAX_DEPTH: usize = 32;

/// Consumer-assigned stable identity. Unknown and provider method names refuse.
/// # Errors
/// Returns an identity error for methods outside the owned Wire surface.
pub fn identity(method: &str) -> Result<u64, Error> {
    Ok(match method {
        "blob_tenant" => 0x424c_4f42_0043_001a,
        "blob_update_tenant" => 0x424c_4f42_0043_001b,
        "blob_resume_current_instance" => 0x424c_4f42_0042_0001,
        "blob_configuration" => 0x424c_4f42_0042_0002,
        "blob_local_status" => 0x424c_4f42_0042_0003,
        "blob_admit_upload" => 0x424c_4f42_0042_0004,
        "blob_upload_admission" => 0x424c_4f42_0042_0005,
        "blob_upload_status" => 0x424c_4f42_0042_0006,
        "blob_revoke_upload" => 0x424c_4f42_0042_0007,
        "blob_prepare_upload" => 0x424c_4f42_0042_0008,
        "blob_upload_certificate_assessment" => 0x424c_4f42_0042_0009,
        "blob_upload_manifest" => 0x424c_4f42_0042_000a,
        "blob_apply_reference" => 0x424c_4f42_0042_000b,
        "blob_reference_receipt" => 0x424c_4f42_0042_000c,
        "blob_upload_history" => 0x424c_4f42_0042_000d,
        "blob_funding_history" => 0x424c_4f42_0042_000e,
        "blob_funding_outcome" => 0x424c_4f42_0042_000f,
        "blob_funding_preparation_assessment" => 0x424c_4f42_0042_0010,
        "blob_revoke_gateway" => 0x424c_4f42_0042_0011,
        "blob_sync_gateways" => 0x424c_4f42_0042_0012,
        "blob_cancel_gateway_sync" => 0x424c_4f42_0042_0013,
        "blob_upload_capacity" => 0x424c_4f42_0042_0014,
        "blob_reference_capacity" => 0x424c_4f42_0042_0015,
        "blob_lookup_content" => 0x424c_4f42_0042_0016,
        "blob_download_descriptor" => 0x424c_4f42_0042_0017,
        "blob_reference_status" => 0x424c_4f42_0042_0018,
        "blob_inspect_account" => 0x424c_4f42_0042_0019,
        "blob_install" => 0x424c_4f42_0042_ff01,
        "blob_post_upgrade" => 0x424c_4f42_0042_ff02,
        _ => return Err(Error::Identity),
    })
}
/// Complete request ceiling, including the eleven-byte Wire header.
/// # Errors
/// Refuses names outside this contract.
pub fn request_limit(method: &str) -> Result<usize, Error> {
    identity(method)?;
    Ok(match method {
        "blob_tenant" | "blob_update_tenant" => 512,
        "blob_prepare_upload" => 128 * 1024,
        "blob_install" => 16 * 1024,
        _ => 4096,
    })
}
/// Complete admission budgets for the mounted request codec.
/// # Errors
/// Refuses unknown and external provider methods.
pub fn request_limits(method: &str) -> Result<Limits, Error> {
    let bytes = request_limit(method)?;
    Ok(if matches!(method, "blob_tenant" | "blob_update_tenant") {
        tenant::LIMITS
    } else {
        limits(bytes)
    })
}
/// Complete admission budgets for the mounted response codec.
/// # Errors
/// Refuses unknown and external provider methods.
pub fn response_limits(method: &str) -> Result<Limits, Error> {
    identity(method)?;
    Ok(if matches!(method, "blob_tenant" | "blob_update_tenant") {
        tenant::LIMITS
    } else {
        limits(MAX_MESSAGE_BYTES)
    })
}
fn limits(bytes: usize) -> Limits {
    Limits {
        bytes,
        allocation: 64 * 1024 * 1024,
        nodes: 262_144,
        depth: MAX_DEPTH,
    }
}
/// Encode one owned native request. Tenant callers use the bounded tenant module.
/// # Errors
/// Refuses unknown identities and values outside encoding/traversal allowances.
pub fn encode_request<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    native::encode_request(identity(method)?, &value, limits(request_limit(method)?))
}
/// Admit and decode a complete request under the selected endpoint identity.
/// # Errors
/// Refuses mismatched identities/directions, noncanonical or over-budget input.
pub fn decode_request<'de, T: Serialize + Deserialize<'de>>(
    method: &str,
    bytes: &'de [u8],
    max: usize,
) -> Result<T, Error> {
    native::decode_request(
        identity(method)?,
        bytes,
        limits(max.min(request_limit(method)?)),
    )
}
/// Encode one complete response; encoding failure must trap after local effects.
/// # Errors
/// Refuses unknown identities and values outside encoding/traversal allowances.
pub fn encode_response<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    native::encode_response(identity(method)?, &value, limits(MAX_MESSAGE_BYTES))
}
/// Decode an independently authenticated reply under the exact original method.
/// # Errors
/// Refuses mismatched identities/directions, noncanonical or over-budget input.
pub fn decode_response<'de, T: Serialize + Deserialize<'de>>(
    method: &str,
    bytes: &'de [u8],
    max: usize,
) -> Result<T, Error> {
    native::decode_response(identity(method)?, bytes, limits(max.min(MAX_MESSAGE_BYTES)))
}

#[cfg(test)]
mod tests;
