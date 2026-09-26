//! Real cycle acceptance/refund and callback rollback with a local substitute.
//! Same-release stable journal recovery; no deployed economics or old-backup safety.

#![cfg(not(target_family = "wasm"))]

mod support;

use blob_test_protocol::funding::{
    FundingAttemptRecord, FundingFailure, FundingObservation, FundingOutcome, FundingReceiptRecord,
    FundingReplyMode, FundingRequest, FundingUpgradeArgs,
};
use candid::Principal;
use ic_testkit::{Fake, pic::CandidCallExt, pocket_ic::RejectCode};
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
        let sender = pic.create_canister();
        let receiver = pic.create_canister();
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).expect("funding Wasm");
        for (canister, peer) in [(sender, receiver), (receiver, sender)] {
            pic.add_cycles(canister, 2_000_000_000_000);
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
    for (index, (accept, reply, outcome)) in cases.into_iter().enumerate() {
        let request = request(u64::try_from(index).expect("small index"), accept, reply);
        let observation = fixture.fund(fixture.driver, request).expect("admitted");
        assert_eq!(
            observation,
            FundingObservation {
                refunded: Some(request.offered - accept),
                transport_accepted: Some(accept),
                outcome,
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
        let receipts = fixture.receipts();
        assert_eq!(
            fixture.fund(fixture.driver, request),
            Err(FundingFailure::AlreadyAdmitted)
        );
        assert_eq!(fixture.receipts(), receipts);
    }
    let attempts = fixture.attempts();
    let receipts = fixture.receipts();
    fixture.upgrade_both();
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    // Neither changing the amount nor changing the reply mode makes an old identity fresh.
    for entry in &attempts {
        let mut changed = entry.request;
        changed.accept = 0;
        changed.reply = FundingReplyMode::Malformed;
        assert_eq!(
            fixture.fund(fixture.driver, changed),
            Err(FundingFailure::AlreadyAdmitted)
        );
    }
    let next = request(99, 13_000_001, FundingReplyMode::Success);
    let observation = fixture
        .fund(fixture.driver, next)
        .expect("new fixture experiment after upgrade");
    assert_eq!(observation.transport_accepted, Some(next.accept));
    assert_eq!(observation.refunded, Some(next.offered - next.accept));
    assert_eq!(&fixture.attempts()[..attempts.len()], attempts);
    assert_eq!(&fixture.receipts()[..receipts.len()], receipts);
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
        Err(FundingFailure::AlreadyAdmitted)
    );
    request.id += 1;
    request.trap_callback = false;
    assert_eq!(
        fixture.fund(fixture.driver, request),
        Err(FundingFailure::InProgress)
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
    assert_eq!(
        fixture.fund(fixture.driver, trapped),
        Err(FundingFailure::AlreadyAdmitted)
    );
    // A later independent experiment can proceed; the trapped receipt stays absent.
    let next = request(3, 19_000_003, FundingReplyMode::Success);
    assert_eq!(
        fixture
            .fund(fixture.driver, next)
            .expect("later payment")
            .transport_accepted,
        Some(next.accept)
    );
    let mut expected = receipts;
    expected.push(FundingReceiptRecord {
        id: next.id,
        available: next.offered,
        accepted: next.accept,
    });
    assert_eq!(fixture.receipts(), expected);
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
    fixture.upgrade_both();
    assert_eq!(
        fixture.fund(fixture.driver, candidate),
        Err(FundingFailure::Limit)
    );
    assert_eq!(fixture.attempts(), attempts);
    assert_eq!(fixture.receipts(), receipts);
    assert_eq!(
        fixture.fund(fixture.driver, attempts[0].request),
        Err(FundingFailure::AlreadyAdmitted)
    );
}

#[test]
fn denied_and_invalid_requests_cannot_offer_cycles_or_inspect_journals() {
    let fixture = Fixture::new();
    let mut request = request(1, 1, FundingReplyMode::Success);
    assert_eq!(
        fixture.fund(Principal::anonymous(), request),
        Err(FundingFailure::Denied)
    );
    for (offered, accept) in [(0, 0), (1_000_000_001, 0), (10, 11)] {
        request.offered = offered;
        request.accept = accept;
        assert_eq!(
            fixture.fund(fixture.driver, request),
            Err(FundingFailure::Limit)
        );
    }
    assert!(fixture.attempts().is_empty());
    assert!(fixture.receipts().is_empty());
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
}
