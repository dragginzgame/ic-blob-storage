use super::*;
use crate::workflow::uploads::admission::inspect;
use crate::workflow::uploads::admission::revoke;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure as F;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
fn wire(p: UploadPermission) -> UploadAdmissionRequest {
    let object = p.request.object.first.object();
    let identity = object.identity();
    UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: object.service(),
            tenant: object.tenant(),
            namespace: identity.namespace.get(),
            upload: p.request.id.get().get(),
            object: identity.object.get(),
            incarnation: identity.incarnation.get(),
            first_reference: p.request.object.first.reference().get().get(),
            root: *p.request.object.root.as_bytes(),
            bytes: p.request.object.bytes,
        },
        uploader: p.uploader,
        expires_at_ns: p.expires_at_ns,
    }
}
#[test]
fn revocation_preserves_exposed_and_confirmed_obligations_and_checks_permission_before_writes() {
    for confirmed in [false, true] {
        let m = memory();
        let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
        let permission = lifecycle::exposed(&mut store);
        if confirmed {
            store.confirm_upload(permission.request).unwrap();
        }
        let input = wire(permission);
        let before = store.usage().unwrap();
        for actor in [2, 5, 6] {
            assert_eq!(revoke(&mut store, context(actor), input), Err(F::Denied));
        }
        for changed in [
            UploadAdmissionRequest {
                uploader: p(6),
                ..input
            },
            UploadAdmissionRequest {
                expires_at_ns: input.expires_at_ns + 1,
                ..input
            },
        ] {
            assert_eq!(revoke(&mut store, context(4), changed), Err(F::Conflict));
        }
        assert!(!inspect(&store, context(4), input).unwrap().revoked);
        let old = store.tenant(context(4), p(4)).unwrap();
        store
            .update_tenant(
                context(2),
                TenantUpdate {
                    tenant: p(4),
                    expected: old,
                    active: false,
                },
            )
            .unwrap();
        let revoked = revoke(&mut store, context(4), input).unwrap();
        assert!(revoked.changed && revoked.admission.revoked);
        assert_eq!(
            revoked.admission.state,
            if confirmed {
                UploadState::Confirmed
            } else {
                UploadState::ExposurePossible
            }
        );
        assert_eq!(store.usage().unwrap(), before);
        assert!(!revoke(&mut store, context(4), input).unwrap().changed);
        drop(store);
        let mut restored = StableUploads::open(m, config()).unwrap();
        assert_eq!(inspect(&restored, context(4), input), Ok(revoked.admission));
        assert_eq!(revoke(&mut restored, context(4), input), Err(F::Fenced));
    }
}
