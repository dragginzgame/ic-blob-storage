//! Native composition of operator input, Candid values and funding policy.
//! No canister, provider response or payment effect is simulated here.

use candid::{Int, Nat, Principal, decode_one, encode_one};
use ic_blob_storage::{
    model::{
        billing::balance::BalanceAmounts,
        gateway::{GatewayList, GatewayListError},
    },
    ops::billing::{
        BillingField, BillingInputError, FundingLimitsInput,
        balance::{
            BalanceAmountsInput, BalanceField, BalanceInputError, balance_amounts_from_candid,
        },
        configuration::{BillingConfigurationInput, configuration_from_candid},
        cycles_from_int, funding_from_nat, funding_limits_from_candid, parse_funding_cycles,
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

#[test]
fn wire_valid_malformed_component_cannot_make_billing_ready() {
    let limits = funding_limits_from_candid(FundingLimitsInput {
        reserve: &Nat::from(1_u8),
        minimum_balance: &Nat::from(10_u8),
        target_balance: &Nat::from(100_u8),
    })
    .expect("valid limits");
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
            decode_one(&wire).expect("decode numeric fixture");
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
fn operator_amounts_survive_candid_and_keep_full_request_reserve_rules() {
    let limits = funding_limits_from_candid(FundingLimitsInput {
        reserve: &Nat::from(500_u16),
        minimum_balance: &Nat::from(10_u8),
        target_balance: &Nat::from(100_u8),
    })
    .expect("valid numeric configuration");

    for (text, fits) in [("000500", true), ("501", false)] {
        let operator_amount = parse_funding_cycles(text).expect("valid operator amount");
        let bytes = encode_one(Nat::from(operator_amount.get())).expect("Candid encode");
        let decoded: Nat = decode_one(&bytes).expect("Candid decode");
        let amount = funding_from_nat(&decoded).expect("bounded positive input");
        assert_eq!(amount, operator_amount);
        let decision = assess_funding(limits, amount, 1_000);
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
        assert_eq!(decision, expected);
    }
}

#[test]
fn diagnostic_top_up_does_not_bypass_funding_admission() {
    let limits = funding_limits_from_candid(FundingLimitsInput {
        reserve: &Nat::from(500_u16),
        minimum_balance: &Nat::from(10_u8),
        target_balance: &Nat::from(100_u8),
    })
    .expect("validated limits");
    let amount = funding_from_nat(&Nat::from(100_u8)).expect("validated request");

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
    let decoded: Nat = decode_one(&bytes).expect("Candid decode");
    assert_eq!(
        funding_from_nat(&decoded),
        Err(BillingInputError::OutOfRange {
            field: BillingField::RequestedFunding,
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
        let decoded: Int = decode_one(&bytes).expect("Candid decode");
        assert_eq!(cycles_from_int(&decoded), expected);
    }
}

#[test]
fn configuration_drives_gateway_bounds_and_funding_diagnosis() {
    let configuration = configuration_from_candid(BillingConfigurationInput {
        cashier: Principal::from_slice(&[9, 1]),
        funding: FundingLimitsInput {
            reserve: &Nat::from(100_u8),
            minimum_balance: &Nat::from(10_u8),
            target_balance: &Nat::from(50_u8),
        },
        max_gateway_entries: 3,
        max_gateway_unique: 2,
    })
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
            requested_cycles: parse_funding_cycles("41").expect("positive amount"),
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
