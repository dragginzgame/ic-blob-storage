use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
mod mutations;
use blob_test_protocol::storage::reference::{ReferenceClientInput, ReferenceProbeFailure};
async fn consumer() -> Fixture {
    let mut f = Fixture::new().await;
    let client = f.harness.pic.create_canister().await.unwrap();
    f.harness
        .pic
        .install_canister(
            client,
            Fixture::wasm(),
            Fixture::installation("fixture_init", f.operator),
            None,
        )
        .await
        .unwrap();
    f.tenant = client;
    f
}
async fn fetch(
    f: &Fixture,
    input: &ReferenceClientInput,
) -> Result<ReferenceReceiptLookup, ReferenceProbeFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "fixture_fetch_reference_receipt",
            (input,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
fn selection(f: &Fixture, request: ReferenceInput) -> ReferenceClientInput {
    ReferenceClientInput {
        request: receipt_request(request),
        tenant: f.tenant.into(),
        max_reply_bytes: 4096,
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replicated_receipt_preserves_historical_success_through_release_suspension_and_restore() {
    let f = consumer().await;
    let input = f.exposed().await;
    let retain = reference(input.request, 1, 2, true);
    let selected = selection(&f, retain);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Absent)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.reference(retain).await.unwrap();
    let original = ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
        request: selected.request,
        result: Ok(ReferenceChange::Changed),
    });
    assert_eq!(fetch(&f, &selected).await, Ok(original));
    f.reference(reference(input.request, 2, 2, false))
        .await
        .unwrap();
    assert_eq!(f.live(retain).await, Ok(false));
    assert_eq!(fetch(&f, &selected).await, Ok(original));
    let changed = ReferenceClientInput {
        request: ReferenceCommand {
            action: ReferenceAction::Release,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        fetch(&f, &changed).await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(fetch(&f, &selected).await, Ok(original));
    f.reference(reference(input.request, 3, 1, false))
        .await
        .unwrap();
    f.fact(input.request, ProviderFact::Deleted).await.unwrap();
    f.fact(input.request, ProviderFact::Settled).await.unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(fetch(&f, &selected).await, Ok(original));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(fetch(&f, &selected).await, Ok(original));
    assert_eq!(f.reference(retain).await, Err(ReferenceFailure::Fenced));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replicated_receipt_returns_recorded_failure_and_enforces_actual_client_and_reply_bounds() {
    let f = consumer().await;
    let input = f.exposed().await;
    let unknown = reference(input.request, 1, 2, false);
    assert_eq!(
        f.reference(unknown).await.unwrap().receipt.result,
        Err(ReferenceTransitionFailure::UnknownReference)
    );
    let selected = selection(&f, unknown);
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
            request: selected.request,
            result: Err(ReferenceTransitionFailure::UnknownReference),
        }))
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        fetch(
            &f,
            &ReferenceClientInput {
                tenant: f.operator.into(),
                ..selected
            }
        )
        .await,
        Err(ReferenceProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            &ReferenceClientInput {
                max_reply_bytes: 1,
                ..selected
            }
        )
        .await,
        Err(ReferenceProbeFailure::Limit)
    );
    let wrong_tenant = ReferenceClientInput {
        request: ReferenceCommand {
            upload: ReferenceUpload {
                tenant: f.other.into(),
                ..selected.request.upload
            },
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        fetch(&f, &wrong_tenant).await,
        Err(ReferenceProbeFailure::Binding)
    );
    let denied: Result<ReferenceReceiptLookup, ReferenceProbeFailure> = f
        .harness
        .pic
        .update_control_as(
            f.tenant,
            f.other,
            "fixture_fetch_reference_receipt",
            (selected,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(denied, Err(ReferenceProbeFailure::Denied));
    let query: Result<ReferenceReceiptLookup, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_control_as(
            f.tenant,
            f.operator,
            "fixture_nonreplicated_reference_receipt",
            (selected,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .await
        .unwrap();
    assert!(matches!(
        fetch(&f, &selected).await,
        Err(ReferenceProbeFailure::Rejected(_))
    ));
}
