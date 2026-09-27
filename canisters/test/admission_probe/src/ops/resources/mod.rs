//! Bounded local diagnostics. One overwritten sample, no log or operational authority.

use super::STATE;
use blob_test_protocol::admission::{ExecutionProfile, Failure};
use ic_blob_storage::model::service::upload::UploadContext;
use std::cell::Cell;

thread_local! {
    // One overwritten observation, never a growing diagnostic journal.
    static DECODE: Cell<Option<(u64, u64, u64, u64)>> = const { Cell::new(None) };
}

pub(crate) fn decoded(before: u64, header: u64, value: u64, after: u64) {
    DECODE.set(Some((before, header, value, after)));
}

pub(crate) fn instructions() -> u64 {
    ic_cdk::api::instruction_counter()
}

pub(crate) fn record(context: UploadContext, before_work: u64) {
    let after_work = instructions();
    let (before_decode, after_header, after_value, after_decode) =
        DECODE.take().expect("command decoder observation");
    STATE.with_borrow_mut(|state| {
        state.as_mut().expect("initialized probe").last_profile = Some(ExecutionProfile {
            caller: context.actor,
            before_decode,
            after_header,
            after_value,
            after_decode,
            before_work,
            after_work,
        });
    });
}

pub(crate) fn inspect(context: UploadContext) -> Result<Option<ExecutionProfile>, Failure> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().expect("initialized probe");
        let bindings = state.config.bindings();
        if context.service != bindings.service {
            return Err(Failure::WrongService);
        }
        if context.actor != bindings.operator {
            return Err(Failure::NotOperator);
        }
        Ok(state.last_profile)
    })
}
