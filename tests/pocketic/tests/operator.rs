//! Typed local qualification; simulated callers never qualify a production provider.
#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
mod operator_balance;
mod operator_funding_lookup;
mod operator_funding_preview;
mod operator_gateways;
mod operator_method_mode;
mod operator_relationship;
mod operator_sync;
use ic_blob_storage_pocketic_tests::installation;
mod support;
use ic_blob_storage_pocketic_tests::service_calls::canister_rejection;
mod sync_request;

use blob_test_protocol::funding::{
    FundingFailure, FundingObservation, FundingReplyMode, FundingRequest, FundingUpgradeArgs,
};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::fake_principal;
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    authority: Principal,
    gateway: Principal,
    sender: Principal,
    receiver: Principal,
    driver: Principal,
}

impl Fixture {
    fn new() -> Self {
        Self::with_source_operator(false)
    }

    fn with_source_operator(source_operator: bool) -> Self {
        let harness = Harness::new();
        let pic = &harness.pic;
        let create = || installation::funded_canister(pic, 2_000_000_000_000);
        let authority = create();
        let gateway = create();
        let sender = create();
        let receiver = create();
        let driver = fake_principal(11);
        installation::install_fixture(
            pic,
            authority,
            "BLOB_AUTHORITY_PROBE_WASM",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (
                    fake_principal(1),
                    fake_principal(2),
                    gateway,
                    if source_operator { gateway } else { driver },
                ),
            )
            .unwrap(),
            None,
        );
        installation::install_fixture(
            pic,
            gateway,
            "BLOB_GATEWAY_SOURCE_WASM",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (authority, gateway, driver),
            )
            .unwrap(),
            None,
        );
        let funding = std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap();
        for (canister, peer) in [(sender, receiver), (receiver, sender)] {
            pic.install_canister(
                canister,
                funding.clone(),
                blob_test_protocol::fixture_wire::encode_args(
                    "fixture_init",
                    (
                        peer,
                        driver,
                        blob_test_protocol::funding::budget::FundingBudgetInput {
                            operating_reserve: 1_000_000_000,
                            other_liabilities: 0,
                            allocated: 100_000_000_000_000,
                            reserve: 1_000_000_000_000,
                        },
                    ),
                )
                .unwrap(),
                None,
            )
            .unwrap();
        }
        Self {
            harness,
            authority,
            gateway,
            sender,
            receiver,
            driver,
        }
    }

    fn journals(&self) -> Vec<Vec<u8>> {
        [self.authority, self.gateway, self.sender, self.receiver]
            .into_iter()
            .map(|id| self.harness.pic.get_stable_memory(id).unwrap())
            .collect()
    }
    fn status(&self, caller: Principal) -> Option<blob_test_protocol::status::OperatorStatusView> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.authority,
            caller,
            "operator_status",
            (),
        )
        .map(|(reply,)| reply)
        .unwrap()
    }
    fn funding_status(&self) -> blob_test_protocol::funding::FundingOperatorStatusView {
        let before = self.journals();
        let (reply,): (Option<blob_test_protocol::funding::FundingOperatorStatusView>,) =
            fixture_calls::query_fixture(
                &self.harness.pic,
                self.sender,
                self.driver,
                "operator_status",
                (),
            )
            .unwrap();
        assert_eq!(self.journals(), before, "inspection preserves all journals");
        reply.unwrap()
    }
}

#[test]
fn status_preserves_journals_unknowns_and_restoration_fences() {
    use blob_test_protocol::{authority::ArchiveCatalog, status::OperatorBlockerView};
    let f = Fixture::new();
    let before = f.journals();
    let report = f.status(f.driver).unwrap();
    assert_eq!(report.service, f.authority);
    assert_eq!(report.namespace, 1);
    assert_eq!(report.provider_balance, None);
    assert_eq!(report.funding_activity, None);
    assert!(
        report
            .catalogs
            .iter()
            .any(|c| c.catalog == ArchiveCatalog::Samples && c.physical == 300)
    );
    assert!(
        report
            .catalogs
            .iter()
            .any(|c| c.catalog == ArchiveCatalog::Journey && c.physical == 0)
    );
    assert_eq!(f.status(f.driver).unwrap(), report);
    assert_eq!(f.status(Principal::anonymous()), None);
    assert_eq!(f.journals(), before);
    f.upgrade_fixture(f.authority, false);
    let frozen = f.journals();
    let report = f.status(f.driver).unwrap();
    assert!(report.fenced);
    assert!(
        report
            .blockers
            .contains(&OperatorBlockerView::RecoveryFenced)
    );
    assert_eq!(f.journals(), frozen);
}

#[test]
fn funding_status_preserves_transport_facts_and_provider_failures_after_restore() {
    use blob_test_protocol::funding::FundingProviderErrorView;
    use blob_test_protocol::funding::{FundingOutcome, FundingReconciliationView};
    use blob_test_protocol::status::FundingActivityView;
    let f = Fixture::new();
    for (index, (reply, expected)) in [
        (FundingReplyMode::Success, FundingOutcome::ReportedSuccess),
        (
            FundingReplyMode::InternalError,
            FundingOutcome::ProviderError(FundingProviderErrorView::InternalError),
        ),
        (
            FundingReplyMode::AccountBalanceOverflow,
            FundingOutcome::ProviderError(FundingProviderErrorView::AccountBalanceOverflow),
        ),
        (
            FundingReplyMode::TopUpWithoutCycles,
            FundingOutcome::ProviderError(FundingProviderErrorView::TopUpWithoutCycles),
        ),
        (
            FundingReplyMode::NotAuthorized,
            FundingOutcome::ProviderError(FundingProviderErrorView::NotAuthorized(
                Principal::anonymous(),
            )),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let id = if index == 0 { u64::MAX } else { index as u64 };
        let (result,): (Result<FundingObservation, FundingFailure>,) =
            fixture_calls::update_fixture(
                &f.harness.pic,
                f.sender,
                f.driver,
                "fund",
                (FundingRequest {
                    id,
                    offered: 1_000_000_000,
                    accept: 400_000_000,
                    reply,
                    trap_callback: false,
                },),
            )
            .unwrap();
        let observation = result.unwrap();
        let report = f.funding_status();
        let attempt = report.attempts[index];
        assert_eq!(attempt.id, id);
        assert_eq!(attempt.transport_accepted, Some(400_000_000));
        assert_eq!(attempt.refunded, Some(600_000_000));
        assert_eq!(attempt.provider_credit, None);
        assert_eq!(observation.outcome, expected);
        assert_eq!(attempt.outcome, Some(expected));
        assert_eq!(
            attempt.reconciliation,
            FundingReconciliationView::CreditRequired(400_000_000)
        );
        assert_eq!(report.funding_activity, FundingActivityView::Uncertain);
    }
    let before = f.funding_status();
    operator_funding_lookup::upgrade(&f);
    let after = f.funding_status();
    assert!(after.fenced);
    assert_eq!(after.attempts, before.attempts);
    assert_eq!(after.budget, before.budget);
}
