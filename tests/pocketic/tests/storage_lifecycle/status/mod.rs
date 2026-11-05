//! The shared query distinguishes current reference state from retained mutation receipts.
use super::*;
use blob_test_protocol::storage::reference::{ReferenceProbeFailure, ReferenceStatusClientInput};
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

fn request(input: ReferenceInput) -> ReferenceStatusRequest {
    let command = receipt_request(input);
    ReferenceStatusRequest {
        upload: command.upload,
        reference: command.reference,
    }
}
async fn inspect(
    f: &Fixture,
    actor: Principal,
    request: ReferenceStatusRequest,
) -> Result<ReferenceStatusResponse, ReferenceFailure> {
    f.harness
        .pic
        .query_blob_as(
            f.service,
            actor,
            "blob_reference_status",
            request,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn reference_status_keeps_history_distinct_from_live_references_through_settlement() {
    let f = Fixture::new().await;
    let permission = f.exposed().await;
    let retain = reference(permission.request, 1, u128::MAX, true);
    f.reference(retain).await.unwrap();
    let expected = f.receipt(retain).await.unwrap();
    let query = request(retain);
    let active = inspect(&f, f.tenant, query).await.unwrap();
    assert_eq!(
        active,
        ReferenceStatusResponse {
            request: query,
            live: true,
            fenced: false
        }
    );
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(inspect(&f, f.tenant, query).await, Ok(active));
    let release = reference(permission.request, 2, u128::MAX, false);
    f.reference(release).await.unwrap();
    assert!(!inspect(&f, f.tenant, query).await.unwrap().live);
    assert_eq!(f.receipt(retain).await, Ok(expected));
    assert!(
        inspect(
            &f,
            f.tenant,
            request(reference(permission.request, 3, 1, false))
        )
        .await
        .unwrap()
        .live
    );
    let unknown = ReferenceStatusRequest {
        reference: u128::MAX - 1,
        ..query
    };
    assert!(!inspect(&f, f.tenant, unknown).await.unwrap().live);
    for actor in [f.controller, f.operator, f.uploader, Principal::anonymous()] {
        assert_eq!(
            inspect(&f, actor, query).await,
            Err(ReferenceFailure::Denied)
        );
    }
    f.reference(reference(permission.request, 3, 1, false))
        .await
        .unwrap();
    f.fact(permission.request, ProviderFact::Deleted)
        .await
        .unwrap();
    f.fact(permission.request, ProviderFact::Settled)
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let retired = ReferenceStatusResponse {
        live: false,
        ..active
    };
    assert_eq!(inspect(&f, f.tenant, query).await, Ok(retired));
    let replicated: Result<ReferenceStatusResponse, ReferenceFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.tenant,
            "blob_reference_status",
            query,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(replicated, Ok(retired));
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
    assert_eq!(
        inspect(&f, f.tenant, query).await,
        Ok(ReferenceStatusResponse {
            fenced: true,
            ..retired
        })
    );
    assert_eq!(f.receipt(retain).await, Ok(expected));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One actual canister client follows reply refusals and both restores"
)]
async fn reference_status_client_authenticates_exact_replies_and_preserves_live_restore_fences() {
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
    let permission = f.exposed().await;
    let input = ReferenceStatusClientInput {
        request: request(reference(permission.request, 1, 1, false)),
        tenant: client.into(),
        max_reply_bytes: 4096,
    };
    let fetch = async |input: ReferenceStatusClientInput| -> Result<ReferenceStatusResponse, ReferenceProbeFailure> {
        f.harness.pic.update_control_as( client, f.operator, "fixture_fetch_reference_status", (input,), REPLY_BYTES).await.map(|(reply,)| reply).unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let client_before = f.harness.pic.get_stable_memory(client).await.unwrap();
    let expected = ReferenceStatusResponse {
        request: input.request,
        live: true,
        fenced: false,
    };
    assert_eq!(fetch(input).await, Ok(expected));
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            max_reply_bytes: 1,
            ..input
        })
        .await,
        Err(ReferenceProbeFailure::Limit)
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            tenant: f.operator.into(),
            ..input
        })
        .await,
        Err(ReferenceProbeFailure::Binding)
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            request: ReferenceStatusRequest {
                upload: ReferenceUpload {
                    bytes: 11,
                    ..input.request.upload
                },
                ..input.request
            },
            ..input
        })
        .await,
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    assert_eq!(
        fetch(ReferenceStatusClientInput {
            request: ReferenceStatusRequest {
                reference: 0,
                ..input.request
            },
            ..input
        })
        .await,
        Err(ReferenceProbeFailure::Invalid)
    );
    let query: Result<ReferenceStatusResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_control_as(
            client,
            f.operator,
            "fixture_nonreplicated_reference_status",
            (input,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    let denied: Result<ReferenceStatusResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .update_control_as(
            client,
            f.other,
            "fixture_fetch_reference_status",
            (input,),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(denied, Err(ReferenceProbeFailure::Denied));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(client).await.unwrap(),
        client_before
    );
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    assert_eq!(fetch(input).await, Ok(expected));
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
    let restored = ReferenceStatusResponse {
        fenced: true,
        ..expected
    };
    assert_eq!(fetch(input).await, Ok(restored));
    f.harness
        .pic
        .upgrade_canister(
            client,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            None,
        )
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let client_before = f.harness.pic.get_stable_memory(client).await.unwrap();
    assert_eq!(fetch(input).await, Ok(restored));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(client).await.unwrap(),
        client_before
    );
}
