//! Real stable-memory restores fence both sides of local cycle transfers.
use super::*;
use ic_testkit::pocket_ic::{CanisterInstallMode, RejectResponse, common::rest::BlobCompression};
use sha2::{Digest, Sha256};

impl Fixture {
    fn upgrade_one(&self, canister: Principal, trap: bool) -> Result<(), RejectResponse> {
        self.harness.pic.upgrade_canister(
            canister,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            candid::encode_one(FundingUpgradeArgs {
                trap_after_restore: trap,
            })
            .unwrap(),
            None,
        )
    }

    fn replace_memory(&self, canister: Principal, bytes: Vec<u8>) {
        self.harness
            .pic
            .set_stable_memory(canister, bytes, BlobCompression::NoCompression);
    }

    fn restart(&self, canister: Principal) {
        self.harness.pic.stop_canister(canister, None).unwrap();
        self.harness.pic.start_canister(canister, None).unwrap();
    }

    fn upgrade_skipping_outgoing_hook(&self, canister: Principal) {
        let pic = &self.harness.pic;
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap();
        let hashes = wasm
            .chunks(1024 * 1024)
            .map(|bytes| pic.upload_chunk(canister, None, bytes.to_vec()).unwrap())
            .collect();
        #[expect(
            clippy::default_trait_access,
            reason = "PocketIC does not reexport UpgradeFlags"
        )]
        let mut mode = CanisterInstallMode::Upgrade(Some(Default::default()));
        let CanisterInstallMode::Upgrade(Some(flags)) = &mut mode else {
            unreachable!()
        };
        flags.skip_pre_upgrade = Some(true);
        pic.install_chunked_canister(
            canister,
            None,
            mode,
            canister,
            hashes,
            Sha256::digest(&wasm).to_vec(),
            candid::encode_one(FundingUpgradeArgs {
                trap_after_restore: false,
            })
            .unwrap(),
        )
        .expect("incoming hook always fences");
    }

    fn assert_sender_fenced(&self, paid: FundingRequest) {
        let before = self.status_unchanged();
        assert!(before.fenced);
        for candidate in [
            paid,
            FundingRequest {
                id: u64::MAX,
                ..paid
            },
        ] {
            assert_eq!(
                self.fund(self.driver, candidate),
                Err(FundingFailure::Fenced)
            );
        }
        for caller in [Principal::anonymous(), self.receiver, Fake::principal(99)] {
            assert_eq!(self.fund(caller, paid), Err(FundingFailure::Denied));
            assert_eq!(self.status_as(self.sender, caller), None);
        }
        assert_eq!(self.status_unchanged(), before);
    }
}

#[test]
fn old_empty_sender_journal_cannot_reuse_a_paid_identity_or_admit_new_work() {
    let f = Fixture::new();
    let old = f.harness.pic.get_stable_memory(f.sender);
    let initial = f.status_unchanged();
    let paid = request(41, 37_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).unwrap();
    let receipts = f.receipts();
    f.replace_memory(f.sender, old);
    f.upgrade_skipping_outgoing_hook(f.sender);
    assert_eq!(f.status_unchanged(), fenced_status(initial));
    f.assert_sender_fenced(paid);
    assert_eq!(f.receipts(), receipts);
    let fenced = f.status_unchanged();
    for _ in 0..2 {
        f.upgrade_one(f.sender, false).unwrap();
        assert_eq!(f.status_unchanged(), fenced);
        f.assert_sender_fenced(paid);
    }
    f.restart(f.sender);
    f.harness
        .pic
        .advance_time(std::time::Duration::from_hours(24));
    f.harness.pic.tick();
    f.assert_sender_fenced(paid);
}

#[test]
fn restored_receiver_rejects_real_attached_cycles_before_acceptance() {
    let f = Fixture::new();
    let old = f.harness.pic.get_stable_memory(f.receiver);
    let paid = request(1, 31_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).unwrap();
    let known = f.attempts();
    f.replace_memory(f.receiver, old);
    f.upgrade_one(f.receiver, false).unwrap();
    assert!(f.receipts().is_empty());
    let fenced = f.harness.pic.get_stable_memory(f.receiver);
    let next = request(2, 41_000_003, FundingReplyMode::Success);
    let observation = f.fund(f.driver, next).unwrap();
    assert_eq!(
        observation,
        FundingObservation {
            refunded: Some(next.offered),
            transport_accepted: Some(0),
            outcome: FundingOutcome::Rejected(RejectCode::CanisterError as u32),
            reconciliation: FundingReconciliationView::NoTransfer,
        }
    );
    assert!(
        f.harness.pic.get_stable_memory(f.receiver) == fenced,
        "rejected receive changed stable memory"
    );
    assert_eq!(f.attempts()[0], known[0]);
    let status = f.status_unchanged();
    assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
    f.upgrade_one(f.sender, false).unwrap();
    f.assert_sender_fenced(paid);
    let receiver_status = f.status_as(f.receiver, f.driver);
    f.upgrade_skipping_outgoing_hook(f.receiver);
    // ic-memory bootstrap metadata can change during a real upgrade; the bound
    // journal and its fence, not allocator bytes, must remain the same.
    assert_eq!(f.status_as(f.receiver, f.driver), receiver_status);
}

