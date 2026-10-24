//! Native composition of operator input, Candid values and funding policy.
//! No canister, provider response or payment effect is simulated here.

use ic_blob_storage::{
    model::{
        billing::balance::BalanceAmounts,
        gateway::{GatewayList, GatewayListError},
    },
    ops::billing::{
        BillingField, BillingInputError,
        balance::{
            BalanceAmountsInput, BalanceField, BalanceInputError, balance_amounts_from_candid,
        },
        cycles_from_int, cycles_from_nat,
    },
    policy::billing::{
        BalanceObservation, BillingBlocker, BillingObservation, FundingDecision, FundingStatus,
        RecoveryState,
        admission::{
            FundingActivity, FundingAdmissionBlocker, FundingAdmissionDecision,
            FundingAdmissionObservation, assess_funding_admission,
        },
        assess_funding, assess_readiness,
    },
};
use ic_blob_storage_contracts::configuration::{
    billing::BillingConfiguration, funding::FundingLimits,
};
use ic_platform::Principal;
use ic_platform::candid::contract::{Int, Nat, encode_one};
use ic_platform::candid::decoder;
use std::num::NonZeroU128;

#[test]
fn wire_valid_malformed_component_cannot_make_billing_ready() {
    let limits = FundingLimits::new(1, 10, 100).expect("valid limits");
    for (ledger, expected) in [
        (
            Int::from(-1),
            Err(BalanceInputError {
                field: BalanceField::Ledger,
            }),
        ),
        (Int::from(0), Ok(100_u128)),
    ] {
        // A tuple exercises actual Candid integers without defining a provider DTO.
        let wire = encode_one((Int::from(100), Int::from(1), Int::from(2), ledger))
            .expect("wire-valid signed amounts");
        let (total, prepaid, promotional, ledger): (Int, Int, Int, Int) =
            decoder::decode_one_with_config(&wire, &decoder::DecoderConfig::new())
                .expect("decode numeric fixture");
        let amounts = balance_amounts_from_candid(BalanceAmountsInput {
            total: &total,
            prepaid: &prepaid,
            promotional: &promotional,
            ledger: &ledger,
        });
        assert_eq!(amounts.map(BalanceAmounts::total), expected);
        let balance = match amounts {
            Ok(value) => BalanceObservation::Available(value.total()),
            Err(_) => BalanceObservation::Malformed,
        };
        let readiness = assess_readiness(
            Some(limits),
            BillingObservation {
                gateway_count: 1,
                balance,
                available_cycles: 1_000,
                recovery: RecoveryState::Reconciled,
            },
        );
        if expected.is_err() {
            assert_eq!(readiness.blockers(), &[BillingBlocker::BalanceMalformed]);
            assert_eq!(readiness.funding(), FundingStatus::BalanceMalformed);
            assert!(!readiness.is_ready());
        } else {
            assert!(readiness.is_ready());
            assert_eq!(readiness.funding(), FundingStatus::NotNeeded);
        }
    }
}

#[test]
fn typed_amounts_keep_full_request_reserve_rules() {
    let limits = FundingLimits::new(500, 10, 100).unwrap();
    for (amount, fits) in [(500, true), (501, false)] {
        let amount = NonZeroU128::new(amount).unwrap();
        let expected = if fits {
            FundingDecision::FitsReserve {
                requested_cycles: amount,
            }
        } else {
            FundingDecision::ReserveWouldBeViolated {
                requested_cycles: amount,
                transferable_cycles: 500,
            }
        };
        assert_eq!(assess_funding(limits, amount, 1_000), expected);
    }
}

