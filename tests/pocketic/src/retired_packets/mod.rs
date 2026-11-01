//! Frozen negative inputs, never a compatibility decoder or old service client.

/// A valid pre-Wire Candid operator-scope packet with fixed fixture principals.
/// The current Wire ingress must refuse this before decoding application fields.
#[must_use]
pub fn candid_scope() -> Vec<u8> {
    include_bytes!("../../fixtures/retired-operator-scope.candid").to_vec()
}
