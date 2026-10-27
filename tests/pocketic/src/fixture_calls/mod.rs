//! Bounded Wire calls to private local fixtures, separate from Blob Wire endpoints.
use ic_platform::pocket_ic::PocketIc;

/// Finite decoding budgets for replies from Blob's installed local fixtures.
/// These cover bounded status/history pages and byte-bearing probe replies;
/// production provider and operator clients retain their own narrower policies.
pub const REPLY_BYTES: usize = blob_test_protocol::fixture_wire::MAX_BYTES;

/// Query Blob's private Wire fixtures under their fixed reply budgets.
/// Wire owns codecs; Testkit owns transport panic classification.
/// # Errors
/// Preserves encoding, transport, rejection and bounded decoding failures.
pub fn query_fixture<T, A>(
    pic: &PocketIc,
    service: ic_platform::principal::ExternalPrincipal,
    caller: ic_platform::principal::ExternalPrincipal,
    method: &str,
    arguments: A,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
    A: serde::Serialize,
{
    let arguments = blob_test_protocol::fixture_wire::encode_args(method, arguments)?;
    let reply = ic_testkit::pic::try_pocket_ic_operation(|| {
        pic.query_call(service, caller, method, arguments)
    })?
    .map_err(ic_testkit::pic::PocketIcCallFailure::Rejected)?;
    blob_test_protocol::fixture_wire::decode_args(method, &reply, REPLY_BYTES).map_err(Into::into)
}

/// Update Blob's private Wire fixtures once, using the same finite budgets.
/// A transport failure never authorizes retrying an uncertain update.
/// # Errors
/// Preserves encoding, transport, rejection and bounded decoding failures.
pub fn update_fixture<T, A>(
    pic: &PocketIc,
    service: ic_platform::principal::ExternalPrincipal,
    caller: ic_platform::principal::ExternalPrincipal,
    method: &str,
    arguments: A,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
    A: serde::Serialize,
{
    let arguments = blob_test_protocol::fixture_wire::encode_args(method, arguments)?;
    let reply = ic_testkit::pic::try_pocket_ic_operation(|| {
        pic.update_call(service, caller, method, arguments)
    })?
    .map_err(ic_testkit::pic::PocketIcCallFailure::Rejected)?;
    blob_test_protocol::fixture_wire::decode_args(method, &reply, REPLY_BYTES).map_err(Into::into)
}