#[test]
fn diagnostic_top_up_does_not_bypass_funding_admission() {
    let limits = FundingLimits::new(500, 10, 100).expect("validated limits");
    let amount = NonZeroU128::new(100).unwrap();

    for (recovery, activity, expected) in [
        (
            RecoveryState::Fenced,
            FundingActivity::Clear,
            FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::RecoveryFenced),
        ),
        (
            RecoveryState::Reconciled,
            FundingActivity::InProgress,
            FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::FundingInProgress),
        ),
        (
            RecoveryState::Reconciled,
            FundingActivity::Uncertain,
            FundingAdmissionDecision::Blocked(FundingAdmissionBlocker::FundingUncertain),
        ),
        (
            RecoveryState::Reconciled,
            FundingActivity::Clear,
            FundingAdmissionDecision::PrepareIntent {
                requested_cycles: amount,
            },
        ),
    ] {
        let readiness = assess_readiness(
            Some(limits),
            BillingObservation {
                gateway_count: 1,
                balance: BalanceObservation::Available(0),
                available_cycles: 1_000,
                recovery,
            },
        );
        // Diagnostics retain useful arithmetic, including during recovery.
        assert_eq!(
            readiness.funding(),
            FundingStatus::TopUp(FundingDecision::FitsReserve {
                requested_cycles: amount
            }),
        );
        assert_eq!(
            assess_funding_admission(
                Some(limits),
                amount,
                FundingAdmissionObservation {
                    available_cycles: 1_000,
                    recovery,
                    activity
                },
            ),
            expected,
        );
    }
}

#[test]
fn valid_candid_encoding_does_not_make_out_of_range_numbers_valid() {
    let oversized =
        Nat::parse(b"340282366920938463463374607431768211456").expect("arbitrary-precision input");
    let bytes = encode_one(oversized).expect("Candid encode");
    let decoded: Nat = decoder::decode_one_with_config(&bytes, &decoder::DecoderConfig::new())
        .expect("Candid decode");
    assert_eq!(
        cycles_from_nat(BillingField::ObservedBalance, &decoded),
        Err(BillingInputError::OutOfRange {
            field: BillingField::ObservedBalance,
        })
    );

    for (balance, expected) in [
        (
            Int::from(-1),
            Err(BillingInputError::OutOfRange {
                field: BillingField::ObservedBalance,
            }),
        ),
        (Int::from(u128::MAX), Ok(u128::MAX)),
    ] {
        let bytes = encode_one(balance).expect("Candid encode");
        let decoded: Int = decoder::decode_one_with_config(&bytes, &decoder::DecoderConfig::new())
            .expect("Candid decode");
        assert_eq!(cycles_from_int(&decoded), expected);
    }
}

#[test]
fn configuration_drives_gateway_bounds_and_funding_diagnosis() {
    let configuration = BillingConfiguration::new(
        Principal::from_slice(&[9, 1]),
        FundingLimits::new(100, 10, 50).unwrap(),
        3,
        2,
    )
    .expect("validated configuration");
    let first = Principal::from_slice(&[1, 1]);
    let second = Principal::from_slice(&[2, 1]);
    let mut gateways = GatewayList::new(&[first, second, first], configuration.gateway_limits())
        .expect("exact configured bounds");
    assert_eq!(
        gateways.replace(&[first; 4]),
        Err(GatewayListError::TooManyEntries {
            actual: 4,
            maximum: 3
        })
    );
    assert_eq!(gateways.principals(), &[first, second]);

    let observation = BillingObservation {
        gateway_count: gateways.principals().len(),
        balance: BalanceObservation::Available(9),
        available_cycles: 140,
        recovery: RecoveryState::Reconciled,
    };
    let blocked = assess_readiness(Some(configuration.funding_limits()), observation);
    assert_eq!(
        blocked.blockers(),
        &[
            BillingBlocker::InsufficientBalance,
            BillingBlocker::ReserveWouldBeViolated
        ]
    );
    assert_eq!(
        blocked.funding(),
        FundingStatus::TopUp(FundingDecision::ReserveWouldBeViolated {
            requested_cycles: NonZeroU128::new(41).unwrap(),
            transferable_cycles: 40,
        })
    );
    let ready = assess_readiness(
        Some(configuration.funding_limits()),
        BillingObservation {
            balance: BalanceObservation::Available(10),
            ..observation
        },
    );
    assert!(ready.is_ready());
    assert_eq!(ready.funding(), FundingStatus::NotNeeded);
}
