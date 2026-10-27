//! Bounded Candid calls only to emulated external provider contracts.
use ic_platform::pocket_ic::PocketIc;

/// Finite decoding budgets for replies from Blob's installed local fixtures.
/// These cover bounded status/history pages and byte-bearing probe replies;
/// production provider and operator clients retain their own narrower policies.
pub const REPLY_LIMITS: ic_platform::candid::DecodeLimits = ic_platform::candid::DecodeLimits {
    reply_bytes: 4 * 1024 * 1024,
    decoding_quota: 20_000_000,
    skipping_quota: 1_000_000,
    type_entries: 512,
    header_bytes: 64 * 1024,
};

/// Query Blob's emulated provider contracts under their fixed reply budgets.
/// Platform owns codecs; Testkit owns transport panic classification.
/// # Errors
/// Preserves encoding, transport, rejection and bounded decoding failures.
pub fn query_provider<T, A>(
    pic: &PocketIc,
    service: ic_platform::principal::ExternalPrincipal,
    caller: ic_platform::principal::ExternalPrincipal,
    method: &str,
    arguments: A,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> ic_platform::candid::contract::ArgumentDecoder<'de>,
    A: ic_platform::candid::contract::ArgumentEncoder,
{
    let arguments = ic_platform::candid::contract::encode_args(arguments)?;
    let reply = ic_testkit::pic::try_pocket_ic_operation(|| {
        pic.query_call(service, caller, method, arguments)
    })?
    .map_err(ic_testkit::pic::PocketIcCallFailure::Rejected)?;
    ic_platform::candid::decode_args(&reply, REPLY_LIMITS).map_err(Into::into)
}

/// Update an emulated external provider contract once, using the same finite budgets.
/// A transport failure never authorizes retrying an uncertain update.
/// # Errors
/// Preserves encoding, transport, rejection and bounded decoding failures.
pub fn update_provider<T, A>(
    pic: &PocketIc,
    service: ic_platform::principal::ExternalPrincipal,
    caller: ic_platform::principal::ExternalPrincipal,
    method: &str,
    arguments: A,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> ic_platform::candid::contract::ArgumentDecoder<'de>,
    A: ic_platform::candid::contract::ArgumentEncoder,
{
    let arguments = ic_platform::candid::contract::encode_args(arguments)?;
    let reply = ic_testkit::pic::try_pocket_ic_operation(|| {
        pic.update_call(service, caller, method, arguments)
    })?
    .map_err(ic_testkit::pic::PocketIcCallFailure::Rejected)?;
    ic_platform::candid::decode_args(&reply, REPLY_LIMITS).map_err(Into::into)
}
