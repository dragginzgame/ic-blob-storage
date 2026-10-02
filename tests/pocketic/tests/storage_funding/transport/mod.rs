//! Actual IC transport/refunds through the shared journal and a labelled substitute.
use super::*;
mod dispatch;
use blob_test_protocol::{
    funding::{
        FundingFailure, FundingOutcome, FundingProviderErrorView, FundingReceiptRecord,
        FundingReplyMode, FundingRequest,
    },
    storage::funding::transport::{CallbackIdentityFault, Input, Observation, Substitute},
};
use ic_blob_storage::dto::{
    funding::{
        FundingPhase,
        outcome::{
            FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
            FundingReconciliation as FundingReconciliationView, FundingReportedBalance,
            FundingResponse,
        },
    },
    operator::OperatorScope,
};
use ic_blob_storage::ops::caffeine::funding::request::CashierTopUpRequest;
use std::num::NonZeroU128;

fn outcome_request(intent: Intent) -> FundingOutcomeRequest {
    FundingOutcomeRequest {
        scope: OperatorScope {
            service: intent.service,
            cashier: intent.cashier,
            payment_account: intent.account,
            namespace: intent.namespace,
        },
        operation: intent.operation,
        offered: intent.offered,
        target_balance: intent.target_balance,
    }
}
fn expected_response(outcome: FundingOutcome) -> FundingResponse {
    match outcome {
        FundingOutcome::ReportedSuccess => {
            FundingResponse::ReportedSuccess(FundingReportedBalance {
                total: 100,
                prepaid: 80,
                promotional: 10,
                ledger: 10,
            })
        }
        FundingOutcome::ProviderError(error) => match error {
            FundingProviderErrorView::NotAuthorized(p) => FundingResponse::NotAuthorized(p),
            FundingProviderErrorView::AccountBalanceOverflow => {
                FundingResponse::AccountBalanceOverflow
            }
            FundingProviderErrorView::InternalError => FundingResponse::InternalError,
            FundingProviderErrorView::TopUpWithoutCycles => FundingResponse::TopUpWithoutCycles,
        },
        FundingOutcome::InvalidReply => FundingResponse::InvalidReply,
        FundingOutcome::Rejected(code) => FundingResponse::Rejected(code),
        FundingOutcome::NotEnqueued => FundingResponse::NotEnqueued,
        FundingOutcome::LiquidityBlocked => FundingResponse::NotDispatched,
    }
}

impl Fixture {
    fn upgrade_storage(&self) {
        self.harness
            .pic
            .upgrade_canister(
                self.service,
                Self::wasm(),
                candid::encode_one(self.operator).unwrap(),
                Some(self.controller),
            )
            .unwrap();
    }
    fn with_cashier() -> Self {
        let harness = Harness::new();
        let cashier = harness.pic.create_canister();
        // The test controller acts for this configured canister operator through
        // PocketIC ingress. Both operator and Cashier bindings remain explicit.
        let f = Self::with_operator(harness, cashier);
        f.harness.pic.install_canister(
            cashier,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            candid::encode_args((
                f.service,
                f.controller,
                blob_test_protocol::funding::budget::FundingBudgetInput {
                    allocated: 1000,
                    reserve: 100,
                    operating_reserve: 1_000_000_000,
                    other_liabilities: 0,
                },
            ))
            .unwrap(),
            None,
        );
        f
    }
    fn configure_cashier(&self, intent: Intent, accepted: u128, reply: FundingReplyMode) {
        let request = CashierTopUpRequest::new(
            intent.cashier,
            intent.account,
            NonZeroU128::new(intent.offered).unwrap(),
            intent
                .target_balance
                .map(|value| NonZeroU128::new(value).unwrap()),
        )
        .unwrap();
        let result: Result<(), FundingFailure> = self
            .harness
            .pic
            .update_candid_as(
                self.operator,
                self.controller,
                "configure_cashier",
                (Substitute {
                    expected_arguments: request.arguments().to_vec(),
                    behavior: FundingRequest {
                        id: u64::try_from(intent.operation).unwrap(),
                        offered: intent.offered,
                        accept: accepted,
                        reply,
                        trap_callback: false,
                    },
                },),
            )
            .unwrap();
        result.unwrap();
    }
    fn transport(&self, actor: Principal, input: Input) -> Result<Observation, Failure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "fixture_funding_transport", (input,))
            .unwrap()
    }
    fn incoming(&self) -> Vec<FundingReceiptRecord> {
        self.harness
            .pic
            .query_candid_as::<Option<Vec<FundingReceiptRecord>>, _>(
                self.operator,
                self.controller,
                "receipts",
                (),
            )
            .unwrap()
            .unwrap()
    }
    fn retained_outcome(
        &self,
        actor: Principal,
        intent: Intent,
    ) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
        self.harness
            .pic
            .query_candid_as(
                self.service,
                actor,
                "blob_funding_outcome",
                (outcome_request(intent),),
            )
            .unwrap()
    }
}
fn input(intent: Intent) -> Input {
    Input {
        intent,
        operating_reserve: 1_000_000_000,
        other_liabilities: 0,
        callback_fault: None,
        callback_identity_fault: None,
    }
}

