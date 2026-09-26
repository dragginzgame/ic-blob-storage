//! Native reply/policy composition; no live account query or payment is performed.

use std::num::NonZeroUsize;

use candid::Principal;
use ic_blob_storage::{
    model::billing::FundingLimits,
    ops::caffeine::balance::{BalanceReply, BalanceReplyLimits, decode_balance_reply},
    policy::billing::{
        BalanceObservation, BillingBlocker, BillingObservation, FundingDecision, FundingStatus,
        RecoveryState, assess_readiness,
    },
};

fn fixture(hex: &str) -> Vec<u8> {
    let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty(), "complete fixture hex bytes");
    pairs
        .iter()
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16)
                .expect("fixture hex byte")
        })
        .collect()
}

fn reply_limits() -> BalanceReplyLimits {
    BalanceReplyLimits {
        max_bytes: NonZeroUsize::new(4096).expect("byte bound"),
        decoding_quota: NonZeroUsize::new(100_000).expect("work bound"),
        skipping_quota: NonZeroUsize::new(1000).expect("skip bound"),
        max_type_entries: NonZeroUsize::new(32).expect("type bound"),
    }
}

#[test]
fn failed_observations_never_produce_a_zero_balance_funding_suggestion() {
    let limits = FundingLimits::new(500, 10, 100).expect("funding limits");
    let account = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").expect("fixture account");
    let reply_limits = reply_limits();
    let success = fixture(include_str!("fixtures/caffeine-balance/success.hex"));
    for (bytes, expected_account, expected_blocker) in [
        (
            fixture(include_str!("fixtures/caffeine-balance/not-found.hex")),
            account,
            BillingBlocker::BalanceUnavailable,
        ),
        (
            fixture(include_str!("fixtures/caffeine-balance/internal.hex")),
            account,
            BillingBlocker::BalanceUnavailable,
        ),
        (
            fixture(include_str!(
                "fixtures/caffeine-balance/negative-ledger.hex"
            )),
            account,
            BillingBlocker::BalanceMalformed,
        ),
        (Vec::new(), account, BillingBlocker::BalanceMalformed),
        (
            success.clone(),
            Principal::from_slice(&[9, 1]),
            BillingBlocker::BalanceMalformed,
        ),
    ] {
        // The eventual workflow must also establish response source/freshness.
        // This local composition preserves errors instead of defaulting to zero.
        let balance = match decode_balance_reply(&bytes, expected_account, reply_limits) {
            Ok(BalanceReply::ReportedBalance { balance, .. }) => {
                BalanceObservation::Available(balance.total())
            }
            Ok(BalanceReply::ProviderFailure(_)) => BalanceObservation::Unavailable,
            Err(_) => BalanceObservation::Malformed,
        };
        let observation = BillingObservation {
            gateway_count: 1,
            balance,
            available_cycles: 1000,
            recovery: RecoveryState::Reconciled,
        };
        let failed = assess_readiness(Some(limits), observation);
        assert_eq!(failed.blockers(), &[expected_blocker]);
        let expected_funding = match balance {
            BalanceObservation::Unavailable => FundingStatus::BalanceUnavailable,
            BalanceObservation::Malformed => FundingStatus::BalanceMalformed,
            BalanceObservation::Available(_) => panic!("failed fixture supplied a balance"),
        };
        assert_eq!(failed.funding(), expected_funding);

        // A later valid observation can recover diagnosis without any mutation.
        let BalanceReply::ReportedBalance { balance, .. } =
            decode_balance_reply(&success, account, reply_limits).expect("valid reply")
        else {
            panic!("success fixture");
        };
        let recovered = assess_readiness(
            Some(limits),
            BillingObservation {
                balance: BalanceObservation::Available(balance.total()),
                ..observation
            },
        );
        assert!(recovered.is_ready());
        assert_eq!(recovered.funding(), FundingStatus::NotNeeded);
        let fenced = assess_readiness(
            Some(limits),
            BillingObservation {
                balance: BalanceObservation::Available(balance.total()),
                recovery: RecoveryState::Fenced,
                ..observation
            },
        );
        assert_eq!(fenced.blockers(), &[BillingBlocker::RecoveryFenced]);
    }

    let zero = fixture(include_str!("fixtures/caffeine-balance/zero.hex"));
    let BalanceReply::ReportedBalance { balance, .. } =
        decode_balance_reply(&zero, account, reply_limits).expect("valid zero reply")
    else {
        panic!("zero fixture");
    };
    let diagnosis = assess_readiness(
        Some(limits),
        BillingObservation {
            gateway_count: 1,
            balance: BalanceObservation::Available(balance.total()),
            available_cycles: 1000,
            recovery: RecoveryState::Reconciled,
        },
    );
    assert_eq!(diagnosis.blockers(), &[BillingBlocker::InsufficientBalance]);
    assert_eq!(
        diagnosis.funding(),
        FundingStatus::TopUp(FundingDecision::FitsReserve {
            requested_cycles: std::num::NonZeroU128::new(100).expect("positive target")
        })
    );
}
