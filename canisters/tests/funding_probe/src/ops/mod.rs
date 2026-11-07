//! Receiver accounting and platform effects; production owns all outgoing funding.

pub(crate) mod cashier;
mod storage;

use std::cell::RefCell;

use crate::model::FundingJournalRecord;
use blob_test_protocol::funding::{FundingReceiptRecord, FundingReplyMode, FundingRequest};
use ic_platform::Principal;
use ic_platform::cdk::call::Call;

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
    storage::create();
    STATE.with_borrow_mut(|state| {
        let initial =
            FundingJournalRecord::new(ic_platform::cdk::api::canister_self(), peer, driver);
        storage::save(&initial);
        *state = Some(initial);
    });
}

pub(crate) fn load() -> FundingJournalRecord {
    storage::open();
    let recovered = storage::load();
    recovered
        .validate(ic_platform::cdk::api::canister_self())
        .expect("bound consistent journal");
    recovered
}

pub(crate) fn restore_fenced(mut recovered: FundingJournalRecord) {
    recovered.fence();
    storage::save(&recovered);
    STATE.with_borrow_mut(|state| *state = Some(recovered));
}

pub(crate) fn receipts(caller: Principal) -> Option<Vec<FundingReceiptRecord>> {
    read(|state| state.receipts(caller))
}

pub(crate) fn accept(caller: Principal, request: FundingRequest) {
    let available = ic_platform::cdk::api::msg_cycles_available();
    read(|state| state.check_receive(caller, request, available));
    let accepted = ic_platform::cdk::api::msg_cycles_accept(request.accept);
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
                .take_raw_args(
                    ic_platform::candid::contract::encode_args(())
                        .expect("fixture scheduling arguments"),
                )
                .await
                .expect("bounded fixture scheduling round");
        }
    }
}

pub(crate) fn reply(mode: FundingReplyMode) {
    read(|state| assert!(!state.fenced(), "restored receiver cannot resume a reply"));
    // Reuse source-backed wire bytes, rather than defining another Cashier schema.
    let hex = match mode {
        FundingReplyMode::Success | FundingReplyMode::DelayedSuccess => {
            include_str!(
                "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/success.hex"
            )
        }
        FundingReplyMode::InternalError => {
            include_str!(
                "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/internal.hex"
            )
        }
        FundingReplyMode::NotAuthorized => {
            include_str!(
                "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/unauthorized.hex"
            )
        }
        FundingReplyMode::AccountBalanceOverflow => {
            include_str!(
                "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/overflow.hex"
            )
        }
        FundingReplyMode::TopUpWithoutCycles => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/without-cycles.hex"
        ),
        FundingReplyMode::LedgerReport => {
            include_str!(
                "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-ledger/credit.hex"
            )
        }
        FundingReplyMode::UnknownError => include_str!(
            "../../../../../crates/ic-blob-storage/tests/fixtures/caffeine-top-up/unknown-error.hex"
        ),
        FundingReplyMode::Malformed => {
            ic_platform::cdk::api::msg_reply(b"not candid");
            return;
        }
        FundingReplyMode::Reject => {
            ic_platform::cdk::api::msg_reject("deliberate rejection after acceptance");
            return;
        }
        FundingReplyMode::Trap => {
            ic_platform::cdk::api::trap("deliberate receiver failure after acceptance")
        }
    };
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty(), "whole fixture bytes");
    let bytes: Vec<u8> = pairs
        .iter()
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("hex text"), 16).expect("hex byte")
        })
        .collect();
    ic_platform::cdk::api::msg_reply(bytes);
}
