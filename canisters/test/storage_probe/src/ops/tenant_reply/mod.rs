//! Local reply scheduling after a real tenant mutation, with no retry or provider call.
use super::{STATE, UploadContext};
use blob_test_protocol::storage::Failure;
use ic_platform::cdk::call::Call;
use std::cell::Cell;

thread_local! {
    static HOLD: Cell<bool> = const { Cell::new(false) };
    static REQUESTS: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn control(context: UploadContext, hold: Option<bool>) -> Result<u64, Failure> {
    STATE.with_borrow(|state| {
        if state.as_ref().expect("installed probe").operator != context.actor {
            return Err(Failure::Denied);
        }
        if let Some(hold) = hold {
            HOLD.set(hold);
        }
        Ok(REQUESTS.get())
    })
}

pub(crate) fn received() {
    REQUESTS.set(
        REQUESTS
            .get()
            .checked_add(1)
            .expect("bounded fixture calls"),
    );
}

pub(crate) async fn wait() {
    for _ in 0..128 {
        if !HOLD.get() {
            return;
        }
        // Yield through the real management API so the driver can observe the
        // committed mutation and expire the client's separate bounded call.
        Call::unbounded_wait(ic_platform::Principal::management_canister(), "raw_rand")
            .take_raw_args(
                ic_platform::candid::contract::encode_args(())
                    .expect("management scheduling arguments"),
            )
            .await
            .expect("fixture scheduling round");
    }
    ic_platform::cdk::api::trap("tenant reply hold exhausted");
}
