//! The explicitly bounded measurement-command envelope, separate from service ingress.
use serde::de::DeserializeOwned;
const COMMAND_BYTES: usize = 16 * 1024;
pub(crate) fn mutation<T: DeserializeOwned, const METHOD: u64>(bytes: Vec<u8>) -> T {
    let before = super::resources::instructions();
    let command = decode::<T, METHOD>(&bytes, COMMAND_BYTES);
    drop(bytes);
    let after = super::resources::instructions();
    super::resources::decoded(before, after);
    command
}

fn decode<T: DeserializeOwned, const METHOD: u64>(bytes: &[u8], maximum: usize) -> T {
    let (value,) = blob_test_protocol::fixture_wire::request_id(METHOD, bytes, maximum)
        .expect("invalid or over-budget probe input");
    value
}
