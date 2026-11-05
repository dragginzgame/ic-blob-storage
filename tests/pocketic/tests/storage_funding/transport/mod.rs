//! Actual IC transport/refunds through the shared journal and a labelled substitute.
use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

mod dispatch;
use blob_test_protocol::{
    funding::{
        FundingFailure, FundingOutcome, FundingProviderErrorView, FundingReceiptRecord,
        FundingReplyMode, FundingRequest,
    },
    storage::funding::transport::{CallbackIdentityFault, Input, Observation, Substitute},
};
use ic_blob_storage::ops::caffeine::funding::request::CashierTopUpRequest;
use ic_blob_storage_contracts::dto::funding::FundingPhase;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReconciliation as FundingReconciliationView;
use ic_blob_storage_contracts::dto::funding::outcome::FundingReportedBalance;
use ic_blob_storage_contracts::dto::funding::outcome::FundingResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
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
            FundingProviderErrorView::NotAuthorized(p) => FundingResponse::NotAuthorized(p.into()),
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
    async fn upgrade_storage(&self) {
        self.harness
            .pic
            .upgrade_canister(
                self.service,
                Self::wasm(),
                Self::installation("fixture_post_upgrade", self.operator),
                Some(self.controller),
            )
            .await
            .unwrap();
    }
    async fn with_cashier() -> Self {
        let harness = Harness::new().await;
        let cashier = harness.pic.create_canister().await.unwrap();
        // The test controller acts for this configured canister operator through
        // PocketIC ingress. Both operator and Cashier bindings remain explicit.
        let f = Self::with_operator(harness, cashier).await;
        f.harness
            .pic
            .install_canister(
                cashier,
                std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
                blob_test_protocol::fixture_wire::encode_args(
                    "fixture_init",
                    (
                        f.service,
                        f.controller,
                        blob_test_protocol::funding::budget::FundingBudgetInput {
                            allocated: 1000,
                            reserve: 100,
                            operating_reserve: 1_000_000_000,
                            other_liabilities: 0,
                        },
                    ),
                )
                .unwrap(),
                None,
            )
            .await
            .unwrap();
        f
    }
    async fn configure_cashier(&self, intent: Intent, accepted: u128, reply: FundingReplyMode) {
        let request = CashierTopUpRequest::new(
            intent.cashier,
            intent.account,
            NonZeroU128::new(intent.offered).unwrap(),
            intent
                .target_balance
                .map(|value| NonZeroU128::new(value).unwrap()),
        )
        .unwrap();
        let result: Result<(), FundingFailure> = fixture_calls::update_fixture(
            &self.harness.pic,
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
        .await
        .map(|(reply,)| reply)
        .unwrap();
        result.unwrap();
    }
    async fn transport(&self, actor: Principal, input: Input) -> Result<Observation, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "fixture_funding_transport",
                (input,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn incoming(&self) -> Vec<FundingReceiptRecord> {
        fixture_calls::query_fixture::<(Option<Vec<FundingReceiptRecord>>,), _>(
            &self.harness.pic,
            self.operator,
            self.controller,
            "receipts",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
        .unwrap()
    }
    async fn retained_outcome(
        &self,
        actor: Principal,
        intent: Intent,
    ) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                "blob_funding_outcome",
                outcome_request(intent),
                REPLY_BYTES,
            )
            .await
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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One actual callback journey keeps attachment, refund and provider-reply decoding independent"
)]
async fn cashier_transport_binds_exact_arguments_and_keeps_refunds_independent_of_reply_decoding() {
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
        let f = Fixture::with_cashier().await;
        let intent = Intent {
            target_balance: Some(u128::MAX),
            ..f.funding_intent(1, 900)
        };
        f.configure_cashier(intent, accepted, reply).await;
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        let result = f.transport(f.operator, input(intent)).await.unwrap();
        assert_eq!(
            (result.accepted, result.refunded, result.outcome),
            (accepted, Some(900 - accepted), expected)
        );
        assert!(result.call_cost > 0);
        let retained = f
            .retained_outcome(f.operator, intent)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retained.request, outcome_request(intent));
        assert_eq!(retained.response, Some(expected_response(expected)));
        assert_eq!(
            retained.reconciliation,
            if accepted == 0 {
                FundingReconciliationView::NoTransfer
            } else if expected == FundingOutcome::ReportedSuccess {
                FundingReconciliationView::ProviderAcknowledged(accepted)
            } else {
                FundingReconciliationView::CreditRequired(accepted)
            }
        );
        assert_eq!(
            f.funding_lookup(f.operator, intent).await,
            Ok(Some(Phase::Callback(900 - accepted)))
        );
        let allocation = f.funding_allocation().await;
        assert_eq!(
            (
                allocation.available,
                allocation.accepted,
                allocation.refunded,
                allocation.uncertain
            ),
            (1000 - accepted, accepted, 900 - accepted, 0)
        );
        assert_eq!(
            f.transport(f.operator, input(intent)).await,
            Err(Failure::Phase)
        );
        assert_eq!(
            f.incoming().await,
            vec![FundingReceiptRecord {
                id: 1,
                available: 900,
                accepted
            }]
        );
        assert_eq!(f.funding_allocation().await, allocation);
        for actor in [f.controller, f.tenant, Principal::anonymous()] {
            assert_eq!(
                f.retained_outcome(actor, intent).await,
                Err(FundingOutcomeFailure::Denied)
            );
        }
        f.upgrade_storage().await;
        assert_eq!(
            f.retained_outcome(f.operator, intent).await,
            Ok(Some(FundingOutcomeResponse {
                fenced: true,
                ..retained
            }))
        );
        assert_eq!(
            f.transport(f.operator, input(intent)).await,
            Err(Failure::Fenced)
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cashier_transport_requires_exact_prepared_operator_intent_and_full_liquidity() {
    let f = Fixture::with_cashier().await;
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 900, FundingReplyMode::Success)
        .await;
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Unknown)
    );
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let reserved = f.funding_allocation().await;
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.transport(actor, input(intent)).await,
            Err(Failure::Denied)
        );
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
            f.transport(f.operator, input(changed)).await,
            Err(Failure::Conflict)
        );
    }
    assert_eq!(
        f.transport(
            f.operator,
            input(Intent {
                cashier: f.other.into(),
                ..intent
            })
        )
        .await,
        Err(Failure::Binding)
    );
    assert_eq!(f.funding_allocation().await, reserved);
    let blocked = f
        .transport(
            f.operator,
            Input {
                operating_reserve: u128::MAX,
                ..input(intent)
            },
        )
        .await
        .unwrap();
    assert_eq!(
        (blocked.accepted, blocked.refunded, blocked.outcome),
        (0, None, FundingOutcome::LiquidityBlocked)
    );
    assert_eq!(f.incoming().await, []);
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::NotEnqueued))
    );
    assert_eq!(f.funding_allocation().await.not_enqueued, 900);
    let retained = f
        .retained_outcome(f.operator, intent)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(retained.response, Some(FundingResponse::NotDispatched));
    assert_eq!(
        retained.reconciliation,
        FundingReconciliationView::NoTransfer
    );
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Phase)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cashier_receiver_trap_rolls_back_acceptance_and_returns_the_full_attachment() {
    let f = Fixture::with_cashier().await;
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Trap)
        .await;
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let result = f.transport(f.operator, input(intent)).await.unwrap();
    assert_eq!(
        (result.accepted, result.refunded, result.outcome),
        (0, Some(900), FundingOutcome::Rejected(5))
    );
    assert_eq!(f.incoming().await, []);
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Callback(900)))
    );
    let allocation = f.funding_allocation().await;
    assert_eq!(
        (
            allocation.available,
            allocation.accepted,
            allocation.refunded,
            allocation.uncertain
        ),
        (1000, 0, 900, 0)
    );
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Phase)
    );
    assert_eq!(f.funding_allocation().await, allocation);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cashier_callback_write_trap_preserves_uncertainty_after_receiver_acceptance() {
    let f = Fixture::with_cashier().await;
    let intent = f.funding_intent(1, 900);
    f.configure_cashier(intent, 400, FundingReplyMode::Success)
        .await;
    f.funding(f.operator, intent, Action::Prepare)
        .await
        .unwrap();
    let reserved = f.funding_allocation().await;
    let prepared = f
        .retained_outcome(f.operator, intent)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(prepared.response, None);
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_funding_transport",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(Input {
                callback_fault: Some(WriteFault::FundingAccounting),
                ..input(intent)
            })
            .unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.incoming().await,
        vec![FundingReceiptRecord {
            id: 1,
            available: 900,
            accepted: 400
        }]
    );
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(f.funding_allocation().await, reserved);
    let uncertain = f
        .retained_outcome(f.operator, intent)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(uncertain.response, None);
    assert_eq!(
        uncertain.reconciliation,
        FundingReconciliationView::TransferUnknown(900)
    );
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Phase)
    );
    f.upgrade_storage().await;
    assert_eq!(
        f.funding_lookup(f.operator, intent).await,
        Ok(Some(Phase::Uncertain))
    );
    assert_eq!(
        f.transport(f.operator, input(intent)).await,
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.retained_outcome(f.operator, intent).await,
        Ok(Some(FundingOutcomeResponse {
            fenced: true,
            ..uncertain
        }))
    );
    assert_eq!(
        f.funding_allocation().await,
        Allocation {
            fenced: true,
            ..reserved
        }
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mismatched_callback_request_never_releases_the_original_uncertain_offer() {
    for fault in [
        CallbackIdentityFault::Offered,
        CallbackIdentityFault::Account,
        CallbackIdentityFault::TargetBalance,
    ] {
        let f = Fixture::with_cashier().await;
        let intent = f.funding_intent(1, 900);
        f.configure_cashier(intent, 400, FundingReplyMode::Success)
            .await;
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        let totals = f.funding_allocation().await;
        assert_eq!(
            f.transport(
                f.operator,
                Input {
                    callback_identity_fault: Some(fault),
                    ..input(intent)
                }
            )
            .await,
            Err(Failure::Conflict)
        );
        assert_eq!(f.incoming().await[0].accepted, 400);
        let retained = f
            .retained_outcome(f.operator, intent)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retained.phase, FundingPhase::Uncertain);
        assert_eq!(retained.response, None);
        assert_eq!(
            retained.reconciliation,
            FundingReconciliationView::TransferUnknown(900)
        );
        assert_eq!(f.funding_allocation().await, totals);
        assert_eq!(
            f.transport(f.operator, input(intent)).await,
            Err(Failure::Phase)
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_cashier_acceptance_is_not_cleared_by_a_later_fully_refunded_call() {
    use blob_test_protocol::status::FundingActivityView;
    let f = Fixture::with_cashier().await;
    for (id, offered, accepted) in [(1, 900, 400), (2, 500, 0)] {
        let intent = f.funding_intent(id, offered);
        f.configure_cashier(intent, accepted, FundingReplyMode::Malformed)
            .await;
        // Explicit local effect controls; production activity admission is separate.
        f.funding(f.operator, intent, Action::Prepare)
            .await
            .unwrap();
        f.transport(f.operator, input(intent)).await.unwrap();
        let summary = f
            .funding_summary(f.operator, f.funding_scope())
            .await
            .unwrap();
        assert_eq!(summary.allocation.accepted, 400);
        assert_eq!(summary.allocation.uncertain, 0);
        assert_eq!(summary.local_activity, FundingActivityView::Uncertain);
        assert_eq!(summary.last_operation, Some(id));
    }
    let before = f
        .funding_summary(f.operator, f.funding_scope())
        .await
        .unwrap();
    f.upgrade_storage().await;
    let after = f
        .funding_summary(f.operator, f.funding_scope())
        .await
        .unwrap();
    assert_eq!(after.allocation.accepted, before.allocation.accepted);
    assert_eq!(after.local_activity, FundingActivityView::Uncertain);
    assert!(after.allocation.fenced);
}