#[test]
fn cashier_transport_binds_exact_arguments_and_keeps_refunds_independent_of_reply_decoding() {
    for (accepted, reply, expected) in [
        (
            0,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            400,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            900,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            400,
            FundingReplyMode::InternalError,
            FundingOutcome::ProviderError(FundingProviderErrorView::InternalError),
        ),
        (
            400,
            FundingReplyMode::Malformed,
            FundingOutcome::InvalidReply,
        ),
        (
            400,
            FundingReplyMode::UnknownError,
            FundingOutcome::InvalidReply,
        ),
        (400, FundingReplyMode::Reject, FundingOutcome::Rejected(4)),
    ] {
        let f = Fixture::with_cashier();
        let intent = Intent {
            target_balance: Some(u128::MAX),
            ..f.funding_intent(1, 900)
        };
        f.configure_cashier(intent, accepted, reply);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        let result = f.transport(f.operator, input(intent)).unwrap();
        assert_eq!(
            (result.accepted, result.refunded, result.outcome),
            (accepted, Some(900 - accepted), expected)
        );
        assert!(result.call_cost > 0);
        let retained = f.retained_outcome(f.operator, intent).unwrap().unwrap();
        assert_eq!(retained.request, outcome_request(intent));
        assert_eq!(retained.response, Some(expected_response(expected)));
        assert_eq!(
            retained.reconciliation,
            if accepted == 0 {
                FundingReconciliationView::NoTransfer
            } else {
                FundingReconciliationView::CreditRequired(accepted)
            }
        );
        assert_eq!(
            f.funding_lookup(f.operator, intent),
            Ok(Some(Phase::Callback(900 - accepted)))
        );
        let allocation = f.funding_allocation();
        assert_eq!(
            (
                allocation.available,
                allocation.accepted,
                allocation.refunded,
                allocation.uncertain
            ),
            (1000 - accepted, accepted, 900 - accepted, 0)
        );
        assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Phase));
        assert_eq!(
            f.incoming(),
            vec![FundingReceiptRecord {
                id: 1,
                available: 900,
                accepted
            }]
        );
        assert_eq!(f.funding_allocation(), allocation);
        for actor in [f.controller, f.tenant, Principal::anonymous()] {
            assert_eq!(
                f.retained_outcome(actor, intent),
                Err(FundingOutcomeFailure::Denied)
            );
        }
        f.upgrade_storage();
        assert_eq!(
            f.retained_outcome(f.operator, intent),
            Ok(Some(FundingOutcomeResponse {
                fenced: true,
                ..retained
            }))
        );
        assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Fenced));
    }
}

#[test]
fn cashier_transport_requires_exact_prepared_operator_intent_and_full_liquidity() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 900, FundingReplyMode::Success);
    assert_eq!(
        f.transport(f.operator, input(intent)),
        Err(Failure::Unknown)
    );
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let reserved = f.funding_allocation();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(f.transport(actor, input(intent)), Err(Failure::Denied));
    }
    for changed in [
        Intent {
            offered: 899,
            ..intent
        },
        Intent {
            target_balance: Some(2),
            ..intent
        },
    ] {
        assert_eq!(
            f.transport(f.operator, input(changed)),
            Err(Failure::Conflict)
        );
    }
    assert_eq!(
        f.transport(
            f.operator,
            input(Intent {
                cashier: f.other,
                ..intent
            })
        ),
        Err(Failure::Binding)
    );
    assert_eq!(f.funding_allocation(), reserved);
    let blocked = f
        .transport(
            f.operator,
            Input {
                operating_reserve: u128::MAX,
                ..input(intent)
            },
        )
        .unwrap();
    assert_eq!(
        (blocked.accepted, blocked.refunded, blocked.outcome),
        (0, None, FundingOutcome::LiquidityBlocked)
    );
    assert_eq!(f.incoming(), []);
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::NotEnqueued))
    );
    assert_eq!(f.funding_allocation().not_enqueued, 900);
    let retained = f.retained_outcome(f.operator, intent).unwrap().unwrap();
    assert_eq!(retained.response, Some(FundingResponse::NotDispatched));
    assert_eq!(
        retained.reconciliation,
        FundingReconciliationView::NoTransfer
    );
    assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Phase));
}

