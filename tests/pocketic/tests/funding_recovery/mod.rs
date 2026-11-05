//! Real stable-memory restores fence both sides of local cycle transfers.
use super::*;
use ic_testkit::pocket_ic::CanisterInstallMode;
use ic_testkit::pocket_ic::common::rest::BlobCompression;
use sha2::{Digest, Sha256};

impl Fixture {
    async fn upgrade_one(
        &self,
        canister: Principal,
        trap: bool,
    ) -> Result<(), ic_testkit::pocket_ic::ClientError> {
        self.harness
            .pic
            .upgrade_canister(
                canister,
                std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
                blob_test_protocol::fixture_wire::encode_one(
                    "fixture_post_upgrade",
                    FundingUpgradeArgs {
                        trap_after_restore: trap,
                    },
                )
                .unwrap(),
                None,
            )
            .await
    }

    async fn replace_memory(&self, canister: Principal, bytes: Vec<u8>) {
        self.harness
            .pic
            .set_stable_memory(canister, bytes, BlobCompression::NoCompression)
            .await
            .unwrap();
    }

    async fn restart(&self, canister: Principal) {
        self.harness
            .pic
            .stop_canister(canister, None)
            .await
            .unwrap();
        self.harness
            .pic
            .start_canister(canister, None)
            .await
            .unwrap();
    }

