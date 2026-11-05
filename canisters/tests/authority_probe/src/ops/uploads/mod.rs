//! Fixed upload facts for IC caller tests; archived by the enclosing state owner.

use super::durable::{Owner, context};
use super::{bound, number, reference};
use blob_test_protocol::uploads::UploadProbeState;
use ic_blob_storage::ic_storage::{MemoryRuntime, VectorMemory};
use ic_blob_storage::policy::catalog::upload::UploadRootStatus;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::configuration::limits::UploadLimits;
use ic_blob_storage_contracts::identity::batch::ProviderRootBatch;
use ic_blob_storage_contracts::identity::batch::RootBatchLimits;
use ic_blob_storage_contracts::upload::binding::UploadObject;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;
use ic_blob_storage_contracts::upload::history::UploadRootState;
use ic_platform::Principal;

pub(crate) struct Uploads {
    pub catalog: Owner,
    _runtime: MemoryRuntime<VectorMemory>,
    pub(super) requests: [UploadRequest; 4],
}

pub(crate) fn initialize(
    service: Principal,
    first: Principal,
    second: Principal,
    operator: Principal,
) -> Uploads {
    let (mut catalog, runtime) = super::durable::install(
        service,
        operator,
        [first, second],
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
    );
    let requests = [(11, first), (12, first), (13, second), (14, second)].map(|(id, tenant)| {
        let reference = reference(id, service, tenant).expect("fixture reference");
        UploadRequest {
            id: UploadRequestId::new(number(u128::from(id))),
            object: UploadObject {
                root: reference.root,
                bytes: 100,
                first: reference.reference,
            },
        }
    });
    for request in requests {
        catalog
            .admit(
                context(service, request.object.first.object().tenant()),
                super::durable::permission(request),
                0,
            )
            .expect("fixture reservation");
    }
    for (actor, request, id) in [(first, requests[1], 12), (second, requests[2], 13)] {
        super::durable::prepare_sample(&mut catalog, request, id);
        ic_blob_storage::test_support::exposure::record(
            &mut catalog,
            context(service, actor),
            request,
            0,
        )
        .unwrap();
    }
    catalog
        .revoke(context(service, second), requests[3])
        .unwrap();
    Uploads {
        catalog,
        requests,
        _runtime: runtime,
    }
}

pub(crate) fn cancel(actor: Principal, root: u8) -> bool {
    super::mutate(|state| {
        let uploads = &mut state.uploads;
        let Some(request) = uploads
            .requests
            .iter()
            .find(|request| request.id.get().get() == u128::from(root))
            .copied()
        else {
            return false;
        };
        let ctx = context(state.service, actor);
        let Ok(view) = uploads.catalog.lookup(ctx, request) else {
            return false;
        };
        view.phase != ic_blob_storage::model::catalog::admission::UploadPhase::Registered
            && uploads.catalog.revoke(ctx, request).is_ok()
    })
}

pub(crate) fn roots() -> ProviderRootBatch {
    let roots: Vec<_> = [11, 12, 13, 14]
        .map(|id| super::root(id).as_bytes().to_vec())
        .into();
    ProviderRootBatch::from_bytes(
        &roots,
        RootBatchLimits {
            max_entries: bound(4),
            max_bytes: bound(128),
        },
    )
    .unwrap()
}

pub(crate) fn status(status: UploadRootStatus) -> Option<UploadProbeState> {
    match status {
        UploadRootStatus::Known(UploadRootState::Reserved) => Some(UploadProbeState::Reserved),
        UploadRootStatus::Known(UploadRootState::Cancelled) => Some(UploadProbeState::Cancelled),
        UploadRootStatus::Known(UploadRootState::Confirmed(_)) => {
            Some(UploadProbeState::Registered)
        }
        _ => None,
    }
}
