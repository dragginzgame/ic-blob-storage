//! Actual local IC dispatch; credit evidence is a labelled synthetic host input.
use super::*;
mod renewal;
use blob_test_protocol::storage::funding::CreditCommand;

pub(super) fn retain_packet(label: &str, bytes: &[u8]) {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    if let Some(root) = std::env::var_os("BLOB_FUNDING_CREDIT_REPORT") {
        let path = std::path::PathBuf::from(root).join(format!(
            "{:04}-{label}.cbor",
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap();
        std::io::Write::write_all(&mut file, bytes).unwrap();
    }
}

fn credit(intent: Intent, accepted: u128, digest: u8) -> CreditCommand {
    CreditCommand {
        intent,
        accepted,
        receipt_digest: [digest; 32],
        established: true,
        fault: None,
    }
}
impl Fixture {
    async fn confirm_credit(
        &self,
        actor: Principal,
        input: CreditCommand,
    ) -> Result<Option<bool>, Failure> {
        let request =
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(input).unwrap();
        retain_packet("credit-request", &request);
        let reply = self
            .harness
            .pic
            .update_call(
                self.service,
                actor,
                "fixture_confirm_funding_credit",
                request,
            )
            .await
            .unwrap();
        retain_packet("credit-reply", &reply);
        ic_blob_storage_pocketic_tests::control_calls::decode_control_one(&reply, 4 * 1024 * 1024)
            .unwrap()
    }
}
async fn retain(f: &Fixture, name: &str) {
    if let Some(root) = std::env::var_os("BLOB_FUNDING_CREDIT_REPORT") {
        let dir = std::path::PathBuf::from(root).join(name);
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(
            dir.join("stable.bin"),
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        )
        .unwrap();
        std::fs::write(
            dir.join("observations.txt"),
            format!(
                "local_status={:#?}\nallocation={:#?}\nrequests={:#?}\n",
                f.local_status().await,
                f.funding_allocation().await,
                f.incoming().await,
            ),
        )
        .unwrap();
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn confirmed_credit_allows_another_dispatch_without_replenishing_spent_cycles() {
    let f = Fixture::with_cashier().await;
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Malformed)
        .await;
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(first)).await).accepted,
        300
    );
    let next = f.funding_intent(2, 400);
    f.funding(f.operator, next, Action::Prepare).await.unwrap();
    assert!(
        refused(f.guarded_dispatch(f.operator, request(next)).await)
            .0
            .contains(&B::JournalUncredited)
    );
    let before = f.funding_allocation().await;
    assert_eq!(
        f.confirm_credit(
            f.operator,
            CreditCommand {
                established: false,
                ..credit(first, 300, 1)
            }
        )
        .await,
        Ok(None)
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)).await,
        Ok(Some(true))
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)).await,
        Ok(Some(false))
    );
    assert_eq!(f.funding_allocation().await, before);
    assert_eq!(f.local_status().await.funding.uncredited_accepted, 0);
    let outcome = f
        .retained_outcome(f.operator, first)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        outcome.reconciliation,
        FundingReconciliationView::CreditConfirmed {
            accepted_cycles: 300,
            receipt_digest: [1; 32],
        }
    );
    f.configure_cashier(next, 200, FundingReplyMode::Malformed)
        .await;
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(next)).await).accepted,
        200
    );
    assert_eq!(
        f.incoming().await.iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(f.funding_allocation().await.accepted, 500);
    assert_eq!(f.funding_allocation().await.available, 500);
    assert_eq!(f.local_status().await.funding.uncredited_accepted, 200);
    assert_eq!(
        f.confirm_credit(f.operator, credit(next, 200, 2)).await,
        Ok(Some(true))
    );
    assert_eq!(f.local_status().await.funding.transport_accepted, 500);
    assert_eq!(f.local_status().await.funding.uncredited_accepted, 0);
    retain(&f, "repeat").await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn credit_accounting_trap_rolls_back_receipt_and_allows_only_confirmation_replay() {
    let f = Fixture::with_cashier().await;
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Malformed)
        .await;
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)).await);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let input = ic_blob_storage_pocketic_tests::control_calls::encode_control_one(CreditCommand {
        fault: Some(WriteFault::FundingAccounting),
        ..credit(first, 300, 1)
    })
    .unwrap();
    retain_packet("credit-trap-request", &input);
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_confirm_funding_credit",
            input,
        )
        .await
        .unwrap_err();
    retain_packet("credit-trap-result", format!("{error:?}").as_bytes());
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        f.retained_outcome(f.operator, first)
            .await
            .unwrap()
            .unwrap()
            .reconciliation,
        FundingReconciliationView::CreditRequired(300)
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)).await,
        Ok(Some(true))
    );
    assert_eq!(f.incoming().await.len(), 1);
    retain(&f, "rollback").await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn confirmation_refuses_wrong_authority_identity_amount_and_receipt_reuse_without_writes() {
    let f = Fixture::with_cashier().await;
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Malformed)
        .await;
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)).await);
    for (actor, input, failure) in [
        (f.tenant, credit(first, 300, 1), Failure::Denied),
        (
            f.operator,
            credit(
                Intent {
                    namespace: 2,
                    ..first
                },
                300,
                1,
            ),
            Failure::Binding,
        ),
        (
            f.operator,
            credit(
                Intent {
                    offered: 399,
                    ..first
                },
                300,
                1,
            ),
            Failure::Conflict,
        ),
        (f.operator, credit(first, 299, 1), Failure::Invalid),
        (f.operator, credit(first, 300, 0), Failure::Invalid),
    ] {
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        assert_eq!(f.confirm_credit(actor, input).await, Err(failure));
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    }
    f.confirm_credit(f.operator, credit(first, 300, 1))
        .await
        .unwrap();
    let next = f.funding_intent(2, 400);
    f.configure_cashier(next, 200, FundingReplyMode::Malformed)
        .await;
    f.funding(f.operator, next, Action::Prepare).await.unwrap();
    settled(f.guarded_dispatch(f.operator, request(next)).await);
    for input in [credit(first, 300, 2), credit(next, 200, 1)] {
        let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
        assert_eq!(
            f.confirm_credit(f.operator, input).await,
            Err(Failure::Conflict)
        );
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            before
        );
    }
    assert_eq!(f.local_status().await.funding.uncredited_accepted, 200);
    retain(&f, "refusals").await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn same_release_restoration_preserves_credit_receipts_and_fences_even_exact_replay() {
    let f = Fixture::with_cashier().await;
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Malformed)
        .await;
    f.funding(f.operator, first, Action::Prepare).await.unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)).await);
    f.confirm_credit(f.operator, credit(first, 300, 1))
        .await
        .unwrap();
    let original = f
        .retained_outcome(f.operator, first)
        .await
        .unwrap()
        .unwrap();
    let allocation = f.funding_allocation().await;
    f.upgrade_storage().await;
    assert_eq!(
        f.retained_outcome(f.operator, first).await,
        Ok(Some(FundingOutcomeResponse {
            fenced: true,
            ..original
        }))
    );
    assert_eq!(
        f.funding_allocation().await,
        Allocation {
            fenced: true,
            ..allocation
        }
    );
    assert_eq!(f.local_status().await.funding.uncredited_accepted, 0);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)).await,
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(f.incoming().await.len(), 1);
    retain(&f, "restore").await;
}
