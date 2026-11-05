//! Real cycle acceptance/refund and callback rollback with a local substitute.
//! Same-release inspection-only journal recovery; no deployed economics or snapshot safety.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
mod funding_budget;
mod funding_recovery;
use ic_blob_storage_pocketic_tests::installation;
mod support;
use ic_blob_storage_pocketic_tests::service_calls::canister_rejection;

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingAttemptStatusView, FundingFailure, FundingObservation,
    FundingOperatorStatusView, FundingOutcome, FundingProviderErrorView, FundingReceiptRecord,
    FundingReconciliationView, FundingReplyMode, FundingRequest, FundingUpgradeArgs,
};
use blob_test_protocol::status::{FundingActivityView, OperatorBlockerView};
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::RejectCode;
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    sender: Principal,
    receiver: Principal,
    driver: Principal,
}

impl Fixture {
    async fn new() -> Self {
        Self::with_budget(blob_test_protocol::funding::budget::FundingBudgetInput {
            operating_reserve: 1_000_000_000,
            other_liabilities: 0,
            allocated: 100_000_000_000_000,
            reserve: 1_000_000_000_000,
        })
        .await
    }

    async fn with_budget(budget: blob_test_protocol::funding::budget::FundingBudgetInput) -> Self {
        let harness = Harness::new().await;
        let pic = &harness.pic;
        let driver = fake_principal(11);
        let create = || installation::funded_canister(pic, 2_000_000_000_000);
        let sender = create().await;
        let receiver = create().await;
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).expect("funding Wasm");
        for (canister, peer) in [(sender, receiver), (receiver, sender)] {
            pic.install_canister(
                canister,
                wasm.clone(),
                blob_test_protocol::fixture_wire::encode_args(
                    "fixture_init",
                    (peer, driver, budget),
                )
                .expect("fixture init"),
                None,
            )
            .await
            .unwrap();
        }
        Self {
            harness,
            sender,
            receiver,
            driver,
        }
    }

    async fn fund(
        &self,
        caller: Principal,
        request: FundingRequest,
    ) -> Result<FundingObservation, FundingFailure> {
        fixture_calls::update_fixture(&self.harness.pic, self.sender, caller, "fund", (request,))
            .await
            .map(|(reply,)| reply)
            .expect("funding call completed")
    }

    async fn attempts(&self) -> Vec<FundingAttemptRecord> {
        let result: Option<Vec<FundingAttemptRecord>> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.sender,
            self.driver,
            "attempts",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("sender journal");
        result.expect("driver allowed")
    }

    async fn receipts(&self) -> Vec<FundingReceiptRecord> {
        let result: Option<Vec<FundingReceiptRecord>> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.receiver,
            self.driver,
            "receipts",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("receiver journal");
        result.expect("driver allowed")
    }

    async fn status_as(
        &self,
        canister: Principal,
        caller: Principal,
    ) -> Option<FundingOperatorStatusView> {
        fixture_calls::query_fixture(&self.harness.pic, canister, caller, "operator_status", ())
            .await
            .map(|(reply,)| reply)
            .expect("read-only funding diagnosis")
    }

    async fn status_unchanged(&self) -> FundingOperatorStatusView {
        let pic = &self.harness.pic;
        let before = [
            pic.get_stable_memory(self.sender).await.unwrap(),
            pic.get_stable_memory(self.receiver).await.unwrap(),
        ];
        let attempts = self.attempts().await;
        let receipts = self.receipts().await;
        let status = self
            .status_as(self.sender, self.driver)
            .await
            .expect("driver status");
        assert_eq!(
            self.status_as(self.sender, self.driver).await,
            Some(status.clone())
        );
        let incoming = self
            .status_as(self.receiver, self.driver)
            .await
            .expect("peer driver status");
        assert_eq!(incoming.service, self.receiver);
        assert_eq!(incoming.peer, self.sender);
        assert_eq!(incoming.receipts, receipts);
        assert_eq!(incoming.attempts, []);
        assert_eq!(incoming.funding_activity, FundingActivityView::Clear);
        assert_eq!(status.service, self.sender);
        assert_eq!(status.peer, self.receiver);
        assert_eq!(status.attempts.len(), attempts.len());
        assert_eq!(status.receipts, []);
        assert!(!status.provider_qualified);
        assert!(!status.billing_configured);
        assert_eq!(status.provider_balance, None);
        assert_eq!(status.available_funding_cycles, None);
        for blocker in [
            OperatorBlockerView::ProviderUnqualified,
            OperatorBlockerView::GatewaysMissing,
            OperatorBlockerView::BillingNotConfigured,
        ] {
            assert!(status.blockers.contains(&blocker));
            assert!(incoming.blockers.contains(&blocker));
        }
        for report in [&status, &incoming] {
            assert_eq!(
                report
                    .blockers
                    .contains(&OperatorBlockerView::RecoveryFenced),
                report.fenced
            );
            assert_eq!(
                report
                    .blockers
                    .contains(&OperatorBlockerView::RecoveryUnknown),
                !report.fenced
            );
        }
        assert_eq!(status.warnings, []);
        assert_eq!(self.attempts().await, attempts);
        assert_eq!(self.receipts().await, receipts);
        assert!(
            [
                pic.get_stable_memory(self.sender).await.unwrap(),
                pic.get_stable_memory(self.receiver).await.unwrap()
            ] == before,
            "read-only status changed stable journals"
        );
        status
    }

    async fn upgrade_both(&self) {
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM"))
            .expect("same-release funding Wasm");
        for canister in [self.sender, self.receiver] {
            self.harness
                .pic
                .upgrade_canister(
                    canister,
                    wasm.clone(),
                    blob_test_protocol::fixture_wire::encode_one(
                        "fixture_post_upgrade",
                        FundingUpgradeArgs {
                            trap_after_restore: false,
                        },
                    )
                    .expect("upgrade args"),
                    None,
                )
                .await
                .expect("restore fixture through host-owned ic-storage");
        }
    }

    async fn assert_completed_status(
        &self,
        request: FundingRequest,
        observation: FundingObservation,
        credit_outstanding: bool,
    ) {
        let status = self.status_unchanged().await;
        assert_eq!(
            status.attempts.last(),
            Some(&FundingAttemptStatusView {
                id: request.id,
                offered: request.offered,
                refunded: observation.refunded,
                transport_accepted: observation.transport_accepted,
                outcome: Some(observation.outcome),
                provider_credit: None,
                reconciliation: observation.reconciliation,
            })
        );
        let activity = if credit_outstanding {
            FundingActivityView::Uncertain
        } else {
            FundingActivityView::Clear
        };
        assert_eq!(status.funding_activity, activity);
        assert_eq!(
            status
                .blockers
                .contains(&OperatorBlockerView::FundingUncertain),
            credit_outstanding
        );
    }

    async fn assert_unknown_status(&self, request: FundingRequest) -> FundingOperatorStatusView {
        let budget = self
            .status_as(self.sender, self.driver)
            .await
            .unwrap()
            .budget;
        assert_eq!(budget.reserved_or_uncertain, request.offered);
        assert_eq!(budget.available, budget.allocated - request.offered);
        assert_eq!(
            (budget.accepted, budget.refunded, budget.not_enqueued),
            (0, 0, 0)
        );
        let status = self.status_unchanged().await;
        assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
        assert!(
            status
                .blockers
                .contains(&OperatorBlockerView::FundingUncertain)
        );
        assert_eq!(
            status.attempts,
            vec![FundingAttemptStatusView {
                id: request.id,
                offered: request.offered,
                refunded: None,
                transport_accepted: None,
                outcome: None,
                provider_credit: None,
                reconciliation: FundingReconciliationView::TransferUnknown(request.offered),
            }]
        );
        status
    }
}