#[test]
fn cashier_receiver_trap_rolls_back_acceptance_and_returns_the_full_attachment() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Trap);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let result = f.transport(f.operator, input(intent)).unwrap();
    assert_eq!(
        (result.accepted, result.refunded, result.outcome),
        (0, Some(900), FundingOutcome::Rejected(5))
    );
    assert_eq!(f.incoming(), []);
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Callback(900)))
    );
    let allocation = f.funding_allocation();
    assert_eq!(
        (
            allocation.available,
            allocation.accepted,
            allocation.refunded,
            allocation.uncertain
        ),
        (1000, 0, 900, 0)
    );
    assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Phase));
    assert_eq!(f.funding_allocation(), allocation);
}

#[test]
fn cashier_callback_write_trap_preserves_uncertainty_after_receiver_acceptance() {
    let f = Fixture::with_cashier();
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Success);
    f.funding(f.operator, intent, Action::Prepare).unwrap();
    let reserved = f.funding_allocation();
    let prepared = f.retained_outcome(f.operator, intent).unwrap().unwrap();
    assert_eq!(prepared.response, None);
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_funding_transport",
            candid::encode_one(Input {
                callback_fault: Some(WriteFault::FundingAccounting),
                ..input(intent)
            })
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(
        f.incoming(),
        vec![FundingReceiptRecord {
            id: 1,
            available: 900,
            accepted: 400
        }]
    );
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(f.funding_allocation(), reserved);
    let uncertain = f.retained_outcome(f.operator, intent).unwrap().unwrap();
    assert_eq!(uncertain.response, None);
    assert_eq!(
        uncertain.reconciliation,
        FundingReconciliationView::TransferUnknown(900)
    );
    assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Phase));
    f.upgrade_storage();
    assert_eq!(
        f.funding_lookup(f.operator, intent),
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Fenced));
    assert_eq!(
        f.retained_outcome(f.operator, intent),
        Ok(Some(FundingOutcomeResponse {
            fenced: true,
            ..uncertain
        }))
    );
    assert_eq!(
        f.funding_allocation(),
        Allocation {
            fenced: true,
            ..reserved
        }
    );
}

#[test]
fn mismatched_callback_request_never_releases_the_original_uncertain_offer() {
    for fault in [
        CallbackIdentityFault::Offered,
        CallbackIdentityFault::Account,
        CallbackIdentityFault::TargetBalance,
    ] {
        let f = Fixture::with_cashier();
        let intent = f.funding_intent(1, 900);
        f.configure_cashier(intent, 400, FundingReplyMode::Success);
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        let totals = f.funding_allocation();
        assert_eq!(
            f.transport(
                f.operator,
                Input {
                    callback_identity_fault: Some(fault),
                    ..input(intent)
                }
            ),
            Err(Failure::Conflict)
        );
        assert_eq!(f.incoming()[0].accepted, 400);
        let retained = f.retained_outcome(f.operator, intent).unwrap().unwrap();
        assert_eq!(retained.phase, FundingPhase::Uncertain);
        assert_eq!(retained.response, None);
        assert_eq!(
            retained.reconciliation,
            FundingReconciliationView::TransferUnknown(900)
        );
        assert_eq!(f.funding_allocation(), totals);
        assert_eq!(f.transport(f.operator, input(intent)), Err(Failure::Phase));
    }
}

#[test]
fn actual_cashier_acceptance_is_not_cleared_by_a_later_fully_refunded_call() {
    use blob_test_protocol::status::FundingActivityView;
    let f = Fixture::with_cashier();
    for (id, offered, accepted) in [(1, 900, 400), (2, 500, 0)] {
        let intent = f.funding_intent(id, offered);
        f.configure_cashier(intent, accepted, FundingReplyMode::Success);
        // Explicit local effect controls; production activity admission is separate.
        f.funding(f.operator, intent, Action::Prepare).unwrap();
        f.transport(f.operator, input(intent)).unwrap();
        let summary = f.funding_summary(f.operator, f.funding_scope()).unwrap();
        assert_eq!(summary.allocation.accepted, 400);
        assert_eq!(summary.allocation.uncertain, 0);
        assert_eq!(summary.local_activity, FundingActivityView::Uncertain);
        assert_eq!(summary.last_operation, Some(id));
    }
    let before = f.funding_summary(f.operator, f.funding_scope()).unwrap();
    f.upgrade_storage();
    let after = f.funding_summary(f.operator, f.funding_scope()).unwrap();
    assert_eq!(after.allocation.accepted, before.allocation.accepted);
    assert_eq!(after.local_activity, FundingActivityView::Uncertain);
    assert!(after.allocation.fenced);
}
