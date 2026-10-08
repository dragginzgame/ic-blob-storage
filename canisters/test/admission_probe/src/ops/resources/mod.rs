//! Fixed diagnostic window, overwritten at capacity; no operational authority.

use super::STATE;
use blob_test_protocol::admission::{ExecutionProfile, Failure, RESOURCE_SAMPLE_CAPACITY};
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::cell::Cell;

thread_local! {
    // Decoder checkpoints for the currently executing message only.
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
        let profiles = &mut state.as_mut().expect("initialized probe").profiles;
        let sequence = profiles.back().map_or(1, |last| {
            last.sequence.checked_add(1).expect("diagnostic sequence")
        });
        if profiles.len() == RESOURCE_SAMPLE_CAPACITY {
            profiles.pop_front();
        }
        profiles.push_back(ExecutionProfile {
            sequence,
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

pub(crate) fn inspect(context: UploadContext, limit: u8) -> Result<Vec<ExecutionProfile>, Failure> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().expect("initialized probe");
        let bindings = state.config.bindings();
        if context.service != bindings.service {
            return Err(Failure::WrongService);
        }
        if context.actor != bindings.operator {
            return Err(Failure::NotOperator);
        }
        let limit = usize::from(limit);
        if limit == 0 || limit > RESOURCE_SAMPLE_CAPACITY {
            return Err(Failure::InvalidInput);
        }
        Ok(state
            .profiles
            .iter()
            .skip(state.profiles.len().saturating_sub(limit))
            .copied()
            .collect())
    })
}
