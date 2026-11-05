//! Actual query-only recovery of local evidence, not provider credit reconciliation.
use super::*;
use blob_test_protocol::funding::lookup::{
    FundingLookupFailure, FundingLookupRequest, FundingLookupState, FundingLookupView,
};
use ic_blob_storage_pocketic_tests::fixture_calls;

use ic_testkit::pocket_ic::common::rest::BlobCompression;

fn request() -> FundingRequest {
    FundingRequest {
        id: u64::MAX,
        offered: 1_000_000,
        accept: 400_000,
        reply: FundingReplyMode::InternalError,
        trap_callback: false,
    }
}
async fn lookup(
    f: &Fixture,
    input: FundingLookupRequest,
    caller: Principal,
) -> Result<FundingLookupView, FundingLookupFailure> {
    fixture_calls::query_fixture(&f.harness.pic, f.sender, caller, "lookup_funding", (input,))
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
async fn unchanged(
    f: &Fixture,
    request: FundingRequest,
) -> Result<FundingLookupView, FundingLookupFailure> {
    let before = f.journals().await;
    let result = lookup(
        f,
        FundingLookupRequest {
            service: f.sender,
            peer: f.receiver,
            attempt: request,
        },
        f.driver,
    )
    .await;
    assert_eq!(
        f.journals().await,
        before,
        "lookup cannot mutate either side of a transfer"
    );
    result
}
fn observed(view: &FundingLookupView) -> FundingObservation {
    let FundingLookupState::Observed(value) = view.state else {
        panic!("expected retained observation: {view:?}")
    };
    value
}
pub(super) async fn upgrade(f: &Fixture) {
    f.harness
        .pic
        .upgrade_canister(
            f.sender,
            std::fs::read(fixture_path("BLOB_FUNDING_PROBE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_one(
                "fixture_post_upgrade",
                FundingUpgradeArgs {
                    trap_after_restore: false,
                },
            )
            .unwrap(),
            None,
        )
        .await
        .unwrap();
}
async fn submit_and_discard_reply(f: &Fixture, request: FundingRequest) {
    // The ingress completes, but its response is deliberately not decoded or used.
    // A subsequent query must recover retained evidence without invoking fund again.
    f.harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", request).unwrap(),
        )
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn lost_ingress_result_is_recovered_exactly_and_conflicting_inputs_cannot_read_it() {
    let f = Fixture::new().await;
    let original = request();
    let absent = unchanged(&f, original).await.unwrap();
    assert_eq!(absent.state, FundingLookupState::Absent);
    submit_and_discard_reply(&f, original).await;
    let result = unchanged(&f, original).await.unwrap();
    assert_eq!(result.request.attempt.id, u64::MAX);
    let observation = observed(&result);
    assert_eq!(observation.transport_accepted, Some(400_000));
    assert_eq!(observation.refunded, Some(600_000));
    assert_eq!(
        observation.outcome,
        blob_test_protocol::funding::FundingOutcome::ProviderError(
            blob_test_protocol::funding::FundingProviderErrorView::InternalError
        )
    );
    assert_eq!(
        observation.reconciliation,
        blob_test_protocol::funding::FundingReconciliationView::CreditRequired(400_000)
    );
    assert_eq!(unchanged(&f, original).await.unwrap(), result);
    for changed in [
        FundingRequest {
            offered: 1_000_001,
            ..original
        },
        FundingRequest {
            accept: 0,
            ..original
        },
        FundingRequest {
            reply: FundingReplyMode::Success,
            ..original
        },
        FundingRequest {
            trap_callback: true,
            ..original
        },
    ] {
        assert_eq!(
            unchanged(&f, changed).await,
            Err(FundingLookupFailure::Conflict)
        );
    }
    upgrade(&f).await;
    let frozen = unchanged(&f, original).await.unwrap();
    assert!(frozen.fenced);
    assert_eq!(frozen.state, result.state);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn old_backup_absence_cannot_erase_a_receipt_or_authorize_another_transfer() {
    let f = Fixture::new().await;
    let old = f.harness.pic.get_stable_memory(f.sender).await.unwrap();
    let original = request();
    submit_and_discard_reply(&f, original).await;
    let receiver = f.harness.pic.get_stable_memory(f.receiver).await.unwrap();
    f.harness
        .pic
        .set_stable_memory(f.sender, old, BlobCompression::NoCompression)
        .await
        .unwrap();
    // Until an actual restore, diagnosis uses the active owner, not old stable bytes.
    assert!(matches!(
        unchanged(&f, original).await.unwrap().state,
        FundingLookupState::Observed(_)
    ));
    upgrade(&f).await;
    let absent = unchanged(&f, original).await.unwrap();
    assert_eq!(absent.state, FundingLookupState::Absent);
    assert!(absent.fenced);
    let result: Result<FundingObservation, FundingFailure> =
        fixture_calls::update_fixture(&f.harness.pic, f.sender, f.driver, "fund", (original,))
            .await
            .map(|(reply,)| reply)
            .unwrap();
    assert_eq!(result, Err(FundingFailure::Fenced));
    assert!(
        f.harness.pic.get_stable_memory(f.receiver).await.unwrap() == receiver,
        "lookup cannot erase the receiver receipt"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn callback_trap_remains_pending_through_lookup_and_restore() {
    let f = Fixture::new().await;
    let original = FundingRequest {
        trap_callback: true,
        ..request()
    };
    let failure = f
        .harness
        .pic
        .update_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", original).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&failure).reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    for restored in [false, true] {
        if restored {
            upgrade(&f).await;
        }
        let result = unchanged(&f, original).await.unwrap();
        assert_eq!(result.state, FundingLookupState::Pending);
        assert_eq!(result.fenced, restored);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn live_lookup_observes_pending_then_original_completion_without_repeating_it() {
    let f = Fixture::new().await;
    let original = FundingRequest {
        reply: FundingReplyMode::DelayedSuccess,
        ..request()
    };
    let query = FundingLookupRequest {
        service: f.sender,
        peer: f.receiver,
        attempt: original,
    };
    let call = f
        .harness
        .pic
        .submit_call(
            f.sender,
            f.driver,
            "fund",
            blob_test_protocol::fixture_wire::encode_one("fund", original).unwrap(),
        )
        .await
        .unwrap();
    let mut pending = false;
    for _ in 0..20 {
        f.harness.pic.tick().await.unwrap();
        let result: Result<FundingLookupView, FundingLookupFailure> = fixture_calls::query_fixture(
            &f.harness.pic,
            f.sender,
            f.driver,
            "lookup_funding",
            (query,),
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
        if result.unwrap().state == FundingLookupState::Pending {
            pending = true;
            break;
        }
    }
    assert!(pending, "observe committed intent before completion");
    let result = unchanged(&f, original).await.unwrap();
    assert_eq!(result.state, FundingLookupState::Pending);
    f.harness.pic.await_call(call).await.unwrap();
    let result = unchanged(&f, original).await.unwrap();
    assert_eq!(observed(&result).transport_accepted, Some(400_000));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lookup_denies_wrong_callers_and_bindings_and_never_falls_back_to_update() {
    let f = Fixture::new().await;
    let before = f.journals().await;
    let input = FundingLookupRequest {
        service: f.sender,
        peer: f.receiver,
        attempt: request(),
    };
    for (caller, query, failure) in [
        (Principal::anonymous(), input, FundingLookupFailure::Denied),
        (
            f.driver,
            FundingLookupRequest {
                peer: fake_principal(99),
                ..input
            },
            FundingLookupFailure::Binding,
        ),
        (
            f.driver,
            FundingLookupRequest {
                attempt: FundingRequest {
                    offered: 10_000_000_000_001,
                    ..request()
                },
                ..input
            },
            FundingLookupFailure::InvalidRequest,
        ),
    ] {
        assert_eq!(lookup(&f, query, caller).await, Err(failure));
    }
    assert!(
        f.journals().await == before,
        "failed lookups preserve every journal"
    );
    let pic = &f.harness.pic;
    let canister = pic.create_canister().await.unwrap();
    pic.install_canister(
        canister,
        super::operator_method_mode::update_wasm("lookup_funding", &[]),
        vec![],
        None,
    )
    .await
    .unwrap();
    let result: Result<(Result<FundingLookupView, FundingLookupFailure>,), _> =
        fixture_calls::query_fixture(pic, canister, f.driver, "lookup_funding", (input,)).await;
    assert!(result.is_err());
    assert_eq!(
        pic.get_stable_memory(canister).await.unwrap(),
        Vec::<u8>::new()
    );
    pic.update_call(canister, f.driver, "lookup_funding", vec![])
        .await
        .unwrap();
    assert_eq!(pic.get_stable_memory(canister).await.unwrap().len(), 65_536);
}