    async fn upgrade_skipping_outgoing_hook(&self, canister: Principal) {
        let pic = &self.harness.pic;
        let wasm = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap();
        let hashes = {
            let mut values = Vec::new();
            for bytes in wasm.chunks(1024 * 1024) {
                values.push(
                    pic.upload_chunk(canister, None, bytes.to_vec())
                        .await
                        .unwrap(),
                );
            }
            values
        };
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
            blob_test_protocol::fixture_wire::encode_one(
                "fixture_post_upgrade",
                FundingUpgradeArgs {
                    trap_after_restore: false,
                },
            )
            .unwrap(),
        )
        .await
        .expect("incoming hook always fences");
    }

    async fn assert_sender_fenced(&self, paid: FundingRequest) {
        let before = self.status_unchanged().await;
        assert!(before.fenced);
        for candidate in [
            paid,
            FundingRequest {
                id: u64::MAX,
                ..paid
            },
        ] {
            assert_eq!(
                self.fund(self.driver, candidate).await,
                Err(FundingFailure::Fenced)
            );
        }
        for caller in [Principal::anonymous(), self.receiver, fake_principal(99)] {
            assert_eq!(self.fund(caller, paid).await, Err(FundingFailure::Denied));
            assert_eq!(self.status_as(self.sender, caller).await, None);
        }
        assert_eq!(self.status_unchanged().await, before);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn old_empty_sender_journal_cannot_reuse_a_paid_identity_or_admit_new_work() {
    let f = Fixture::new().await;
    let old = f.harness.pic.get_stable_memory(f.sender).await.unwrap();
    let initial = f.status_unchanged().await;
    let paid = request(41, 37_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).await.unwrap();
    let receipts = f.receipts().await;
    f.replace_memory(f.sender, old).await;
    f.upgrade_skipping_outgoing_hook(f.sender).await;
    assert_eq!(f.status_unchanged().await, fenced_status(initial));
    f.assert_sender_fenced(paid).await;
    assert_eq!(f.receipts().await, receipts);
    let fenced = f.status_unchanged().await;
    for _ in 0..2 {
        f.upgrade_one(f.sender, false).await.unwrap();
        assert_eq!(f.status_unchanged().await, fenced);
        f.assert_sender_fenced(paid).await;
    }
    f.restart(f.sender).await;
    f.harness
        .pic
        .advance_time(std::time::Duration::from_hours(24))
        .await
        .unwrap();
    f.harness.pic.tick().await.unwrap();
    f.assert_sender_fenced(paid).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn restored_receiver_rejects_real_attached_cycles_before_acceptance() {
    let f = Fixture::new().await;
    let old = f.harness.pic.get_stable_memory(f.receiver).await.unwrap();
    let paid = request(1, 31_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).await.unwrap();
    let known = f.attempts().await;
    f.replace_memory(f.receiver, old).await;
    f.upgrade_one(f.receiver, false).await.unwrap();
    assert_eq!(f.receipts().await, []);
    let fenced = f.harness.pic.get_stable_memory(f.receiver).await.unwrap();
    let next = request(2, 41_000_003, FundingReplyMode::Success);
    let observation = f.fund(f.driver, next).await.unwrap();
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
        f.harness.pic.get_stable_memory(f.receiver).await.unwrap() == fenced,
        "rejected receive changed stable memory"
    );
    assert_eq!(f.attempts().await[0], known[0]);
    let status = f.status_unchanged().await;
    assert_eq!(status.funding_activity, FundingActivityView::Uncertain);
    f.upgrade_one(f.sender, false).await.unwrap();
    f.assert_sender_fenced(paid).await;
    let receiver_status = f.status_as(f.receiver, f.driver).await;
    f.upgrade_skipping_outgoing_hook(f.receiver).await;
    // ic-storage bootstrap metadata can change during a real upgrade; the bound
    // journal and its fence, not allocator bytes, must remain the same.
    assert_eq!(f.status_as(f.receiver, f.driver).await, receiver_status);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn invalid_journals_reject_upgrade_atomically_and_keep_the_active_owner_usable() {
    let f = Fixture::new().await;
    let paid = request(1, 13_000_001, FundingReplyMode::Success);
    f.fund(f.driver, paid).await.unwrap();
    let before = f.status_unchanged().await;
    let original = f.harness.pic.get_stable_memory(f.sender).await.unwrap();
    let wrong_service = f.harness.pic.get_stable_memory(f.receiver).await.unwrap();
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
        f.replace_memory(f.sender, bytes).await;
        let before_failure = f.harness.pic.get_stable_memory(f.sender).await.unwrap();
        let error = f.upgrade_one(f.sender, false).await.unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        assert!(
            f.harness.pic.get_stable_memory(f.sender).await.unwrap() == before_failure,
            "failed upgrade changed stable memory"
        );
        assert_eq!(f.status_unchanged().await, before);
    }
    f.replace_memory(f.sender, original).await;
    let next = request(2, 29_000_001, FundingReplyMode::Success);
    assert_eq!(
        f.fund(f.driver, next).await.unwrap().transport_accepted,
        Some(next.accept)
    );
    let status = f.status_unchanged().await;
    let stable = f.harness.pic.get_stable_memory(f.sender).await.unwrap();
    assert_eq!(
        crate::canister_rejection(&f.upgrade_one(f.sender, true).await.unwrap_err()).reject_code,
        RejectCode::CanisterError
    );
    assert!(
        f.harness.pic.get_stable_memory(f.sender).await.unwrap() == stable,
        "failed restore committed its fence"
    );
    assert_eq!(f.status_unchanged().await, status);
    f.upgrade_one(f.sender, false).await.unwrap();
    assert_eq!(f.status_unchanged().await, fenced_status(status));
    f.assert_sender_fenced(paid).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn captured_inflight_intent_restores_as_unknown_after_the_live_call_completes() {
    let f = Fixture::new().await;
    let paid = request(7, 31_000_001, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", paid).unwrap(),
        )
        .await
        .unwrap();
    let mut captured = None;
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        let attempts = f.attempts().await;
        if let Some(entry) = attempts.first() {
            assert!(entry.observation.is_none(), "capture before completion");
            captured = Some(f.harness.pic.get_stable_memory(f.sender).await.unwrap());
            break;
        }
    }
    let pending = captured.expect("real committed outgoing intent");
    let reply: Result<FundingObservation, FundingFailure> =
        blob_test_protocol::fixture_wire::decode_one(
            "fund",
            &f.harness.pic.await_call(call).await.unwrap(),
            blob_test_protocol::fixture_wire::MAX_BYTES,
        )
        .unwrap();
    assert_eq!(reply.unwrap().transport_accepted, Some(paid.accept));
    let receipts = f.receipts().await;
    f.replace_memory(f.sender, pending).await;
    f.upgrade_one(f.sender, false).await.unwrap();
    f.assert_unknown_status(paid).await;
    f.assert_sender_fenced(paid).await;
    assert_eq!(f.receipts().await, receipts);
    f.upgrade_one(f.receiver, false).await.unwrap();
    assert_eq!(f.receipts().await, receipts);
    f.assert_sender_fenced(paid).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upgrade_with_a_live_callback_cannot_complete_or_replay_the_restored_intent() {
    let f = Fixture::new().await;
    let paid = request(9, 29_000_003, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", paid).unwrap(),
        )
        .await
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        if !f.attempts().await.is_empty() {
            break;
        }
    }
    f.assert_unknown_status(paid).await;
    f.upgrade_one(f.sender, false).await.unwrap();
    let restored = f.assert_unknown_status(paid).await;
    assert!(restored.fenced);
    let error = f
        .harness
        .pic
        .await_call(call)
        .await
        .expect_err("restored callback cannot complete");
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.status_unchanged().await, restored);
    assert_eq!(
        f.receipts().await,
        vec![FundingReceiptRecord {
            id: paid.id,
            available: paid.offered,
            accepted: paid.accept,
        }]
    );
    f.assert_sender_fenced(paid).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn receiver_upgrade_after_acceptance_preserves_spent_cycles_and_exact_refund() {
    let f = Fixture::new().await;
    let paid = request(11, 43_000_007, FundingReplyMode::DelayedSuccess);
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", paid).unwrap(),
        )
        .await
        .unwrap();
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        if !f.receipts().await.is_empty() {
            break;
        }
    }
    let receipts = vec![FundingReceiptRecord {
        id: paid.id,
        available: paid.offered,
        accepted: paid.accept,
    }];
    assert_eq!(f.receipts().await, receipts);
    f.assert_unknown_status(paid).await;
    f.upgrade_one(f.receiver, false).await.unwrap();
    let reply: Result<FundingObservation, FundingFailure> =
        blob_test_protocol::fixture_wire::decode_one(
            "fund",
            &f.harness.pic.await_call(call).await.unwrap(),
            blob_test_protocol::fixture_wire::MAX_BYTES,
        )
        .unwrap();
    assert_eq!(
        reply.unwrap(),
        FundingObservation {
            refunded: Some(paid.offered - paid.accept),
            transport_accepted: Some(paid.accept),
            outcome: FundingOutcome::Rejected(RejectCode::CanisterError as u32),
            reconciliation: FundingReconciliationView::CreditRequired(paid.accept),
        }
    );
    assert_eq!(f.receipts().await, receipts);
    assert!(f.status_as(f.receiver, f.driver).await.unwrap().fenced);
    assert_eq!(
        f.status_unchanged().await.funding_activity,
        FundingActivityView::Uncertain
    );
    f.upgrade_one(f.sender, false).await.unwrap();
    f.assert_sender_fenced(paid).await;
}
