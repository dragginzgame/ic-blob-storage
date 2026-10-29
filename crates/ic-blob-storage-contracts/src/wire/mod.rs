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

// Policy is declared beside the mounted signature. Lifecycle frames are host-owned.
macro_rules! contract {
    (external; $($entry:tt)*) => {
        None
    };
    ($id:literal; $mode:ident, tenant_inspection, $($entry:tt)*) => {
        Some(crate::endpoints::WireContract {
            id: $id,
            request: tenant::LIMITS,
            response: tenant::LIMITS,
        })
    };
    ($id:literal; $mode:ident, tenant_update, $($entry:tt)*) => {
        Some(crate::endpoints::WireContract {
            id: $id,
            request: tenant::LIMITS,
            response: tenant::LIMITS,
        })
    };
    ($id:literal; $mode:ident, manifest, $($entry:tt)*) => {
        Some(crate::endpoints::WireContract {
            id: $id,
            request: limits(128 * 1024),
            response: limits(MAX_MESSAGE_BYTES),
        })
    };
    ($id:literal; $($entry:tt)*) => {
        Some(crate::endpoints::WireContract {
            id: $id,
            request: limits(4096),
            response: limits(MAX_MESSAGE_BYTES),
        })
    };
}
macro_rules! contracts {
    ($emit:ident; $([$constant:ident, $id:tt; $($entry:tt)*])*) => {
        pub(crate) const fn selected_contract(method: &str) -> Result<crate::endpoints::WireContract, Error> {
            $(const $constant: &[u8] = crate::protocol::$constant.as_bytes();)*
            const STANDALONE_METRICS: &[u8] = crate::standalone::METRICS_METHOD.name.as_bytes();
            match method.as_bytes() {
                $($constant => match contract!($id; $($entry)*) {
                    Some(value) => Ok(value), None => Err(Error::Identity),
                },)*
                // Lifecycle inputs are host-owned and are not callable service methods.
                b"blob_install" => Ok(crate::endpoints::WireContract {
                    id: 0x424c_4f42_0052_ff01, request: limits(16 * 1024), response: limits(MAX_MESSAGE_BYTES),
                }),
                b"blob_post_upgrade" => Ok(crate::endpoints::WireContract {
                    id: 0x424c_4f42_0042_ff02, request: limits(4096), response: limits(MAX_MESSAGE_BYTES),
                }),
                STANDALONE_METRICS => Ok(crate::standalone::METRICS_WIRE),
                _ => Err(Error::Identity),
            }
        }
    };
}
crate::__blob_service_catalog!(
    dispatch = [contracts],
    emit = unused,
    workflow = unused,
    context = unused,
    contracts = crate,
    clock = unused,
    caller = unused,
    trap = unused,
);
/// Consumer-assigned stable identity. Unknown and provider method names refuse.
/// # Errors
/// Returns an identity error for methods outside the owned Wire surface.
pub const fn identity(method: &str) -> Result<u64, Error> {
    match selected_contract(method) {
        Ok(value) => Ok(value.id),
        Err(error) => Err(error),
    }
}
/// Complete request ceiling, including the eleven-byte Wire header.
/// # Errors
/// Refuses names outside this contract.
pub const fn request_limit(method: &str) -> Result<usize, Error> {
    match selected_contract(method) {
        Ok(value) => Ok(value.request.bytes),
        Err(error) => Err(error),
    }
}
/// Complete admission budgets for the mounted request codec.
/// # Errors
/// Refuses unknown and external provider methods.
pub const fn request_limits(method: &str) -> Result<Limits, Error> {
    match selected_contract(method) {
        Ok(value) => Ok(value.request),
        Err(error) => Err(error),
    }
}
/// Complete admission budgets for the mounted response codec.
/// # Errors
/// Refuses unknown and external provider methods.
pub const fn response_limits(method: &str) -> Result<Limits, Error> {
    match selected_contract(method) {
        Ok(value) => Ok(value.response),
        Err(error) => Err(error),
    }
}
const fn limits(bytes: usize) -> Limits {
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
    native::encode_request(identity(method)?, &value, request_limits(method)?)
}
/// Admit and decode a complete request under the selected endpoint identity.
/// # Errors
/// Refuses mismatched identities/directions, malformed primitives or over-budget input.
pub fn decode_request<'de, T: Deserialize<'de>>(
    method: &str,
    bytes: &'de [u8],
    max: usize,
) -> Result<T, Error> {
    native::decode_request(
        identity(method)?,
        bytes,
        Limits {
            bytes: max.min(request_limit(method)?),
            ..request_limits(method)?
        },
    )
}
/// Encode one complete response; encoding failure must trap after local effects.
/// # Errors
/// Refuses unknown identities and values outside encoding/traversal allowances.
pub fn encode_response<T: Serialize>(method: &str, value: T) -> Result<Vec<u8>, Error> {
    native::encode_response(identity(method)?, &value, response_limits(method)?)
}
/// Decode an independently authenticated reply under the exact original method.
/// # Errors
/// Refuses mismatched identities/directions, malformed primitives or over-budget input.
pub fn decode_response<'de, T: Deserialize<'de>>(
    method: &str,
    bytes: &'de [u8],
    max: usize,
) -> Result<T, Error> {
    native::decode_response(
        identity(method)?,
        bytes,
        Limits {
            bytes: max.min(response_limits(method)?.bytes),
            ..response_limits(method)?
        },
    )
}

#[cfg(test)]
mod tests;
