use super::*;
use crate::{
    model::{
        lifecycle::requests::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
        service::read::download::CaffeineDownloadScope,
    },
    ops::service::uploads::read::download::DownloadDescriptorError,
};
fn scope(owner: Principal, namespace: NonZeroU128) -> CaffeineDownloadScope {
    CaffeineDownloadScope::new(owner, namespace, "project/&β").unwrap()
}
#[test]
fn download_descriptor_binds_service_not_payer_or_tenant_and_preserves_original_metadata() {
    let base = config();
    let config = crate::model::service::configuration::ServiceConfiguration::new(
        crate::model::service::configuration::ServiceBindings {
            payment_account: p(8),
            ..base.bindings()
        },
        base.limits(),
        base.billing(),
    )
    .unwrap();
    let mut store = StableUploads::install(memory(), config).unwrap();
    let input = lifecycle::exposed(&mut store);
    let root = input.request.object.root;
    let reference = input.request.object.first;
    let scope = scope(p(1), NonZeroU128::MIN);
    assert_eq!(
        store.download_descriptor(context(4), &scope, root, reference),
        Err(DownloadDescriptorError::Unavailable)
    );
    store.confirm_upload(input.request).unwrap();
    let before = store.usage().unwrap();
    let view = store
        .download_descriptor(context(4), &scope, root, reference)
        .unwrap();
    assert_eq!(view.reference, reference);
    assert_eq!(view.descriptor.content.request, input.request);
    assert_eq!(
        view.descriptor
            .headers
            .iter()
            .map(|h| (h.name.as_str(), h.value.as_str()))
            .collect::<Vec<_>>(),
        HEADERS
            .iter()
            .map(|h| (h.name, h.value))
            .collect::<Vec<_>>()
    );
    for owner in [p(2), p(4), p(8)] {
        let wrong = CaffeineDownloadScope::new(owner, NonZeroU128::MIN, "project/&β").unwrap();
        assert_eq!(
            store.download_descriptor(context(4), &wrong, root, reference),
            Err(DownloadDescriptorError::Binding)
        );
    }
    let wrong =
        CaffeineDownloadScope::new(p(1), NonZeroU128::new(2).unwrap(), "project/&β").unwrap();
    assert_eq!(
        store.download_descriptor(context(4), &wrong, root, reference),
        Err(DownloadDescriptorError::Binding)
    );
    assert_eq!(
        store.download_descriptor(context(2), &scope, root, reference),
        Err(DownloadDescriptorError::Store(UploadStoreError::Admission(
            UploadAdmissionError::NotProject
        )))
    );
    assert_eq!(store.usage().unwrap(), before);
}
#[test]
fn operational_download_refuses_suspension_release_and_restore_while_inspection_remains() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let input = lifecycle::exposed(&mut store);
    store.confirm_upload(input.request).unwrap();
    let root = input.request.object.root;
    let reference = input.request.object.first;
    let scope = scope(p(1), NonZeroU128::MIN);
    let before = store
        .download_descriptor(context(4), &scope, root, reference)
        .unwrap();
    let active = store.tenant(context(4), p(4)).unwrap();
    let suspended = store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: active,
                active: false,
            },
        )
        .unwrap();
    assert_eq!(
        store.download_descriptor(context(4), &scope, root, reference),
        Err(DownloadDescriptorError::Store(UploadStoreError::Admission(
            UploadAdmissionError::Tenant(crate::model::service::tenant::TenantError::Suspended)
        )))
    );
    assert_eq!(
        store
            .retained_content_descriptor(context(4), root, reference)
            .unwrap(),
        Some(before)
    );
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(suspended),
                active: true,
            },
        )
        .unwrap();
    let second = ReferenceKey::new(
        reference.object(),
        ReferenceId::new(NonZeroU128::new(2).unwrap()),
    );
    for (id, operation) in [
        (1, ReferenceOperation::Retain(second)),
        (2, ReferenceOperation::Release(reference)),
    ] {
        store
            .apply_reference(
                context(4),
                input.request,
                ReferenceRequest {
                    id: ReferenceRequestId::new(NonZeroU128::new(id).unwrap()),
                    operation,
                },
            )
            .unwrap();
    }
    assert_eq!(
        store.download_descriptor(context(4), &scope, root, reference),
        Err(DownloadDescriptorError::Unavailable)
    );
    let live = store
        .download_descriptor(context(4), &scope, root, second)
        .unwrap();
    let restored = StableUploads::open(m, config()).unwrap();
    assert_eq!(
        restored.download_descriptor(context(4), &scope, root, second),
        Err(DownloadDescriptorError::Store(UploadStoreError::Fenced))
    );
    assert_eq!(
        restored
            .retained_content_descriptor(context(4), root, second)
            .unwrap(),
        Some(live)
    );
}
