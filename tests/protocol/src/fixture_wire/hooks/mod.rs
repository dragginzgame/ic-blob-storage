//! Thin entrypoint hooks; native tests exercise the pure codec directly.
use serde::{Serialize, de::DeserializeOwned};

/// Decode a positional RPC request.
/// # Panics
/// Traps on invalid or over-budget frames before dispatch.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "entrypoint owns ingress bytes"
)]
pub fn args<T: DeserializeOwned, const METHOD: u64>(bytes: Vec<u8>) -> T {
    super::request_id(METHOD, &bytes, super::MAX_BYTES).expect("private Wire arguments")
}
/// Decode one RPC argument.
/// # Panics
/// Traps on invalid ingress before dispatch.
#[must_use]
pub fn one<T: DeserializeOwned, const METHOD: u64>(bytes: Vec<u8>) -> T {
    let (value,) = args::<(T,), METHOD>(bytes);
    value
}
/// Require an explicit empty Wire request; old empty/Candid inputs refuse.
/// # Panics
/// Traps on non-unit or invalid requests.
pub fn unit<const METHOD: u64>(bytes: Vec<u8>) {
    let () = args::<(), METHOD>(bytes);
}
/// Decode installation arguments under the lifecycle identity.
/// # Panics
/// Traps before initialization on invalid input.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "lifecycle owns ingress bytes"
)]
pub fn init<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    super::request("fixture_init", &bytes, super::MAX_BYTES).expect("private Wire installation")
}
/// Decode one installation argument.
/// # Panics
/// Traps before initialization on invalid input.
#[must_use]
pub fn init_one<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    let (value,) = init(bytes);
    value
}
/// Decode restoration arguments under the separate lifecycle identity.
/// # Panics
/// Traps before restoration on invalid input.
#[must_use]
#[expect(
    clippy::needless_pass_by_value,
    reason = "lifecycle owns ingress bytes"
)]
pub fn upgrade<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    super::request("fixture_post_upgrade", &bytes, super::MAX_BYTES)
        .expect("private Wire restoration")
}
/// Decode one restoration argument.
/// # Panics
/// Traps before restoration on invalid input.
#[must_use]
pub fn upgrade_one<T: DeserializeOwned>(bytes: Vec<u8>) -> T {
    let (value,) = upgrade(bytes);
    value
}
/// Require the empty restoration argument sequence.
/// # Panics
/// Traps before restoration on invalid input.
pub fn upgrade_unit(bytes: Vec<u8>) {
    let () = upgrade::<()>(bytes);
}
/// Encode a single result in its positional tuple.
/// # Panics
/// Traps rather than returning partial output.
#[must_use]
pub fn reply<T: Serialize, const METHOD: u64>(value: T) -> Vec<u8> {
    super::response_id(METHOD, (value,)).expect("private Wire reply")
}
