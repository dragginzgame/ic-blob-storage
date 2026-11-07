//! One labelled measurement command; public service ingress is tested separately.
use blob_test_protocol::admission::Command;
pub(super) fn encode(command: Command) -> (&'static str, Vec<u8>) {
    let method = "fixture_admission_measurement";
    (
        method,
        blob_test_protocol::fixture_wire::encode_one(method, command).unwrap(),
    )
}
