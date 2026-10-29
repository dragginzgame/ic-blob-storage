use super::*;
use crate::test_support::exposure;
use crate::workflow::uploads::{admission, manifests};
use ic_blob_storage_contracts::{
    dto::upload::{
        UploadState, admission::UploadAdmissionFailure as A, exposure::UploadExposureFailure as F,
    },
    upload::binding::UploadPermission,
};

pub(super) fn permission() -> UploadPermission {
    ic_blob_storage_contracts::upload::admission::parse(
        context(4),
        manifest_boundary::input().permission,
    )
    .unwrap()
}
pub(super) fn prepared(
    m: UploadMemories<ic_memory::RuntimeMemory<ic_memory::VectorMemory>>,
) -> StableUploads<ic_memory::VectorMemory> {
    let mut store = StableUploads::install(m, config()).unwrap();
    enroll(&mut store);
    let input = manifest_boundary::input();
    admission::admit(&mut store, context(4), input.permission, 1).unwrap();
    manifests::prepare(&mut store, context(5), &input, 2).unwrap();
    store
}
#[test]
fn exposure_binds_actual_actor_original_permission_and_commits_once() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let p = manifest_boundary::input().permission;
    let usage = store.usage().unwrap();
    for actor in [2, 4, 6] {
        assert_eq!(
            exposure::commit(&mut store, context(actor), p, 2),
            Err(F::Permission(A::Denied))
        );
    }
    let mut changed = p;
    changed.expires_at_ns += 1;
    assert_eq!(
        exposure::commit(&mut store, context(5), changed, 2),
        Err(F::Permission(A::Conflict))
    );
    assert_eq!(store.usage().unwrap(), usage);
    exposure::commit(&mut store, context(5), p, 2).unwrap();
    let result = exposure::inspect(&store, context(5), p).unwrap();
    assert_eq!(result.permission, p);
    assert_eq!(result.state, UploadState::Registered);
    assert_eq!(
        store.usage().unwrap(),
        crate::model::catalog::admission::UploadUsage {
            active_reservations: 0,
            reserved_bytes: 0,
            ..usage
        }
    );
    assert_eq!(
        exposure::commit(&mut store, context(5), p, 2),
        Err(F::Phase)
    );
}
#[test]
fn exposure_preserves_expiry_revocation_activation_and_restore_fences() {
    let m = memory();
    let mut store = prepared(clone_memory(&m));
    let p = manifest_boundary::input().permission;
    assert_eq!(
        exposure::commit(&mut store, context(5), p, 100),
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
        exposure::commit(&mut store, context(5), p, 2),
        Err(F::Permission(A::Inactive))
    );
    store.revoke(context(4), permission().request).unwrap();
    assert_eq!(
        exposure::commit(&mut store, context(5), p, 2),
        Err(F::Revoked)
    );
    let cancelled = exposure::inspect(&store, context(5), p).unwrap();
    assert!(cancelled.revoked);
    assert_eq!(cancelled.state, UploadState::Cancelled);
    drop(store);
    let mut restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(exposure::inspect(&restored, context(4), p), Ok(cancelled));
    assert_eq!(
        exposure::commit(&mut restored, context(5), p, 2),
        Err(F::Permission(A::Fenced))
    );
}
#[test]
fn exposure_requires_preparation_and_retains_uncertainty_after_withdrawal() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    enroll(&mut store);
    let input = manifest_boundary::input();
    admission::admit(&mut store, context(4), input.permission, 1).unwrap();
    assert_eq!(
        exposure::commit(&mut store, context(5), input.permission, 2),
        Err(F::Unprepared)
    );
    manifests::prepare(&mut store, context(5), &input, 2).unwrap();
    exposure::commit(&mut store, context(5), input.permission, 2).unwrap();
    let usage = store.usage().unwrap();
    store.revoke(context(4), permission().request).unwrap();
    assert_eq!(store.usage().unwrap(), usage);
    let retained = exposure::inspect(&store, context(5), input.permission).unwrap();
    assert!(retained.revoked);
    assert_eq!(retained.state, UploadState::Registered);
    assert_eq!(
        exposure::inspect(&store, context(6), input.permission),
        Err(F::Permission(A::Denied))
    );
    drop(store);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(
        exposure::inspect(&restored, context(5), input.permission),
        Ok(retained)
    );
}
