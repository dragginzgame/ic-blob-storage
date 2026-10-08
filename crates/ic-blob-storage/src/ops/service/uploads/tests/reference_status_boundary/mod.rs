use super::*;
use crate::workflow::references::status::inspect;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure as F;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;

fn request() -> ReferenceStatusRequest {
    ReferenceStatusRequest {
        upload: ReferenceUpload {
            service: p(1),
            tenant: p(4),
            namespace: 1,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: u128::MAX - 2,
            first_reference: u128::MAX - 3,
            root: *built().hashes().provider_root.as_bytes(),
            bytes: 10,
        },
        reference: u128::MAX - 3,
    }
}
fn reserved() -> (
    StableUploads<VectorMemory>,
    UploadMemories<VectorMemory>,
    UploadPermission,
) {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    enroll(&mut store);
    let input = request();
    let mut permission = permission(1);
    permission.request =
        ic_blob_storage_contracts::reference::parse_upload(context(4), input.upload).unwrap();
    store.admit(context(4), permission, 1).unwrap();
    (store, m, permission)
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One matrix checks each independent original upload binding"
)]
fn reference_status_boundary_rejects_invalid_changed_and_unconfirmed_uploads() {
    let (store, _, _) = reserved();
    let input = request();
    assert_eq!(inspect(&store, context(4), input), Err(F::Unconfirmed));
    assert_eq!(inspect(&store, context(2), input), Err(F::Denied));
    assert_eq!(
        inspect(
            &store,
            context(4),
            ReferenceStatusRequest {
                reference: 0,
                ..input
            }
        ),
        Err(F::Invalid)
    );
    for (upload, failure) in [
        (
            ReferenceUpload {
                service: p(9),
                ..input.upload
            },
            F::Binding,
        ),
        (
            ReferenceUpload {
                namespace: 2,
                ..input.upload
            },
            F::Binding,
        ),
        (
            ReferenceUpload {
                upload: 1,
                ..input.upload
            },
            F::Unknown,
        ),
        (
            ReferenceUpload {
                bytes: 0,
                ..input.upload
            },
            F::Invalid,
        ),
        (
            ReferenceUpload {
                object: 0,
                ..input.upload
            },
            F::Invalid,
        ),
        (
            ReferenceUpload {
                incarnation: 0,
                ..input.upload
            },
            F::Invalid,
        ),
        (
            ReferenceUpload {
                first_reference: 0,
                ..input.upload
            },
            F::Invalid,
        ),
        (
            ReferenceUpload {
                bytes: 11,
                ..input.upload
            },
            F::Conflict,
        ),
        (
            ReferenceUpload {
                object: 1,
                ..input.upload
            },
            F::Conflict,
        ),
        (
            ReferenceUpload {
                incarnation: 1,
                ..input.upload
            },
            F::Conflict,
        ),
        (
            ReferenceUpload {
                first_reference: 1,
                ..input.upload
            },
            F::Conflict,
        ),
        (
            ReferenceUpload {
                root: [9; 32],
                ..input.upload
            },
            F::Conflict,
        ),
    ] {
        assert_eq!(
            inspect(
                &store,
                context(4),
                ReferenceStatusRequest { upload, ..input }
            ),
            Err(failure)
        );
    }
}
#[test]
fn reference_status_boundary_preserves_liveness_through_suspension_release_settlement_and_restore()
{
    let (mut store, memory, permission) = reserved();
    let built = built();
    store
        .prepare_manifest(
            context(5),
            permission.request,
            UploadManifest {
                chunks: built.manifest().chunks(),
                headers: &HEADERS,
            },
            2,
        )
        .unwrap();
    store.expose(context(5), permission.request, 3).unwrap();
    store.confirm_upload(permission.request).unwrap();
    let input = request();
    let mut expected = ReferenceStatusResponse {
        request: input,
        live: true,
        fenced: false,
    };
    assert_eq!(inspect(&store, context(4), input), Ok(expected));
    assert!(
        !inspect(
            &store,
            context(4),
            ReferenceStatusRequest {
                reference: 1,
                ..input
            }
        )
        .unwrap()
        .live
    );
    let enrollment = store.tenant(context(4), p(4)).unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: enrollment,
                active: false,
            },
        )
        .unwrap();
    assert_eq!(inspect(&store, context(4), input), Ok(expected));
    store
        .apply_reference(
            context(4),
            permission.request,
            ReferenceRequest {
                id: ReferenceRequestId::new(NonZeroU128::MIN),
                operation: ReferenceOperation::Release(permission.request.object.first),
            },
        )
        .unwrap();
    expected.live = false;
    assert_eq!(inspect(&store, context(4), input), Ok(expected));
    store.confirm_provider_deleted(permission.request).unwrap();
    store.confirm_billing_stopped(permission.request).unwrap();
    assert_eq!(inspect(&store, context(4), input), Ok(expected));
    drop(store);
    let restored = StableUploads::open(memory, config()).unwrap();
    expected.fenced = true;
    assert_eq!(inspect(&restored, context(4), input), Ok(expected));
    assert_eq!(
        candid::decode_one::<ReferenceStatusResponse>(&candid::encode_one(expected).unwrap())
            .unwrap(),
        expected
    );
}
