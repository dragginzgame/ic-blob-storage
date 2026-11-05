use super::*;
use blob_test_protocol::consumer::Revocation;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse;
use ic_blob_storage_pocketic_tests::control_calls::ControlCallExt;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
pub(super) async fn withdraw(
    f: &Fixture,
    fault: Fault,
    max_reply_bytes: u32,
) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "revoke",
            Revocation {
                asset: 1,
                fault,
                max_reply_bytes,
            },
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
async fn recover_withdrawal(f: &Fixture) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_control_as(
            f.tenant,
            f.operator,
            "recover_revocation",
            1_u128,
            REPLY_BYTES,
        )
        .await
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revocation_traps_preserve_tombstone_and_recover_without_resending() {
    for fault in [Fault::AfterIntent, Fault::AfterRevocation] {
        let f = fixture().await;
        let (input, permission, _) = prepared(&f).await;
        admit(&f, &input).await.unwrap();
        assert_eq!(
            withdraw(&f, Fault::None, 4096).await,
            Err(ConsumerFailure::State)
        );
        cancel(&f, 1).await.unwrap();
        f.enroll(Some(f.tenant().await.unwrap()), false)
            .await
            .unwrap();
        let error = f
            .harness
            .pic
            .update_call(
                f.tenant,
                f.operator,
                "revoke",
                ic_blob_storage_contracts::document::encode(Revocation {
                    asset: 1,
                    fault,
                    max_reply_bytes: 4096,
                })
                .unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
        let pending = asset(&f, 1).await.unwrap();
        assert!(pending.cancelled);
        assert_eq!(pending.revocation_result, None);
        if fault == Fault::AfterIntent {
            assert!(!pending.revocation_started);
            assert!(
                !f.lookup(f.tenant, permission.request)
                    .await
                    .unwrap()
                    .revoked
            );
            assert_eq!(recover_withdrawal(&f).await, Err(ConsumerFailure::State));
            assert_eq!(
                withdraw(&f, Fault::None, 4096)
                    .await
                    .unwrap()
                    .revocation_result,
                Some(Ok(()))
            );
        } else {
            assert!(pending.revocation_started);
            assert!(
                f.lookup(f.tenant, permission.request)
                    .await
                    .unwrap()
                    .revoked
            );
            assert_eq!(
                withdraw(&f, Fault::None, 4096).await,
                Err(ConsumerFailure::Pending)
            );
            let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
            let recovered = recover_withdrawal(&f).await.unwrap();
            assert_eq!(recovered.revocation_result, Some(Ok(())));
            assert_eq!(recovered.upload_state, Some(UploadState::Cancelled));
            assert_eq!(withdraw(&f, Fault::None, 4096).await, Ok(recovered));
            assert_eq!(
                f.harness.pic.get_stable_memory(f.service).await.unwrap(),
                before
            );
        }
        assert_eq!(
            f.status().await.usage,
            JourneyUsage {
                logical: 0,
                physical: 0,
                liability: 0
            }
        );
        assert_eq!(release(&f, 1).await, Err(ConsumerFailure::State));
        assert!(!register(&f, &input).await.unwrap().published);
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unusable_revocation_reply_keeps_exposed_bytes_charged_and_late_completion_releasable() {
    let f = fixture().await;
    let (input, permission, manifest) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    f.prepare(&manifest).await.unwrap();
    f.expose(permission.request).await.unwrap();
    cancel(&f, 1).await.unwrap();
    assert_eq!(
        withdraw(&f, Fault::None, 1).await,
        Err(ConsumerFailure::Transport)
    );
    assert_eq!(
        withdraw(&f, Fault::None, 4096).await,
        Err(ConsumerFailure::Pending)
    );
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let known = recover_withdrawal(&f).await.unwrap();
    assert_eq!(known.revocation_result, Some(Ok(())));
    assert_eq!(known.upload_state, Some(UploadState::Registered));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 10,
            physical: 10,
            liability: 10
        }
    );
    recover(&f, 1, false).await.unwrap();
    assert_eq!(
        release(&f, 1).await.unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    assert_eq!(
        f.status().await.usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
    assert!(!register(&f, &input).await.unwrap().published);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revocation_rejects_changed_permissions_and_restored_service_without_claiming_cleanup() {
    let f = fixture().await;
    let (input, permission, _) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    let p = admission_input(permission);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.uploader, f.operator, f.controller, f.other] {
        let result: Result<UploadRevocationResponse, A> = f
            .harness
            .pic
            .update_blob_as(f.service, actor, "blob_revoke_upload", p, REPLY_BYTES)
            .await
            .unwrap();
        assert_eq!(result, Err(A::Denied));
    }
    for changed in [
        UploadAdmissionRequest {
            uploader: f.other.into(),
            ..p
        },
        UploadAdmissionRequest {
            expires_at_ns: p.expires_at_ns - 1,
            ..p
        },
    ] {
        let result: Result<UploadRevocationResponse, A> = f
            .harness
            .pic
            .update_blob_as(
                f.service,
                f.tenant,
                "blob_revoke_upload",
                changed,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        assert_eq!(result, Err(A::Conflict));
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    cancel(&f, 1).await.unwrap();
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
    let refused = withdraw(&f, Fault::None, 4096).await.unwrap();
    assert_eq!(refused.revocation_result, Some(Err(A::Fenced)));
    assert_eq!(recover_withdrawal(&f).await, Err(ConsumerFailure::Pending));
    assert_eq!(asset(&f, 1).await, Ok(refused));
    assert!(
        !f.lookup(f.tenant, permission.request)
            .await
            .unwrap()
            .revoked
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn consumer_upgrade_keeps_uncertain_withdrawal_and_blocks_dispatch_or_acknowledgment() {
    let f = fixture().await;
    let (input, _, _) = prepared(&f).await;
    admit(&f, &input).await.unwrap();
    cancel(&f, 1).await.unwrap();
    assert_eq!(
        withdraw(&f, Fault::None, 1).await,
        Err(ConsumerFailure::Transport)
    );
    let pending = asset(&f, 1).await.unwrap();
    assert!(pending.revocation_started && pending.cancelled);
    assert_eq!(pending.revocation_result, None);
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(asset(&f, 1).await, Ok(pending));
    assert_eq!(
        withdraw(&f, Fault::None, 4096).await,
        Err(ConsumerFailure::Fenced)
    );
    assert_eq!(recover_withdrawal(&f).await, Err(ConsumerFailure::Fenced));
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}
