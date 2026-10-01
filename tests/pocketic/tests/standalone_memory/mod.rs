//! Host capacity is scoped, passive and independent of service accounting.
use super::*;
use ic_blob_storage::dto::operator::{LocalStatusFailure, memory::HostMemoryStatus};
use ic_blob_storage::ops::service::operator::memory::HOST_MEMORY_STATUS_METHOD;

impl Fixture {
    fn memory_status(
        &self,
        actor: Principal,
        scope: ic_blob_storage::dto::operator::OperatorScope,
    ) -> Result<HostMemoryStatus, LocalStatusFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, HOST_MEMORY_STATUS_METHOD, (scope,))
            .unwrap()
    }
}
fn conservation(view: HostMemoryStatus) {
    assert_eq!(
        view.physical_extent_bytes,
        view.manager_metadata_bytes + view.allocated_bucket_bytes + view.unmanaged_bytes
    );
    assert_eq!(
        view.allocated_bucket_bytes,
        view.virtual_extent_bytes + view.bucket_slack_bytes
    );
    assert_eq!(
        view.allocated_bucket_bytes,
        view.current_binding_bytes + view.ledger_binding_bytes + view.unknown_binding_bytes
    );
}
#[test]
fn host_memory_status_is_operator_scoped_passive_and_available_after_fenced_restore() {
    let f = Fixture::new();
    let scope = f.operator_scope();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.memory_status(actor, scope),
            Err(LocalStatusFailure::Denied)
        );
    }
    for changed in [
        ic_blob_storage::dto::operator::OperatorScope {
            service: f.tenant,
            ..scope
        },
        ic_blob_storage::dto::operator::OperatorScope {
            namespace: 1,
            ..scope
        },
        ic_blob_storage::dto::operator::OperatorScope {
            cashier: f.tenant,
            ..scope
        },
        ic_blob_storage::dto::operator::OperatorScope {
            payment_account: f.operator,
            ..scope
        },
    ] {
        assert_eq!(
            f.memory_status(f.operator, changed),
            Err(LocalStatusFailure::Binding)
        );
    }
    let initial = f.memory_status(f.operator, scope).unwrap();
    assert_eq!(initial.scope, scope);
    assert_eq!(initial.bucket_size_pages, 16);
    assert!(initial.allocated_bucket_bytes > 0);
    assert_eq!(initial.unknown_binding_bytes, 0);
    conservation(initial);
    let replicated: Result<HostMemoryStatus, LocalStatusFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.operator, HOST_MEMORY_STATUS_METHOD, (scope,))
        .unwrap();
    assert_eq!(replicated, Ok(initial));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.enroll(f.operator).unwrap();
    let permission = f.manifest().permission;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    conservation(f.memory_status(f.operator, scope).unwrap());
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    conservation(f.memory_status(f.operator, scope).unwrap());
    let local = f.local_status(f.operator, scope).unwrap();
    assert!(
        local.uploads.fenced && local.funding.fenced && local.gateways.fenced && local.reads.fenced
    );
    assert_eq!(
        local.uploads.reserved_bytes,
        u128::from(permission.upload.bytes)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