#[test]
fn invalid_journals_reject_upgrade_atomically_and_keep_the_active_owner_usable() {
    let f = Fixture::new();
    let paid = request(1, 13_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).unwrap();
    let before = f.status_unchanged();
    let original = f.harness.pic.get_stable_memory(f.sender);
    let wrong_service = f.harness.pic.get_stable_memory(f.receiver);
    let digest = Sha256::digest(include_bytes!("../../../../Cargo.lock"));
    let mut wrong_release = original.clone();
    let locations: Vec<_> = original
        .windows(digest.len())
        .enumerate()
        .filter_map(|(index, bytes)| (bytes == digest.as_slice()).then_some(index))
        .collect();
    let [index] = locations.as_slice() else {
        panic!("one exact release binding")
    };
    wrong_release[*index] ^= 1;
    for bytes in [vec![], vec![0xff; 65_536], wrong_service, wrong_release] {
        f.replace_memory(f.sender, bytes);
        let before_failure = f.harness.pic.get_stable_memory(f.sender);
        let error = f.upgrade_one(f.sender, false).unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert!(
            f.harness.pic.get_stable_memory(f.sender) == before_failure,
            "failed upgrade changed stable memory"
        );
        assert_eq!(f.status_unchanged(), before);
    }
    f.replace_memory(f.sender, original);
    let next = request(2, 29_000_001, FundingReplyMode::Success);
    assert_eq!(
        f.fund(f.driver, next).unwrap().transport_accepted,
        Some(next.accept)
    );
    let status = f.status_unchanged();
    let stable = f.harness.pic.get_stable_memory(f.sender);
    assert_eq!(
        f.upgrade_one(f.sender, true).unwrap_err().reject_code,
        RejectCode::CanisterError
    );
    assert!(
        f.harness.pic.get_stable_memory(f.sender) == stable,
        "failed restore committed its fence"
    );
    assert_eq!(f.status_unchanged(), status);
    f.upgrade_one(f.sender, false).unwrap();
    assert_eq!(f.status_unchanged(), fenced_status(status));
    f.assert_sender_fenced(paid);
}

#[test]
fn captured_inflight_intent_restores_as_unknown_after_the_live_call_completes() {
    let f = Fixture::new();
    let paid = request(7, 31_000_001, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(paid).unwrap(),
        )
        .unwrap();
    let mut captured = None;
    for _ in 0..20 {
        f.harness.pic.tick();
        let attempts = f.attempts();
        if let Some(entry) = attempts.first() {
            assert!(entry.observation.is_none(), "capture before completion");
            captured = Some(f.harness.pic.get_stable_memory(f.sender));
            break;
        }
    }
    let pending = captured.expect("real committed outgoing intent");
    let reply: Result<FundingObservation, FundingFailure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(reply.unwrap().transport_accepted, Some(paid.accept));
    let receipts = f.receipts();
    f.replace_memory(f.sender, pending);
    f.upgrade_one(f.sender, false).unwrap();
    f.assert_unknown_status(paid);
    f.assert_sender_fenced(paid);
    assert_eq!(f.receipts(), receipts);
    f.upgrade_one(f.receiver, false).unwrap();
    assert_eq!(f.receipts(), receipts);
    f.assert_sender_fenced(paid);
}

#[test]
fn upgrade_with_a_live_callback_cannot_complete_or_replay_the_restored_intent() {
    let f = Fixture::new();
    let paid = request(9, 29_000_003, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(paid).unwrap(),
        )
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick();
        if !f.attempts().is_empty() {
            break;
        }
    }
    f.assert_unknown_status(paid);
    f.upgrade_one(f.sender, false).unwrap();
    let restored = f.assert_unknown_status(paid);
    assert!(restored.fenced);
    let error = f
        .harness
        .pic
        .await_call(call)
        .expect_err("restored callback cannot complete");
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.status_unchanged(), restored);
    assert_eq!(
        f.receipts(),
        vec![FundingReceiptRecord {
            id: paid.id,
            available: paid.offered,
            accepted: paid.accept,
        }]
    );
    f.assert_sender_fenced(paid);
}

#[test]
fn receiver_upgrade_after_acceptance_preserves_spent_cycles_and_exact_refund() {
    let f = Fixture::new();
    let paid = request(11, 43_000_007, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            candid::encode_one(paid).unwrap(),
        )
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick();
        if !f.receipts().is_empty() {
            break;
        }
    }
    let receipts = vec![FundingReceiptRecord {
        id: paid.id,
        available: paid.offered,
        accepted: paid.accept,
    }];
    assert_eq!(f.receipts(), receipts);
    f.assert_unknown_status(paid);
    f.upgrade_one(f.receiver, false).unwrap();
    let reply: Result<FundingObservation, FundingFailure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(
        reply.unwrap(),
        FundingObservation {
            refunded: Some(paid.offered - paid.accept),
            transport_accepted: Some(paid.accept),
            outcome: FundingOutcome::Rejected(RejectCode::CanisterError as u32),
            reconciliation: FundingReconciliationView::CreditRequired(paid.accept),
        }
    );
    assert_eq!(f.receipts(), receipts);
    assert!(f.status_as(f.receiver, f.driver).unwrap().fenced);
    assert_eq!(
        f.status_unchanged().funding_activity,
        FundingActivityView::Uncertain
    );
    f.upgrade_one(f.sender, false).unwrap();
    f.assert_sender_fenced(paid);
}