fn request(id: u64, accept: u128, reply: FundingReplyMode) -> FundingRequest {
    FundingRequest {
        id,
        offered: 100_000_000,
        accept,
        reply,
        trap_callback: false,
    }
}

fn fenced_status(mut status: FundingOperatorStatusView) -> FundingOperatorStatusView {
    status.fenced = true;
    for blocker in &mut status.blockers {
        if *blocker == OperatorBlockerView::RecoveryUnknown {
            *blocker = OperatorBlockerView::RecoveryFenced;
        }
    }
    status
}

fn reply_cases() -> Vec<(u128, FundingReplyMode, FundingOutcome)> {
    vec![
        (
            0,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            37_000_001,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            100_000_000,
            FundingReplyMode::Success,
            FundingOutcome::ReportedSuccess,
        ),
        (
            19_000_003,
            FundingReplyMode::InternalError,
            FundingOutcome::ProviderError(FundingProviderErrorView::InternalError),
        ),
        (
            23_000_003,
            FundingReplyMode::NotAuthorized,
            FundingOutcome::ProviderError(FundingProviderErrorView::NotAuthorized(
                Principal::anonymous(),
            )),
        ),
        (
            100_000_000,
            FundingReplyMode::AccountBalanceOverflow,
            FundingOutcome::ProviderError(FundingProviderErrorView::AccountBalanceOverflow),
        ),
        (
            31_000_003,
            FundingReplyMode::TopUpWithoutCycles,
            FundingOutcome::ProviderError(FundingProviderErrorView::TopUpWithoutCycles),
        ),
        (
            0,
            FundingReplyMode::TopUpWithoutCycles,
            FundingOutcome::ProviderError(FundingProviderErrorView::TopUpWithoutCycles),
        ),
        (
            17_000_001,
            FundingReplyMode::LedgerReport,
            FundingOutcome::InvalidReply,
        ),
        (
            11_000_001,
            FundingReplyMode::UnknownError,
            FundingOutcome::InvalidReply,
        ),
        (
            29_000_007,
            FundingReplyMode::Malformed,
            FundingOutcome::InvalidReply,
        ),
        (
            41_000_009,
            FundingReplyMode::Reject,
            FundingOutcome::Rejected(4),
        ),
    ]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn refunds_are_call_specific_and_independent_of_cashier_reply_decoding() {
    let fixture = Fixture::new().await;
    let mut credit_outstanding = false;
    for (index, (accept, reply, outcome)) in reply_cases().into_iter().enumerate() {
        let request = request(u64::try_from(index).expect("small index"), accept, reply);
        let observation = fixture
            .fund(fixture.driver, request)
            .await
            .expect("admitted");
        assert_eq!(
            observation,
            FundingObservation {
                refunded: Some(request.offered - accept),
                transport_accepted: Some(accept),
                outcome,
                reconciliation: if accept == 0 {
                    FundingReconciliationView::NoTransfer
                } else {
                    FundingReconciliationView::CreditRequired(accept)
                },
            }
        );
        assert_eq!(
            fixture.receipts().await.last(),
            Some(&FundingReceiptRecord {
                id: request.id,
                available: request.offered,
                accepted: accept,
            })
        );
        assert_eq!(
            fixture.attempts().await.last(),
            Some(&FundingAttemptRecord {
                request,
                observation: Some(observation),
            })
        );
        credit_outstanding |= accept > 0;
        fixture
            .assert_completed_status(request, observation, credit_outstanding)
            .await;
        let receipts = fixture.receipts().await;
        assert_eq!(
            fixture.fund(fixture.driver, request).await,
            Err(FundingFailure::AlreadyAdmitted)
        );
        assert_eq!(fixture.receipts().await, receipts);
    }
    let attempts = fixture.attempts().await;
    let receipts = fixture.receipts().await;
    let status = fixture.status_unchanged().await;
    fixture.upgrade_both().await;
    assert_eq!(fixture.status_unchanged().await, fenced_status(status));
    assert_eq!(fixture.attempts().await, attempts);
    assert_eq!(fixture.receipts().await, receipts);
    // Restored journals cannot pay again even with changed identities or parameters.
    for entry in &attempts {
        let mut changed = entry.request;
        changed.accept = 0;
        changed.reply = FundingReplyMode::Malformed;
        assert_eq!(
            fixture.fund(fixture.driver, changed).await,
            Err(FundingFailure::Fenced)
        );
    }
    let next = request(99, 13_000_001, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next).await,
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.attempts().await, attempts);
    assert_eq!(fixture.receipts().await, receipts);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn callback_trap_retains_pending_intent_and_blocks_another_payment() {
    for reply in [
        FundingReplyMode::Success,
        FundingReplyMode::TopUpWithoutCycles,
        FundingReplyMode::LedgerReport,
    ] {
        assert_callback_trap_retains_pending_intent(reply).await;
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "One callback-trap journey retains the original intent and journals through failed and successful restoration"
)]
async fn assert_callback_trap_retains_pending_intent(reply: FundingReplyMode) {
    let fixture = Fixture::new().await;
    let mut request = request(7, 31_000_001, reply);
    request.trap_callback = true;
    let result = fixture
        .harness
        .pic
        .update_call(
            fixture.sender,
            fixture.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", request).expect("request"),
        )
        .await;
    assert_eq!(
        crate::canister_rejection(&result.expect_err("real callback trap")).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        fixture.attempts().await,
        vec![FundingAttemptRecord {
            request,
            observation: None
        }]
    );
    let receipts = vec![FundingReceiptRecord {
        id: request.id,
        available: request.offered,
        accepted: request.accept,
    }];
    assert_eq!(fixture.receipts().await, receipts);
    // A failed upgrade must leave the previous executable and its journals usable.
    // Exercise both sides before trying a successful upgrade of the same release.
    let attempts = fixture.attempts().await;
    let status = fixture.assert_unknown_status(request).await;
    let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).expect("funding Wasm");
    for canister in [fixture.sender, fixture.receiver] {
        let failure = fixture
            .harness
            .pic
            .upgrade_canister(
                canister,
                wasm.clone(),
                blob_test_protocol::fixture_wire::encode_one(
                    "fixture_post_upgrade",
                    FundingUpgradeArgs {
                        trap_after_restore: true,
                    },
                )
                .expect("upgrade args"),
                None,
            )
            .await
            .expect_err("real post_upgrade trap");
        assert_eq!(
            crate::canister_rejection(&failure).reject_code,
            RejectCode::CanisterError
        );
        assert_eq!(fixture.attempts().await, attempts);
        assert_eq!(fixture.receipts().await, receipts);
        assert_eq!(fixture.status_unchanged().await, status);
        let mut next = request;
        next.id += 1;
        next.trap_callback = false;
        assert_eq!(
            fixture.fund(fixture.driver, next).await,
            Err(FundingFailure::InProgress)
        );
        assert_eq!(fixture.receipts().await, receipts);
    }
    fixture.upgrade_both().await;
    let status = fenced_status(status);
    assert_eq!(fixture.status_unchanged().await, status);
    fixture
        .harness
        .pic
        .advance_time(std::time::Duration::from_hours(24))
        .await
        .unwrap();
    fixture.harness.pic.tick().await.unwrap();
    assert_eq!(fixture.status_unchanged().await, status);
    assert_eq!(
        fixture.attempts().await,
        vec![FundingAttemptRecord {
            request,
            observation: None
        }]
    );
    assert_eq!(fixture.receipts().await, receipts);
    assert_eq!(
        fixture.fund(Principal::anonymous(), request).await,
        Err(FundingFailure::Denied)
    );
    assert_eq!(
        fixture.fund(fixture.driver, request).await,
        Err(FundingFailure::Fenced)
    );
    request.id += 1;
    request.trap_callback = false;
    assert_eq!(
        fixture.fund(fixture.driver, request).await,
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts().await, receipts);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn receiver_trap_rolls_back_acceptance_and_receipt_before_refunding_the_sender() {
    let fixture = Fixture::new().await;
    let prior = request(1, 13_000_001, FundingReplyMode::Success);
    fixture
        .fund(fixture.driver, prior)
        .await
        .expect("prior payment");
    let receipts = fixture.receipts().await;
    let trapped = request(2, 37_000_001, FundingReplyMode::Trap);
    let observation = fixture
        .fund(fixture.driver, trapped)
        .await
        .expect("receiver failure observed");
    assert_eq!(
        observation,
        FundingObservation {
            refunded: Some(trapped.offered),
            transport_accepted: Some(0),
            outcome: FundingOutcome::Rejected(RejectCode::CanisterError as u32),
            reconciliation: FundingReconciliationView::NoTransfer,
        }
    );
    let attempts = fixture.attempts().await;
    assert_eq!(
        attempts.last(),
        Some(&FundingAttemptRecord {
            request: trapped,
            observation: Some(observation),
        })
    );
    assert_eq!(fixture.receipts().await, receipts);
    fixture.upgrade_both().await;
    assert_eq!(fixture.attempts().await, attempts);
    assert_eq!(fixture.receipts().await, receipts);
    let status = fixture.status_unchanged().await;
    assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
    assert!(
        status
            .blockers
            .contains(&OperatorBlockerView::FundingUncertain)
    );
    assert_eq!(
        status.attempts.last().unwrap().reconciliation,
        FundingReconciliationView::NoTransfer
    );
    assert_eq!(
        status.attempts.first().unwrap().reconciliation,
        FundingReconciliationView::CreditRequired(prior.accept)
    );
    assert_eq!(
        fixture.fund(fixture.driver, trapped).await,
        Err(FundingFailure::Fenced)
    );
    // The receiver's trapped receipt remains absent, and neither side resumes.
    let next = request(3, 19_000_003, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next).await,
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts().await, receipts);
}

