//! Bounded fixture journals and single platform effects for a local-only experiment.

pub(crate) mod liquidity;
pub(crate) mod lookup;
pub(crate) mod preview;
pub(crate) mod status;
mod storage;

use std::{cell::RefCell, num::NonZeroUsize};

use crate::model::FundingJournalRecord;
use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingOutcome,
    FundingProviderErrorView, FundingReceiptRecord, FundingReconciliationView, FundingReplyMode,
    FundingRequest,
};
use candid::Principal;
use ic_blob_storage::model::billing::transfer::FundingTransfer;
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

pub(crate) fn initialize(
    peer: Principal,
    driver: Principal,
    input: blob_test_protocol::funding::budget::FundingBudgetInput,
) {
    storage::open();
    STATE.with_borrow_mut(|state| {
        let budget = crate::model::budget::FundingBudgetRecord::new(
            input.allocated,
            input.reserve,
            input.operating_reserve,
            input.other_liabilities,
        )
        .expect("valid explicit attachment budget");
        let initial = FundingJournalRecord::new(ic_cdk::api::canister_self(), peer, driver, budget);
        storage::save(&initial);
        *state = Some(initial);
    });
}

pub(crate) fn load() -> FundingJournalRecord {
    storage::open();
    let recovered = storage::load();
    recovered
        .validate(ic_cdk::api::canister_self())
        .expect("bound consistent journal");
    recovered
}

pub(crate) fn restored_reconciliations(
    record: &FundingJournalRecord,
) -> Vec<(FundingTransfer, FundingReconciliationView)> {
    record
        .all_attempts()
        .iter()
        .filter_map(|entry| {
            entry.observation.map(|observation| {
                (
                    crate::model::checked_transfer(entry).expect("validated restored transfer"),
                    observation.reconciliation,
                )
            })
        })
        .collect()
}

pub(crate) fn restore_fenced(mut recovered: FundingJournalRecord) {
    recovered.fence();
    storage::save(&recovered);
    STATE.with_borrow_mut(|state| *state = Some(recovered));
}

pub(crate) fn admit(
    caller: Principal,
    request: FundingRequest,
) -> Result<Principal, FundingFailure> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized fixture");
        let result = state.admit(caller, request);
        if result.is_ok() {
            storage::save(state);
        }
        result
    })
}

pub(crate) fn complete(request: FundingRequest, observation: FundingObservation) {
    mutate(|state| state.complete(request, observation));
}

pub(crate) fn attempts(caller: Principal) -> Option<Vec<FundingAttemptRecord>> {
    read(|state| state.attempts(caller))
}

pub(crate) fn receipts(caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
    read(|state| state.receipts(caller))
}

/// Local domain facts before workflow policy and passive view conversion.
pub(crate) struct ObservedFundingCall {
    pub(crate) transfer: FundingTransfer,
    pub(crate) outcome: FundingOutcome,
}

pub(crate) async fn transfer(prepared: liquidity::PreparedFundingCall) -> ObservedFundingCall {
    let offered = prepared.offered;
    let result = prepared.call.await;
    // Capture in this call's continuation, before decoding, further awaits or spawning.
    // An enqueue failure runs without a response callback: never sample its ambient refund.
    let refunded = match &result {
        Ok(_) | Err(CallFailed::CallRejected(_)) => Some(ic_cdk::api::msg_cycles_refunded()),
        Err(CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_)) => {
            None
        }
    };
    let transfer = match refunded {
        Some(refund) => FundingTransfer::unbounded_callback(offered, refund)
            .expect("call-specific refund within attachment"),
        None => FundingTransfer::not_enqueued(offered),
    };
    let outcome = match result {
        Ok(response) => classify(&response.into_bytes()),
        Err(CallFailed::CallRejected(error)) => FundingOutcome::Rejected(error.raw_reject_code()),
        Err(CallFailed::InsufficientLiquidCycleBalance(_) | CallFailed::CallPerformFailed(_)) => {
            FundingOutcome::NotEnqueued
        }
    };
    ObservedFundingCall { transfer, outcome }
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
        Ok(TopUpReply::ProviderFailure(error)) => FundingOutcome::ProviderError(match error {
            TopUpProviderError::NotAuthorized(principal) => {
                FundingProviderErrorView::NotAuthorized(principal)
            }
            TopUpProviderError::AccountBalanceOverflow => {
                FundingProviderErrorView::AccountBalanceOverflow
            }
            TopUpProviderError::InternalError => FundingProviderErrorView::InternalError,
            TopUpProviderError::TopUpWithoutCycles => FundingProviderErrorView::TopUpWithoutCycles,
        }),
        Err(_) => FundingOutcome::InvalidReply,
    }
}

pub(crate) fn accept(caller: Principal, request: FundingRequest) {
    let available = ic_cdk::api::msg_cycles_available();
    read(|state| state.check_receive(caller, request, available));
    let accepted = ic_cdk::api::msg_cycles_accept(request.accept);
    mutate(|state| {
        state.record_acceptance(FundingReceiptRecord {
            id: request.id,
            available,
            accepted,
        });
    });
}

pub(crate) async fn delay_reply(mode: FundingReplyMode) {
    if mode == FundingReplyMode::DelayedSuccess {
        // Local scheduling only: commit the receiver's acceptance and let the
        // test capture real outstanding journals before returning the reply.
        for _ in 0..8 {
            read(|state| {
                assert!(
                    !state.fenced(),
                    "restored receiver cannot resume scheduling"
                );
            });
            Call::unbounded_wait(Principal::management_canister(), "raw_rand")
                .await
                .expect("bounded fixture scheduling round");
        }
    }
}

pub(crate) fn reply(mode: FundingReplyMode) {
    read(|state| assert!(!state.fenced(), "restored receiver cannot resume a reply"));
    // Reuse source-backed wire bytes, rather than defining another Cashier schema.
    let hex = match mode {
        FundingReplyMode::Success | FundingReplyMode::DelayedSuccess => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/success.hex"
        ),
        FundingReplyMode::InternalError => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/internal.hex"
        ),
        FundingReplyMode::NotAuthorized => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/unauthorized.hex"
        ),
        FundingReplyMode::AccountBalanceOverflow => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/overflow.hex"
        ),
        FundingReplyMode::TopUpWithoutCycles => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/without-cycles.hex"
        ),
        FundingReplyMode::LedgerReport => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-ledger/credit.hex"
        ),
        FundingReplyMode::UnknownError => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/unknown-error.hex"
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
