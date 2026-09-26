//! Bounded fixture journals and single platform effects for a local-only experiment.

mod storage;

use std::{cell::RefCell, num::NonZeroUsize};

use crate::model::FundingJournalRecord;
use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingOutcome, FundingReceiptRecord,
    FundingReplyMode, FundingRequest,
};
use candid::Principal;
use ic_blob_storage::ops::caffeine::funding::{
    TopUpProviderError, TopUpReply, TopUpReplyLimits, decode_top_up_reply,
};
use ic_cdk::call::{Call, CallFailed};

thread_local! {
    static STATE: RefCell<Option<FundingJournalRecord>> = const { RefCell::new(None) };
}

fn read<T>(f: impl FnOnce(&FundingJournalRecord) -> T) -> T {
    STATE.with_borrow(|state| f(state.as_ref().expect("initialized fixture")))
}

fn mutate<T>(f: impl FnOnce(&mut FundingJournalRecord) -> T) -> T {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized fixture");
        let result = f(state);
        storage::save(state);
        result
    })
}

pub(crate) fn initialize(peer: Principal, driver: Principal) {
    storage::open();
    STATE.with_borrow_mut(|state| {
        let initial = FundingJournalRecord::new(peer, driver);
        storage::save(&initial);
        *state = Some(initial);
    });
}

pub(crate) fn restore() {
    storage::open();
    let recovered = storage::load();
    recovered.validate();
    STATE.with_borrow_mut(|state| *state = Some(recovered));
}

pub(crate) fn admit(
    caller: Principal,
    request: FundingRequest,
) -> Result<Principal, FundingFailure> {
    mutate(|state| state.admit(caller, request))
}

pub(crate) fn complete(id: u64, observation: FundingObservation) {
    mutate(|state| state.complete(id, observation));
}

pub(crate) fn attempts(caller: Principal) -> Option<Vec<FundingAttemptRecord>> {
    read(|state| state.attempts(caller))
}

pub(crate) fn receipts(caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
    read(|state| state.receipts(caller))
}

pub(crate) async fn transfer(peer: Principal, request: FundingRequest) -> FundingObservation {
    let result = Call::unbounded_wait(peer, "receive")
        .with_arg(request)
        .with_cycles(request.offered)
        .await;
    // Capture in this call's continuation, before decoding, further awaits or spawning.
    // An enqueue failure runs without a response callback: never sample its ambient refund.
    let refunded = match &result {
        Ok(_) | Err(CallFailed::CallRejected(_)) => Some(ic_cdk::api::msg_cycles_refunded()),
        Err(_) => None,
    };
    let outcome = match result {
        Ok(response) => classify(&response.into_bytes()),
        Err(CallFailed::CallRejected(error)) => FundingOutcome::Rejected(error.raw_reject_code()),
        Err(_) => FundingOutcome::NotEnqueued,
    };
    FundingObservation {
        refunded,
        transport_accepted: refunded
            .map(|refund| request.offered.checked_sub(refund).expect("refund bound")),
        outcome,
    }
}

fn classify(bytes: &[u8]) -> FundingOutcome {
    let positive = |value| NonZeroUsize::new(value).expect("positive fixture budget");
    let limits = TopUpReplyLimits {
        max_bytes: positive(4096),
        decoding_quota: positive(100_000),
        skipping_quota: positive(1000),
        max_type_entries: positive(32),
    };
    match decode_top_up_reply(bytes, limits) {
        Ok(TopUpReply::ReportedSuccess { .. }) => FundingOutcome::ReportedSuccess,
        Ok(TopUpReply::ProviderFailure(TopUpProviderError::InternalError)) => {
            FundingOutcome::ProviderError
        }
        _ => FundingOutcome::InvalidReply,
    }
}

pub(crate) fn accept(caller: Principal, request: FundingRequest) {
    read(|state| state.check_receive(caller));
    let available = ic_cdk::api::msg_cycles_available();
    let accepted = ic_cdk::api::msg_cycles_accept(request.accept);
    mutate(|state| {
        state.record_acceptance(FundingReceiptRecord {
            id: request.id,
            available,
            accepted,
        });
    });
}

pub(crate) fn reply(mode: FundingReplyMode) {
    // Reuse source-backed wire bytes, rather than defining another Cashier schema.
    let hex = match mode {
        FundingReplyMode::Success => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/success.hex"
        ),
        FundingReplyMode::ProviderError => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/internal.hex"
        ),
        FundingReplyMode::Malformed => {
            ic_cdk::api::msg_reply(b"not candid");
            return;
        }
        FundingReplyMode::Reject => {
            ic_cdk::api::msg_reject("deliberate rejection after acceptance");
            return;
        }
        FundingReplyMode::Trap => ic_cdk::trap("deliberate receiver failure after acceptance"),
    };
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty(), "whole fixture bytes");
    let bytes: Vec<u8> = pairs
        .iter()
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("hex text"), 16).expect("hex byte")
        })
        .collect();
    ic_cdk::api::msg_reply(bytes);
}
