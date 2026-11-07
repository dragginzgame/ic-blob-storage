//! Fixture orchestration. No payment retry or production provider contract.

use blob_test_protocol::funding::FundingFailure;
use ic_platform::Principal;

use crate::ops;

pub(crate) fn configure_cashier(
    caller: Principal,
    input: blob_test_protocol::storage::funding::transport::Substitute,
) -> Result<(), FundingFailure> {
    ops::cashier::configure(caller, input)
}
pub(crate) async fn receive_top_up(caller: Principal, arguments: Vec<u8>) {
    let mode = ops::cashier::accept(caller, &arguments);
    ops::delay_reply(mode).await;
    ops::reply(mode);
}

pub(crate) fn restore() {
    let journal = ops::load();
    // No pre-upgrade snapshot is needed: every mutation is already durable.
    // Restore only receiver evidence, never an executable continuation.
    ops::restore_fenced(journal);
}
