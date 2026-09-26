//! Fixed upload facts for IC caller tests; no provider effects or stable state.

use super::{bound, number, reference};
use blob_test_protocol::uploads::UploadProbeState;
use candid::Principal;
use ic_blob_storage::{
    model::{
        catalog::{
            CatalogLimits,
            admission::{
                UploadCatalog, UploadLimits, UploadObject, UploadRequest, UploadRequestId,
                read::UploadRootState,
            },
        },
        identity::{
            ContentDigest,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
    },
    policy::catalog::upload::UploadRootStatus,
};

pub(crate) struct Uploads {
    pub catalog: UploadCatalog,
    requests: [UploadRequest; 4],
}

pub(crate) fn initialize(service: Principal, first: Principal, second: Principal) -> Uploads {
    let mut catalog = UploadCatalog::new(
        service,
        CatalogLimits {
            max_objects: bound(4),
            max_tenant_objects: bound(2),
            max_physical_bytes: number(400),
            max_liability_bytes: number(400),
            max_tenant_logical_bytes: number(200),
            max_references_per_object: bound(1),
            max_receipts_per_object: bound(1),
        },
        UploadLimits {
            max_active: bound(4),
            max_tenant_active: bound(2),
        },
    )
    .expect("fixture upload owner");
    let requests = [(11, first), (12, first), (13, second), (14, second)].map(|(id, tenant)| {
        let reference = reference(id, service, tenant).expect("fixture reference");
        UploadRequest {
            id: UploadRequestId::new(number(u128::from(id))),
            object: UploadObject {
                root: reference.root,
                bytes: 100,
                first: reference.reference,
            },
            content: ContentDigest::compute(&[id]),
        }
    });
    for request in requests {
        catalog
            .reserve(request.object.first.object().tenant(), request)
            .expect("fixture reservation");
    }
    catalog
        .mark_exposure_possible(first, requests[1])
        .expect("uncertain fixture");
    catalog
        .mark_exposure_possible(second, requests[2])
        .expect("fixture exposure");
    catalog
        .confirm_upload(requests[2])
        .expect("substituted completion fact");
    catalog
        .cancel(second, requests[3])
        .expect("unexposed cancellation");
    Uploads { catalog, requests }
}

pub(crate) fn cancel(actor: Principal, root: u8) -> bool {
    super::STATE.with_borrow_mut(|state| {
        let uploads = &mut state.as_mut().expect("initialized").uploads;
        let Some(request) = uploads
            .requests
            .iter()
            .find(|request| request.object.root.as_bytes()[0] == root)
            .copied()
        else {
            return false;
        };
        uploads.catalog.cancel(actor, request).is_ok()
    })
}

pub(crate) fn roots() -> ProviderRootBatch {
    ProviderRootBatch::from_bytes(
        &[vec![11; 32], vec![12; 32], vec![13; 32], vec![14; 32]],
        RootBatchLimits {
            max_entries: bound(4),
            max_bytes: bound(128),
        },
    )
    .expect("fixed root batch")
}

pub(crate) fn status(status: UploadRootStatus) -> Option<UploadProbeState> {
    match status {
        UploadRootStatus::Known(UploadRootState::Reserved) => Some(UploadProbeState::Reserved),
        UploadRootStatus::Known(UploadRootState::ExposurePossible) => {
            Some(UploadProbeState::ExposurePossible)
        }
        UploadRootStatus::Known(UploadRootState::Cancelled) => Some(UploadProbeState::Cancelled),
        UploadRootStatus::Known(UploadRootState::Confirmed(_)) => Some(UploadProbeState::Confirmed),
        _ => None,
    }
}
