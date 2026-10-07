//! Actual local IC dispatch; credit evidence is a labelled synthetic host input.
use super::*;
mod renewal;
use blob_test_protocol::storage::funding::CreditCommand;

pub(super) fn retain_packet(label: &str, bytes: &[u8]) {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    if let Some(root) = std::env::var_os("BLOB_FUNDING_CREDIT_REPORT") {
        let path = std::path::PathBuf::from(root).join(format!(
            "{:04}-{label}.candid",
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
    fn confirm_credit(
        &self,
        actor: Principal,
        input: CreditCommand,
    ) -> Result<Option<bool>, Failure> {
        let request = candid::encode_one(input).unwrap();
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
            .unwrap();
        retain_packet("credit-reply", &reply);
        candid::decode_one(&reply).unwrap()
    }
}
fn retain(f: &Fixture, name: &str) {
    if let Some(root) = std::env::var_os("BLOB_FUNDING_CREDIT_REPORT") {
        let dir = std::path::PathBuf::from(root).join(name);
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(
            dir.join("stable.bin"),
            f.harness.pic.get_stable_memory(f.service),
        )
        .unwrap();
        std::fs::write(
            dir.join("observations.txt"),
            format!(
                "local_status={:#?}\nallocation={:#?}\nrequests={:#?}\n",
                f.local_status(),
                f.funding_allocation(),
                f.incoming(),
            ),
        )
        .unwrap();
    }
}
#[test]
fn confirmed_credit_allows_another_dispatch_without_replenishing_spent_cycles() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(first))).accepted,
        300
    );
    let next = f.funding_intent(2, 400);
    f.funding(f.operator, next, Action::Prepare).unwrap();
    assert!(
        refused(f.guarded_dispatch(f.operator, request(next)))
            .0
            .contains(&B::JournalUncredited)
    );
    let before = f.funding_allocation();
    assert_eq!(
        f.confirm_credit(
            f.operator,
            CreditCommand {
                established: false,
                ..credit(first, 300, 1)
            }
        ),
        Ok(None)
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Ok(Some(true))
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Ok(Some(false))
    );
    assert_eq!(f.funding_allocation(), before);
    assert_eq!(f.local_status().funding.uncredited_accepted, 0);
    let outcome = f.retained_outcome(f.operator, first).unwrap().unwrap();
    assert_eq!(
        outcome.reconciliation,
        FundingReconciliationView::CreditConfirmed {
            accepted_cycles: 300,
            receipt_digest: [1; 32],
        }
    );
    f.configure_cashier(next, 200, FundingReplyMode::Success);
    assert_eq!(
        settled(f.guarded_dispatch(f.operator, request(next))).accepted,
        200
    );
    assert_eq!(
        f.incoming().iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(f.funding_allocation().accepted, 500);
    assert_eq!(f.funding_allocation().available, 500);
    assert_eq!(f.local_status().funding.uncredited_accepted, 200);
    assert_eq!(
        f.confirm_credit(f.operator, credit(next, 200, 2)),
        Ok(Some(true))
    );
    assert_eq!(f.local_status().funding.transport_accepted, 500);
    assert_eq!(f.local_status().funding.uncredited_accepted, 0);
    retain(&f, "repeat");
}
#[test]
fn credit_accounting_trap_rolls_back_receipt_and_allows_only_confirmation_replay() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)));
    let before = f.harness.pic.get_stable_memory(f.service);
    let input = candid::encode_one(CreditCommand {
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
        .unwrap_err();
    retain_packet("credit-trap-result", format!("{error:?}").as_bytes());
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(
        f.retained_outcome(f.operator, first)
            .unwrap()
            .unwrap()
            .reconciliation,
        FundingReconciliationView::CreditRequired(300)
    );
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Ok(Some(true))
    );
    assert_eq!(f.incoming().len(), 1);
    retain(&f, "rollback");
}
#[test]
fn confirmation_refuses_wrong_authority_identity_amount_and_receipt_reuse_without_writes() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)));
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
        let before = f.harness.pic.get_stable_memory(f.service);
        assert_eq!(f.confirm_credit(actor, input), Err(failure));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
    f.confirm_credit(f.operator, credit(first, 300, 1)).unwrap();
    let next = f.funding_intent(2, 400);
    f.configure_cashier(next, 200, FundingReplyMode::Success);
    f.funding(f.operator, next, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(next)));
    for input in [credit(first, 300, 2), credit(next, 200, 1)] {
        let before = f.harness.pic.get_stable_memory(f.service);
        assert_eq!(f.confirm_credit(f.operator, input), Err(Failure::Conflict));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    }
    assert_eq!(f.local_status().funding.uncredited_accepted, 200);
    retain(&f, "refusals");
}
#[test]
fn same_release_restoration_preserves_credit_receipts_and_fences_even_exact_replay() {
    let f = Fixture::with_cashier();
    let first = f.funding_intent(1, 400);
    f.configure_cashier(first, 300, FundingReplyMode::Success);
    f.funding(f.operator, first, Action::Prepare).unwrap();
    settled(f.guarded_dispatch(f.operator, request(first)));
    f.confirm_credit(f.operator, credit(first, 300, 1)).unwrap();
    let original = f.retained_outcome(f.operator, first).unwrap().unwrap();
    let allocation = f.funding_allocation();
    f.upgrade_storage();
    assert_eq!(
        f.retained_outcome(f.operator, first),
        Ok(Some(FundingOutcomeResponse {
            fenced: true,
            ..original
        }))
    );
    assert_eq!(
        f.funding_allocation(),
        Allocation {
            fenced: true,
            ..allocation
        }
    );
    assert_eq!(f.local_status().funding.uncredited_accepted, 0);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.confirm_credit(f.operator, credit(first, 300, 1)),
        Err(Failure::Fenced)
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(f.incoming().len(), 1);
    retain(&f, "restore");
}