#[tokio::test(flavor = "multi_thread")]
async fn lifetime_journal_capacity_survives_upgrade_without_forgetting_payments() {
    let fixture = Fixture::new().await;
    let mut candidate = request(0, 1, FundingReplyMode::Success);
    loop {
        // Test-run cap, deliberately independent of the fixture's capacity constant.
        assert!(
            candidate.id < 100,
            "experiment must reach its bounded journal limit"
        );
        match fixture.fund(fixture.driver, candidate).await {
            Ok(_) => candidate.id += 1,
            Err(FundingFailure::Limit) => break,
            other => panic!("unexpected admission: {other:?}"),
        }
    }
    let attempts = fixture.attempts().await;
    let receipts = fixture.receipts().await;
    assert_ne!(receipts, []);
    let status = fixture.status_unchanged().await;
    fixture.upgrade_both().await;
    assert_eq!(fixture.status_unchanged().await, fenced_status(status));
    assert_eq!(
        fixture.fund(fixture.driver, candidate).await,
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.attempts().await, attempts);
    assert_eq!(fixture.receipts().await, receipts);
    assert_eq!(
        fixture.fund(fixture.driver, attempts[0].request).await,
        Err(FundingFailure::Fenced)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn liquidity_rejection_has_no_callback_refund_and_preserves_exact_unsent_history() {
    let fixture = Fixture::new().await;
    let prior = request(1, 13_000_001, FundingReplyMode::Success);
    fixture
        .fund(fixture.driver, prior)
        .await
        .expect("prior refunded call");
    let budget_before = fixture.status_unchanged().await.budget;
    let receipts = fixture.receipts().await;
    let mut unsent = request(2, 1, FundingReplyMode::Success);
    unsent.offered = fixture
        .harness
        .pic
        .cycle_balance(fixture.sender)
        .await
        .unwrap()
        + 1;
    let observation = fixture
        .fund(fixture.driver, unsent)
        .await
        .expect("local liquidity refusal observed");
    assert_eq!(
        observation,
        FundingObservation {
            refunded: None,
            transport_accepted: Some(0),
            outcome: FundingOutcome::LiquidityBlocked,
            reconciliation: FundingReconciliationView::NoTransfer,
        }
    );
    let attempts = fixture.attempts().await;
    assert_eq!(
        attempts.last(),
        Some(&FundingAttemptRecord {
            request: unsent,
            observation: Some(observation),
        })
    );
    assert_eq!(fixture.receipts().await, receipts);
    let budget_after = fixture.status_unchanged().await.budget;
    assert_eq!(budget_after.available, budget_before.available);
    assert_eq!(budget_after.accepted, budget_before.accepted);
    assert_eq!(budget_after.refunded, budget_before.refunded);
    assert_eq!(budget_after.not_enqueued, unsent.offered);
    assert_eq!(budget_after.reserved_or_uncertain, 0);
    assert_eq!(budget_after.revision, budget_before.revision + 2);
    fixture.upgrade_both().await;
    assert_eq!(fixture.status_unchanged().await.budget, budget_after);
    assert_eq!(fixture.attempts().await, attempts);
    assert_eq!(fixture.receipts().await, receipts);
    // Changed parameters do not turn the already admitted identity into a retry.
    let status = fixture.status_unchanged().await;
    assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
    assert_eq!(
        status.attempts.last(),
        Some(&FundingAttemptStatusView {
            id: unsent.id,
            offered: unsent.offered,
            refunded: None,
            transport_accepted: Some(0),
            outcome: Some(FundingOutcome::LiquidityBlocked),
            provider_credit: None,
            reconciliation: FundingReconciliationView::NoTransfer,
        })
    );
    assert_eq!(status.attempts.first().unwrap().provider_credit, None);
    let changed = request(unsent.id, 1, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, changed).await,
        Err(FundingFailure::Fenced)
    );
    let next = request(3, 19_000_003, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next).await,
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts().await, receipts);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn denied_and_invalid_requests_cannot_offer_cycles_or_inspect_journals() {
    let fixture = Fixture::new().await;
    let mut request = request(1, 1, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(Principal::anonymous(), request).await,
        Err(FundingFailure::Denied)
    );
    for (offered, accept) in [(0, 0), (u128::MAX, 0), (10, 11)] {
        request.offered = offered;
        request.accept = accept;
        assert_eq!(
            fixture.fund(fixture.driver, request).await,
            Err(FundingFailure::Limit)
        );
    }
    assert_eq!(fixture.attempts().await, []);
    assert_eq!(fixture.receipts().await, []);
    let status = fixture.status_unchanged().await;
    assert_eq!(status.funding_activity, FundingActivityView::Clear);
    assert!(
        !status
            .blockers
            .contains(&OperatorBlockerView::FundingUncertain)
    );
    // Anonymous is this fixture's actual controller, not its funding driver.
    for canister in [fixture.sender, fixture.receiver] {
        for caller in [
            Principal::anonymous(),
            fake_principal(99),
            fixture.sender,
            fixture.receiver,
        ] {
            assert_eq!(fixture.status_as(canister, caller).await, None);
        }
    }
    let hidden: Option<Vec<FundingAttemptRecord>> = fixture_calls::query_fixture(
        &fixture.harness.pic,
        fixture.sender,
        Principal::anonymous(),
        "attempts",
        (),
    )
    .await
    .map(|(reply,)| reply)
    .expect("private journal query");
    assert_eq!(hidden, None);
    let rejected = fixture
        .harness
        .pic
        .update_call(
            fixture.receiver,
            fixture.driver,
            "receive",
            blob_test_protocol::fixture_wire::encode_one("receive", request).expect("request"),
        )
        .await
        .expect_err("driver cannot impersonate peer");
    assert_eq!(
        crate::canister_rejection(&rejected).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(fixture.receipts().await, []);
    fixture.upgrade_both().await;
    assert_eq!(fixture.status_unchanged().await, fenced_status(status));
    assert_eq!(
        fixture
            .status_as(fixture.sender, Principal::anonymous())
            .await,
        None
    );
}
