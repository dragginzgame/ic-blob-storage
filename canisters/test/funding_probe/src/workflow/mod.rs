//! Fixture orchestration. No payment retry or production provider contract.

use blob_test_protocol::funding::{FundingFailure, FundingObservation, FundingRequest};
use candid::Principal;

use crate::ops;

pub(crate) async fn fund(
    caller: Principal,
    request: FundingRequest,
) -> Result<FundingObservation, FundingFailure> {
    let peer = ops::admit(caller, request)?;
    let observation = ops::transfer(peer, request).await;
    ops::complete(request.id, observation);
    if request.trap_callback {
        // The actual IC rolls back complete() while the receiver's acceptance survives.
        ic_cdk::trap("deliberate fixture callback failure");
    }
    Ok(observation)
}

pub(crate) fn receive(caller: Principal, request: FundingRequest) {
    ops::accept(caller, request);
    ops::reply(request.reply);
}
