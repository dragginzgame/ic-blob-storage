use super::*;
use crate::policy::upload::exposure::UploadExposureBlocker as B;
use crate::policy::upload::exposure::UploadExposureHostEvidence;
use crate::policy::upload::exposure::assess_exposure;
use crate::workflow::uploads::admission;
use crate::workflow::uploads::exposure;
use crate::workflow::uploads::exposure::UploadExposureResult;
use crate::workflow::uploads::manifests;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as A;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure as F;
pub(super) fn evidence() -> UploadExposureHostEvidence {
    let permission = ic_blob_storage_contracts::upload::admission::parse(
        context(4),
        manifest_boundary::input().permission,
    )
    .unwrap();
    UploadExposureHostEvidence {
        permission,
        observed_at_ns: 2,
        namespace_binding: true,
        trusted_uploader: true,
        current_owner: true,
        durable_commit: true,
    }
}
pub(super) fn prepared(m: UploadMemories<VectorMemory>) -> StableUploads<VectorMemory> {
    let mut store = StableUploads::install(m, config()).unwrap();
    enroll(&mut store);
    let input = manifest_boundary::input();
    admission::admit(&mut store, context(4), input.permission, 1).unwrap();
    manifests::prepare(&mut store, context(5), &input, 2).unwrap();
    store
}
#[test]
fn exposure_policy_keeps_each_missing_host_fact_independent() {
    let yes = evidence();
    assert_eq!(assess_exposure(yes, 2).blockers, []);
    for (changed, blocker) in [
        (
            UploadExposureHostEvidence {
                observed_at_ns: 1,
                ..yes
            },
            B::StaleObservation,
        ),
        (
            UploadExposureHostEvidence {
                observed_at_ns: 3,
                ..yes
            },
            B::StaleObservation,
        ),
        (
            UploadExposureHostEvidence {
                namespace_binding: false,
                ..yes
            },
            B::NamespaceBinding,
        ),
        (
            UploadExposureHostEvidence {
                trusted_uploader: false,
                ..yes
            },
            B::TrustedUploader,
        ),
        (
            UploadExposureHostEvidence {
                current_owner: false,
                ..yes
            },
            B::CurrentOwner,
        ),
        (
            UploadExposureHostEvidence {
                durable_commit: false,
                ..yes
            },
            B::Durability,
        ),
    ] {
        assert_eq!(assess_exposure(changed, 2).blockers, vec![blocker]);
    }
    assert_eq!(
        assess_exposure(
            UploadExposureHostEvidence {
                observed_at_ns: 1,
                namespace_binding: false,
                trusted_uploader: false,
                current_owner: false,
                durable_commit: false,
                ..yes
            },
            2
        )
        .blockers,
        vec![
            B::StaleObservation,
            B::NamespaceBinding,
            B::TrustedUploader,
            B::CurrentOwner,
            B::Durability
        ]
    );
}
#[test]
fn exposure_binds_complete_permission_and_current_evidence_before_any_write() {
    let mut store = prepared(memory());
    let p = manifest_boundary::input().permission;
    let yes = evidence();
    let usage = store.usage().unwrap();
    for actor in [2, 4, 6] {
        assert_eq!(
            exposure::commit(&mut store, context(actor), p, yes, 2),
            Err(F::Permission(A::Denied))
        );
    }
    let mut changed = p;
    changed.expires_at_ns += 1;
    assert_eq!(
        exposure::commit(&mut store, context(5), changed, yes, 2),
        Err(F::Permission(A::Conflict))
    );
    let mut foreign = yes;
    foreign.permission.expires_at_ns += 1;
    assert_eq!(
        exposure::commit(&mut store, context(5), p, foreign, 2),
        Err(F::EvidenceBinding)
    );
    for missing in [
        UploadExposureHostEvidence {
            durable_commit: false,
            ..yes
        },
        UploadExposureHostEvidence {
            current_owner: false,
            ..yes
        },
    ] {
        let assessment = exposure::inspect_preparation(&store, context(5), p, missing, 2).unwrap();
        assert_eq!(
            exposure::commit(&mut store, context(5), p, missing, 2),
            Ok(UploadExposureResult::Blocked(assessment))
        );
    }
    assert_eq!(
        exposure::inspect_preparation(&store, context(5), p, yes, 2)
            .unwrap()
            .blockers,
        []
    );
    assert_eq!(
        exposure::commit(&mut store, context(5), p, yes, 3),
        Ok(UploadExposureResult::Blocked(assess_exposure(yes, 3)))
    );
    assert_eq!(
        exposure::inspect(&store, context(5), p).unwrap().state,
        UploadState::Reserved
    );
    assert_eq!(store.usage().unwrap(), usage);
    let UploadExposureResult::Exposed(result) =
        exposure::commit(&mut store, context(5), p, yes, 2).unwrap()
    else {
        panic!("qualified local exposure")
    };
    assert_eq!(result.permission, p);
    assert_eq!(result.state, UploadState::ExposurePossible);
    assert_eq!(store.usage().unwrap(), usage);
    assert_eq!(
        exposure::commit(&mut store, context(5), p, yes, 2),
        Err(F::Phase)
    );
}
#[test]
fn exposure_preview_never_overrides_revocation_activation_expiry_or_restore() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let p = manifest_boundary::input().permission;
    let yes = evidence();
    assert_eq!(
        exposure::inspect_preparation(&store, context(5), p, yes, 2)
            .unwrap()
            .blockers,
        []
    );
    assert_eq!(
        exposure::commit(
            &mut store,
            context(5),
            p,
            UploadExposureHostEvidence {
                observed_at_ns: 100,
                ..yes
            },
            100
        ),
        Err(F::Permission(A::Expired))
    );
    let enrolled = store.tenant(context(2), p.upload.tenant).unwrap().unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p.upload.tenant,
                expected: Some(enrolled),
                active: false,
            },
        )
        .unwrap();
    assert_eq!(
        exposure::commit(&mut store, context(5), p, yes, 2),
        Err(F::Permission(A::Inactive))
    );
    store.revoke(context(4), yes.permission.request).unwrap();
    assert_eq!(
        exposure::commit(&mut store, context(5), p, yes, 2),
        Err(F::Revoked)
    );
    let cancelled = exposure::inspect(&store, context(5), p).unwrap();
    assert!(cancelled.revoked);
    assert_eq!(cancelled.state, UploadState::Cancelled);
    drop(store);
    let mut restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(exposure::inspect(&restored, context(4), p), Ok(cancelled));
    assert_eq!(
        exposure::commit(&mut restored, context(5), p, yes, 2),
        Err(F::Permission(A::Fenced))
    );
}
#[test]
fn exposure_requires_preparation_and_retains_charged_uncertainty_after_withdrawal() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    enroll(&mut store);
    let input = manifest_boundary::input();
    let yes = evidence();
    admission::admit(&mut store, context(4), input.permission, 1).unwrap();
    assert_eq!(
        exposure::commit(&mut store, context(5), input.permission, yes, 2),
        Err(F::Unprepared)
    );
    manifests::prepare(&mut store, context(5), &input, 2).unwrap();
    exposure::commit(&mut store, context(5), input.permission, yes, 2).unwrap();
    let usage = store.usage().unwrap();
    store.revoke(context(4), yes.permission.request).unwrap();
    assert_eq!(store.usage().unwrap(), usage);
    let retained = exposure::inspect(&store, context(5), input.permission).unwrap();
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::ExposurePossible);
    assert_eq!(
        exposure::inspect(&store, context(6), input.permission),
        Err(F::Permission(A::Denied))
    );
    drop(store);
    let restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(
        exposure::inspect(&restored, context(5), input.permission),
        Ok(retained)
    );
}
