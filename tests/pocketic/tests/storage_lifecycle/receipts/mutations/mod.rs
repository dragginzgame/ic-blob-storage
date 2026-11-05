use super::*;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
async fn mutate(
    f: &Fixture,
    input: &ReferenceClientInput,
) -> Result<ReferenceMutationResponse, ReferenceProbeFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "fixture_mutate_reference",
            (input,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn committed_mutation_with_unusable_reply_recovers_exact_receipt_and_cleanup_at_capacity() {
    let f = consumer().await;
    let permission = f.exposed().await;
    let retain = reference(permission.request, u128::MAX, 2, true);
    let selected = selection(&f, retain);
    // The service commits, but the client cannot use its reply. No automatic retry.
    assert_eq!(
        mutate(
            &f,
            &ReferenceClientInput {
                max_reply_bytes: 1,
                ..selected
            }
        )
        .await,
        Err(ReferenceProbeFailure::Limit)
    );
    assert_eq!(f.live(retain).await, Ok(true));
    let expected = changed(retain, false);
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Found(expected.receipt))
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(mutate(&f, &selected).await, Ok(changed(retain, true)));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    let conflicting = ReferenceClientInput {
        request: ReferenceCommand {
            action: ReferenceAction::Release,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        mutate(&f, &conflicting).await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    let absent = selection(&f, reference(permission.request, 4, 9, false));
    assert_eq!(
        mutate(&f, &absent).await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Capacity))
    );
    assert_eq!(fetch(&f, &absent).await, Ok(ReferenceReceiptLookup::Absent));
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(
        mutate(
            &f,
            &selection(&f, reference(permission.request, 4, 3, true))
        )
        .await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Inactive))
    );
    for (operation, id) in [(2, 1), (3, 2)] {
        let release = reference(permission.request, operation, id, false);
        assert_eq!(
            mutate(&f, &selection(&f, release)).await,
            Ok(changed(release, false))
        );
    }
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
    assert_eq!(mutate(&f, &selected).await, Ok(changed(retain, true)));
    assert_eq!(f.live(retain).await, Ok(false));
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
    assert_eq!(
        mutate(&f, &selected).await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Fenced))
    );
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Found(expected.receipt))
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replicated_mutation_preserves_recorded_failure_and_does_not_treat_it_as_release_success() {
    let f = consumer().await;
    let permission = f.exposed().await;
    let unknown = reference(permission.request, 1, 9, false);
    let selected = selection(&f, unknown);
    let result = mutate(&f, &selected).await.unwrap();
    assert!(!result.replayed);
    assert_eq!(
        result.receipt.result,
        Err(ReferenceTransitionFailure::UnknownReference)
    );
    assert_eq!(result.receipt.request, selected.request);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        mutate(&f, &selected).await,
        Ok(ReferenceMutationResponse {
            replayed: true,
            ..result
        })
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    let release = reference(permission.request, 2, 1, false);
    assert_eq!(
        mutate(&f, &selection(&f, release)).await,
        Ok(changed(release, false))
    );
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Found(result.receipt))
    );
    assert_eq!(f.live(release).await, Ok(false));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mutation_rejects_foreign_callers_invalid_intent_and_nonreplicated_execution_without_writes()
 {
    let f = consumer().await;
    let permission = f.exposed().await;
    let retain = reference(permission.request, 1, 2, true);
    let selected = selection(&f, retain);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.controller, f.operator, f.uploader, f.other] {
        let denied: Result<ReferenceMutationResponse, ReferenceFailure> = f
            .harness
            .pic
            .update_blob_as(
                f.service,
                actor,
                REFERENCE_APPLY_METHOD,
                selected.request,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(denied, Err(ReferenceFailure::Denied));
    }
    assert_eq!(
        mutate(
            &f,
            &ReferenceClientInput {
                tenant: f.other.into(),
                ..selected
            }
        )
        .await,
        Err(ReferenceProbeFailure::Binding)
    );
    let invalid = ReferenceClientInput {
        request: ReferenceCommand {
            operation: 0,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        mutate(&f, &invalid).await,
        Err(ReferenceProbeFailure::Invalid)
    );
    let wrong_namespace = ReferenceClientInput {
        request: ReferenceCommand {
            upload: ReferenceUpload {
                namespace: 2,
                ..selected.request.upload
            },
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        mutate(&f, &wrong_namespace).await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Binding))
    );
    let query: Result<ReferenceMutationResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_control_as(
            f.tenant,
            f.operator,
            "fixture_nonreplicated_reference_mutation",
            (selected,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    assert_eq!(
        fetch(&f, &selected).await,
        Ok(ReferenceReceiptLookup::Absent)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}
