use super::*;
use blob_test_protocol::consumer::Revocation;
use ic_blob_storage::dto::upload::admission::{
    UploadAdmissionFailure as A, UploadAdmissionRequest, UploadRevocationResponse,
};
pub(super) fn withdraw(
    f: &Fixture,
    fault: Fault,
    max_reply_bytes: u32,
) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(
            f.tenant,
            f.operator,
            "revoke",
            (Revocation {
                asset: 1,
                fault,
                max_reply_bytes,
            },),
        )
        .unwrap()
}
fn recover_withdrawal(f: &Fixture) -> Result<AssetView, ConsumerFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "recover_revocation", (1_u128,))
        .unwrap()
}
#[test]
fn revocation_traps_preserve_tombstone_and_recover_without_resending() {
    for fault in [Fault::AfterIntent, Fault::AfterRevocation] {
        let f = fixture();
        let (input, permission, _) = prepared(&f);
        admit(&f, &input).unwrap();
        assert_eq!(withdraw(&f, Fault::None, 4096), Err(ConsumerFailure::State));
        cancel(&f, 1).unwrap();
        f.enroll(Some(f.tenant().unwrap()), false).unwrap();
        let error = f
            .harness
            .pic
            .update_call(
                f.tenant,
                f.operator,
                "revoke",
                candid::encode_one(Revocation {
                    asset: 1,
                    fault,
                    max_reply_bytes: 4096,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        let pending = asset(&f, 1).unwrap();
        assert!(pending.cancelled);
        assert_eq!(pending.revocation_result, None);
        if fault == Fault::AfterIntent {
            assert!(!pending.revocation_started);
            assert!(!f.lookup(f.tenant, permission.request).unwrap().revoked);
            assert_eq!(recover_withdrawal(&f), Err(ConsumerFailure::State));
            assert_eq!(
                withdraw(&f, Fault::None, 4096).unwrap().revocation_result,
                Some(Ok(()))
            );
        } else {
            assert!(pending.revocation_started);
            assert!(f.lookup(f.tenant, permission.request).unwrap().revoked);
            assert_eq!(
                withdraw(&f, Fault::None, 4096),
                Err(ConsumerFailure::Pending)
            );
            let before = f.harness.pic.get_stable_memory(f.service);
            let recovered = recover_withdrawal(&f).unwrap();
            assert_eq!(recovered.revocation_result, Some(Ok(())));
            assert_eq!(recovered.upload_state, Some(UploadState::Cancelled));
            assert_eq!(withdraw(&f, Fault::None, 4096), Ok(recovered));
            assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        }
        assert_eq!(
            f.status().usage,
            JourneyUsage {
                logical: 0,
                physical: 0,
                liability: 0
            }
        );
        assert_eq!(release(&f, 1), Err(ConsumerFailure::State));
        assert!(!register(&f, &input).unwrap().published);
    }
}
#[test]
fn unusable_revocation_reply_keeps_exposed_bytes_charged_and_late_completion_releasable() {
    let f = fixture();
    let (input, permission, manifest) = prepared(&f);
    admit(&f, &input).unwrap();
    f.prepare(&manifest).unwrap();
    f.expose(permission.request).unwrap();
    cancel(&f, 1).unwrap();
    assert_eq!(
        withdraw(&f, Fault::None, 1),
        Err(ConsumerFailure::Transport)
    );
    assert_eq!(
        withdraw(&f, Fault::None, 4096),
        Err(ConsumerFailure::Pending)
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    let known = recover_withdrawal(&f).unwrap();
    assert_eq!(known.revocation_result, Some(Ok(())));
    assert_eq!(known.upload_state, Some(UploadState::ExposurePossible));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 10,
            physical: 10,
            liability: 10
        }
    );
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    recover(&f, 1, false).unwrap();
    assert_eq!(
        release(&f, 1).unwrap().release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
    assert!(!register(&f, &input).unwrap().published);
}
#[test]
fn revocation_rejects_changed_permissions_and_restored_service_without_claiming_cleanup() {
    let f = fixture();
    let (input, permission, _) = prepared(&f);
    admit(&f, &input).unwrap();
    let p = admission_input(permission);
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.uploader, f.operator, f.controller, f.other] {
        let result: Result<UploadRevocationResponse, A> = f
            .harness
            .pic
            .update_candid_as(f.service, actor, "blob_revoke_upload", (p,))
            .unwrap();
        assert_eq!(result, Err(A::Denied));
    }
    for changed in [
        UploadAdmissionRequest {
            uploader: f.other,
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
            .update_candid_as(f.service, f.tenant, "blob_revoke_upload", (changed,))
            .unwrap();
        assert_eq!(result, Err(A::Conflict));
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    cancel(&f, 1).unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let refused = withdraw(&f, Fault::None, 4096).unwrap();
    assert_eq!(refused.revocation_result, Some(Err(A::Fenced)));
    assert_eq!(recover_withdrawal(&f), Err(ConsumerFailure::Pending));
    assert_eq!(asset(&f, 1), Ok(refused));
    assert!(!f.lookup(f.tenant, permission.request).unwrap().revoked);
}
#[test]
fn consumer_upgrade_keeps_uncertain_withdrawal_and_blocks_dispatch_or_acknowledgment() {
    let f = fixture();
    let (input, _, _) = prepared(&f);
    admit(&f, &input).unwrap();
    cancel(&f, 1).unwrap();
    assert_eq!(
        withdraw(&f, Fault::None, 1),
        Err(ConsumerFailure::Transport)
    );
    let pending = asset(&f, 1).unwrap();
    assert!(pending.revocation_started && pending.cancelled);
    assert_eq!(pending.revocation_result, None);
    let before = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .upgrade_canister(
            f.tenant,
            consumer_wasm(),
            candid::encode_args(()).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(asset(&f, 1), Ok(pending));
    assert_eq!(
        withdraw(&f, Fault::None, 4096),
        Err(ConsumerFailure::Fenced)
    );
    assert_eq!(recover_withdrawal(&f), Err(ConsumerFailure::Fenced));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
