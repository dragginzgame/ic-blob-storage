use super::*;
use crate::workflow::references::apply;
use crate::workflow::uploads::inspect;
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::UploadState;
use ic_blob_storage_contracts::dto::upload::UploadStatusFailure;
fn wire(request: UploadRequest) -> ReferenceUpload {
    let object = request.object.first.object();
    let id = object.identity();
    ReferenceUpload {
        service: object.service(),
        tenant: object.tenant(),
        namespace: id.namespace.get(),
        upload: request.id.get().get(),
        object: id.object.get(),
        incarnation: id.incarnation.get(),
        first_reference: request.object.first.reference().get().get(),
        root: *request.object.root.as_bytes(),
        bytes: request.object.bytes,
    }
}
#[test]
fn upload_status_is_exact_tenant_history_not_liveness_or_mutation_authority() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let permission = lifecycle::exposed(&mut store);
    let upload = wire(permission.request);
    assert_eq!(
        inspect(&store, context(4), upload).unwrap().state,
        UploadState::ExposurePossible
    );
    for actor in [2, 5, 6] {
        assert_eq!(
            inspect(&store, context(actor), upload),
            Err(UploadStatusFailure::Denied)
        );
    }
    assert_eq!(
        inspect(
            &store,
            context(4),
            ReferenceUpload {
                first_reference: 2,
                ..upload
            }
        ),
        Err(UploadStatusFailure::Conflict)
    );
    store.revoke(context(4), permission.request).unwrap();
    let uncertain = inspect(&store, context(4), upload).unwrap();
    assert!(uncertain.revoked);
    assert_eq!(uncertain.state, UploadState::ExposurePossible);
    store.confirm_upload(permission.request).unwrap();
    assert_eq!(
        store
            .confirmed(context(4), permission.request)
            .unwrap()
            .receipt_slots,
        0
    );
    assert_eq!(
        inspect(&store, context(4), upload).unwrap().state,
        UploadState::Confirmed
    );
    apply(
        &mut store,
        context(4),
        ReferenceCommand {
            upload,
            reference: upload.first_reference,
            operation: 1,
            action: ReferenceAction::Release,
        },
    )
    .unwrap();
    store.confirm_provider_deleted(permission.request).unwrap();
    store.confirm_billing_stopped(permission.request).unwrap();
    let original = inspect(&store, context(4), upload).unwrap();
    assert_eq!(original.state, UploadState::Confirmed);
    drop(store);
    let restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(inspect(&restored, context(4), upload), Ok(original));
}
