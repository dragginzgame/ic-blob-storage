//! Real cycle acceptance/refund and callback rollback with a local substitute.
//! Same-release inspection-only journal recovery; no deployed economics or snapshot safety.

#![cfg(not(target_family = "wasm"))]

mod funding_recovery;
mod support;

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingAttemptStatusView, FundingFailure, FundingObservation,
    FundingOperatorStatusView, FundingOutcome, FundingReceiptRecord, FundingReconciliationView,
    FundingReplyMode, FundingRequest, FundingUpgradeArgs,
};
use blob_test_protocol::status::{FundingActivityView, OperatorBlockerView};
use candid::Principal;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CreateCanisterParams, RejectCode},
};
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    sender: Principal,
    receiver: Principal,
    driver: Principal,
}

impl Fixture {
    fn new() -> Self {
        let harness = Harness::new();
        let pic = &harness.pic;
        let driver = Fake::principal(11);
        let create = || {
            pic.create_canister_with_params(
                None,
                CreateCanisterParams {
                    cycles: Some(2_000_000_000_000),
                    ..CreateCanisterParams::default()
                },
            )
            .expect("explicit fixture cycle budget")
        };
        let sender = create();
        let receiver = create();
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).expect("funding Wasm");
        for (canister, peer) in [(sender, receiver), (receiver, sender)] {
            pic.install_canister(
                canister,
                wasm.clone(),
                candid::encode_args((peer, driver)).expect("fixture init"),
                None,
            );
        }
        Self {
            harness,
            sender,
            receiver,
            driver,
        }
    }

    fn fund(
        &self,
        caller: Principal,
        request: FundingRequest,
    ) -> Result<FundingObservation, FundingFailure> {
        self.harness
            .pic
            .update_candid_as(self.sender, caller, "fund", (request,))
            .expect("funding call completed")
    }

    fn attempts(&self) -> Vec<FundingAttemptRecord> {
        let result: Option<Vec<FundingAttemptRecord>> = self
            .harness
            .pic
            .query_candid_as(self.sender, self.driver, "attempts", ())
            .expect("sender journal");
        result.expect("driver allowed")
    }

    fn receipts(&self) -> Vec<FundingReceiptRecord> {
        let result: Option<Vec<FundingReceiptRecord>> = self
            .harness
            .pic
            .query_candid_as(self.receiver, self.driver, "receipts", ())
            .expect("receiver journal");
        result.expect("driver allowed")
    }

    fn status_as(
        &self,
        canister: Principal,
        caller: Principal,
    ) -> Option<FundingOperatorStatusView> {
        self.harness
            .pic
            .query_candid_as(canister, caller, "operator_status", ())
            .expect("read-only funding diagnosis")
    }

    fn status_unchanged(&self) -> FundingOperatorStatusView {
        let pic = &self.harness.pic;
        let before = [self.sender, self.receiver].map(|id| pic.get_stable_memory(id));
        let attempts = self.attempts();
        let receipts = self.receipts();
        let status = self
            .status_as(self.sender, self.driver)
            .expect("driver status");
        assert_eq!(
            self.status_as(self.sender, self.driver),
            Some(status.clone())
        );
        let incoming = self
            .status_as(self.receiver, self.driver)
            .expect("peer driver status");
        assert_eq!(incoming.service, self.receiver);
        assert_eq!(incoming.peer, self.sender);
        assert_eq!(incoming.receipts, receipts);
        assert!(incoming.attempts.is_empty());
        assert_eq!(incoming.funding_activity, FundingActivityView::Clear);
        assert_eq!(status.service, self.sender);
        assert_eq!(status.peer, self.receiver);
        assert_eq!(status.attempts.len(), attempts.len());
        assert!(status.receipts.is_empty());
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
        assert!(status.warnings.is_empty());
        assert_eq!(self.attempts(), attempts);
        assert_eq!(self.receipts(), receipts);
        assert!(
            [self.sender, self.receiver].map(|id| pic.get_stable_memory(id)) == before,
            "read-only status changed stable journals"
        );
        status
    }

    fn upgrade_both(&self) {
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM"))
            .expect("same-release funding Wasm");
        for canister in [self.sender, self.receiver] {
            self.harness
                .pic
                .upgrade_canister(
                    canister,
                    wasm.clone(),
                    candid::encode_one(FundingUpgradeArgs {
                        trap_after_restore: false,
                    })
                    .expect("upgrade args"),
                    None,
                )
                .expect("restore fixture through host-owned ic-memory");
        }
    }

    fn assert_completed_status(
        &self,
        request: FundingRequest,
        observation: FundingObservation,
        credit_outstanding: bool,
    ) {
        let status = self.status_unchanged();
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

    fn assert_unknown_status(&self, request: FundingRequest) -> FundingOperatorStatusView {
        let status = self.status_unchanged();
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

#[test]
fn refunds_are_call_specific_and_independent_of_cashier_reply_decoding() {
    let fixture = Fixture::new();
    let cases = [
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
            FundingReplyMode::ProviderError,
            FundingOutcome::ProviderError,
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
    ];
    let mut credit_outstanding = false;
    for (index, (accept, reply, outcome)) in cases.into_iter().enumerate() {
        let request = request(u64::try_from(index).expect("small index"), accept, reply);
        let observation = fixture.fund(fixture.driver, request).expect("admitted");
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
            fixture.receipts().last(),
            Some(&FundingReceiptRecord {
                id: request.id,
                available: request.offered,
                accepted: accept,
            })
        );
        assert_eq!(
            fixture.attempts().last(),
            Some(&FundingAttemptRecord {
                request,
                observation: Some(observation),
            })
        );
        credit_outstanding |= accept > 0;
        fixture.assert_completed_status(request, observation, credit_outstanding);
        let receipts = fixture.receipts();
        assert_eq!(
            fixture.fund(fixture.driver, request),
            Err(FundingFailure::AlreadyAdmitted)
        );
        assert_eq!(fixture.receipts(), receipts);
    }
    let attempts = fixture.attempts();
    let receipts = fixture.receipts();
    let status = fixture.status_unchanged();
    fixture.upgrade_both();
    assert_eq!(fixture.status_unchanged(), fenced_status(status));
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    // Restored journals cannot pay again even with changed identities or parameters.
    for entry in &attempts {
        let mut changed = entry.request;
        changed.accept = 0;
        changed.reply = FundingReplyMode::Malformed;
        assert_eq!(
            fixture.fund(fixture.driver, changed),
            Err(FundingFailure::Fenced)
        );
    }
    let next = request(99, 13_000_001, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
}

#[test]
fn callback_trap_retains_pending_intent_and_blocks_another_payment() {
    let fixture = Fixture::new();
    let mut request = request(7, 31_000_001, FundingReplyMode::Success);
    request.trap_callback = true;
    let result = fixture.harness.pic.update_call(
        fixture.sender,
        fixture.driver,
        "fund",
        candid::encode_one(request).expect("request"),
    );
    assert_eq!(
        result.expect_err("real callback trap").reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        fixture.attempts(),
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
    assert_eq!(fixture.receipts(), receipts);
    // A failed upgrade must leave the previous executable and its journals usable.
    // Exercise both sides before trying a successful upgrade of the same release.
    let attempts = fixture.attempts();
    let status = fixture.assert_unknown_status(request);
    let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).expect("funding Wasm");
    for canister in [fixture.sender, fixture.receiver] {
        let failure = fixture
            .harness
            .pic
            .upgrade_canister(
                canister,
                wasm.clone(),
                candid::encode_one(FundingUpgradeArgs {
                    trap_after_restore: true,
                })
                .expect("upgrade args"),
                None,
            )
            .expect_err("real post_upgrade trap");
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(fixture.attempts(), attempts);
        assert_eq!(fixture.receipts(), receipts);
        assert_eq!(fixture.status_unchanged(), status);
        let mut next = request;
        next.id += 1;
        next.trap_callback = false;
        assert_eq!(
            fixture.fund(fixture.driver, next),
            Err(FundingFailure::InProgress)
        );
        assert_eq!(fixture.receipts(), receipts);
    }
    fixture.upgrade_both();
    let status = fenced_status(status);
    assert_eq!(fixture.status_unchanged(), status);
    fixture
        .harness
        .pic
        .advance_time(std::time::Duration::from_hours(24));
    fixture.harness.pic.tick();
    assert_eq!(fixture.status_unchanged(), status);
    assert_eq!(
        fixture.attempts(),
        vec![FundingAttemptRecord {
            request,
            observation: None
        }]
    );
    assert_eq!(fixture.receipts(), receipts);
    assert_eq!(
        fixture.fund(Principal::anonymous(), request),
        Err(FundingFailure::Denied)
    );
    assert_eq!(
        fixture.fund(fixture.driver, request),
        Err(FundingFailure::Fenced)
    );
    request.id += 1;
    request.trap_callback = false;
    assert_eq!(
        fixture.fund(fixture.driver, request),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts(), receipts);
}

#[test]
fn receiver_trap_rolls_back_acceptance_and_receipt_before_refunding_the_sender() {
    let fixture = Fixture::new();
    let prior = request(1, 13_000_001, FundingReplyMode::Success);
    fixture.fund(fixture.driver, prior).expect("prior payment");
    let receipts = fixture.receipts();
    let trapped = request(2, 37_000_001, FundingReplyMode::Trap);
    let observation = fixture
        .fund(fixture.driver, trapped)
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
    let attempts = fixture.attempts();
    assert_eq!(
        attempts.last(),
        Some(&FundingAttemptRecord {
            request: trapped,
            observation: Some(observation),
        })
    );
    assert_eq!(fixture.receipts(), receipts);
    fixture.upgrade_both();
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    let status = fixture.status_unchanged();
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
        fixture.fund(fixture.driver, trapped),
        Err(FundingFailure::Fenced)
    );
    // The receiver's trapped receipt remains absent, and neither side resumes.
    let next = request(3, 19_000_003, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts(), receipts);
}

#[test]
fn lifetime_journal_capacity_survives_upgrade_without_forgetting_payments() {
    let fixture = Fixture::new();
    let mut candidate = request(0, 1, FundingReplyMode::Success);
    loop {
        // Test-run cap, deliberately independent of the fixture's capacity constant.
        assert!(
            candidate.id < 100,
            "experiment must reach its bounded journal limit"
        );
        match fixture.fund(fixture.driver, candidate) {
            Ok(_) => candidate.id += 1,
            Err(FundingFailure::Limit) => break,
            other => panic!("unexpected admission: {other:?}"),
        }
    }
    let attempts = fixture.attempts();
    let receipts = fixture.receipts();
    assert!(!receipts.is_empty());
    let status = fixture.status_unchanged();
    fixture.upgrade_both();
    assert_eq!(fixture.status_unchanged(), fenced_status(status));
    assert_eq!(
        fixture.fund(fixture.driver, candidate),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    assert_eq!(
        fixture.fund(fixture.driver, attempts[0].request),
        Err(FundingFailure::Fenced)
    );
}

#[test]
fn enqueue_failure_has_no_callback_refund_and_preserves_exact_unsent_history() {
    let fixture = Fixture::new();
    let prior = request(1, 13_000_001, FundingReplyMode::Success);
    fixture
        .fund(fixture.driver, prior)
        .expect("prior refunded call");
    let receipts = fixture.receipts();
    let mut unsent = request(2, 1, FundingReplyMode::Success);
    unsent.offered = fixture.harness.pic.cycle_balance(fixture.sender) + 1;
    let observation = fixture
        .fund(fixture.driver, unsent)
        .expect("local enqueue failure observed");
    assert_eq!(
        observation,
        FundingObservation {
            refunded: None,
            transport_accepted: Some(0),
            outcome: FundingOutcome::NotEnqueued,
            reconciliation: FundingReconciliationView::NoTransfer,
        }
    );
    let attempts = fixture.attempts();
    assert_eq!(
        attempts.last(),
        Some(&FundingAttemptRecord {
            request: unsent,
            observation: Some(observation),
        })
    );
    assert_eq!(fixture.receipts(), receipts);
    fixture.upgrade_both();
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    // Changed parameters do not turn the already admitted identity into a retry.
    let status = fixture.status_unchanged();
    assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
    assert_eq!(
        status.attempts.last(),
        Some(&FundingAttemptStatusView {
            id: unsent.id,
            offered: unsent.offered,
            refunded: None,
            transport_accepted: Some(0),
            outcome: Some(FundingOutcome::NotEnqueued),
            provider_credit: None,
            reconciliation: FundingReconciliationView::NoTransfer,
        })
    );
    assert_eq!(status.attempts.first().unwrap().provider_credit, None);
    let changed = request(unsent.id, 1, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, changed),
        Err(FundingFailure::Fenced)
    );
    let next = request(3, 19_000_003, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(fixture.driver, next),
        Err(FundingFailure::Fenced)
    );
    assert_eq!(fixture.receipts(), receipts);
}

#[test]
fn denied_and_invalid_requests_cannot_offer_cycles_or_inspect_journals() {
    let fixture = Fixture::new();
    let mut request = request(1, 1, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(Principal::anonymous(), request),
        Err(FundingFailure::Denied)
    );
    for (offered, accept) in [(0, 0), (u128::MAX, 0), (10, 11)] {
        request.offered = offered;
        request.accept = accept;
        assert_eq!(
            fixture.fund(fixture.driver, request),
            Err(FundingFailure::Limit)
        );
    }
    assert!(fixture.attempts().is_empty());
    assert!(fixture.receipts().is_empty());
    let status = fixture.status_unchanged();
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
            Fake::principal(99),
            fixture.sender,
            fixture.receiver,
        ] {
            assert_eq!(fixture.status_as(canister, caller), None);
        }
    }
    let hidden: Option<Vec<FundingAttemptRecord>> = fixture
        .harness
        .pic
        .query_candid_as(fixture.sender, Principal::anonymous(), "attempts", ())
        .expect("private journal query");
    assert_eq!(hidden, None);
    let rejected = fixture
        .harness
        .pic
        .update_call(
            fixture.receiver,
            fixture.driver,
            "receive",
            candid::encode_one(request).expect("request"),
        )
        .expect_err("driver cannot impersonate peer");
    assert_eq!(rejected.reject_code, RejectCode::CanisterError);
    assert!(fixture.receipts().is_empty());
    fixture.upgrade_both();
    assert_eq!(fixture.status_unchanged(), fenced_status(status));
    assert_eq!(
        fixture.status_as(fixture.sender, Principal::anonymous()),
        None
    );
}
