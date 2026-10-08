//! Actual standalone operator scope, maintained accounting and passive restore inspection.
use super::*;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::protocol::LOCAL_STATUS_METHOD;
impl Fixture {
    pub(super) fn operator_scope(&self) -> OperatorScope {
        OperatorScope {
            service: self.service,
            namespace: self.config.namespace,
            cashier: self.config.billing.cashier,
            payment_account: self.config.payment_account,
        }
    }
    pub(super) fn local_status(
        &self,
        actor: Principal,
        scope: OperatorScope,
    ) -> Result<LocalServiceStatus, LocalStatusFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, LOCAL_STATUS_METHOD, (scope,))
            .unwrap()
    }
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One operator observation journey through admission, cancellation and restoration"
)]
fn standalone_operator_inspection_is_scoped_passive_and_retains_history_after_cancellation() {
    let f = Fixture::new();
    let scope = f.operator_scope();
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.local_status(actor, scope),
            Err(LocalStatusFailure::Denied)
        );
    }
    for changed in [
        OperatorScope {
            service: f.tenant,
            ..scope
        },
        OperatorScope {
            namespace: 1,
            ..scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..scope
        },
        OperatorScope {
            payment_account: f.operator,
            ..scope
        },
    ] {
        assert_eq!(
            f.local_status(f.operator, changed),
            Err(LocalStatusFailure::Binding)
        );
    }
    let initial = f.local_status(f.operator, scope).unwrap();
    assert_eq!(initial.scope, scope);
    assert_eq!(
        initial.funding.available_allocation,
        f.config.funding.allocated
    );
    assert_eq!(
        initial.funding.attachment_allowance,
        f.config.funding.allocated - f.config.funding.reserve
    );
    assert_eq!(initial.funding.transport_accepted, 0);
    assert_eq!(initial.funding.reserved_or_uncertain, 0);
    assert_eq!(initial.uploads.operations, 0);
    assert_eq!(initial.reads.sessions, 0);
    assert_eq!(initial.gateways.members, []);
    let replicated: Result<LocalServiceStatus, LocalStatusFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.operator, LOCAL_STATUS_METHOD, (scope,))
        .unwrap();
    assert_eq!(replicated, Ok(initial.clone()));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    f.enroll(f.operator).unwrap();
    let permission = f.manifest().permission;
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_admit_upload", (permission,))
        .unwrap();
    admitted.unwrap();
    let occupied = f.local_status(f.operator, scope).unwrap();
    assert_eq!(occupied.uploads.operations, 1);
    assert_eq!(occupied.uploads.active_reservations, 1);
    assert_eq!(
        occupied.uploads.reserved_bytes,
        u128::from(permission.upload.bytes)
    );
    assert_eq!(
        occupied.uploads.logical_bytes,
        occupied.uploads.reserved_bytes
    );
    assert_eq!(
        occupied.uploads.physical_bytes,
        occupied.uploads.reserved_bytes
    );
    assert_eq!(
        occupied.uploads.liability_bytes,
        occupied.uploads.reserved_bytes
    );
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(f.service, f.tenant, "blob_revoke_upload", (permission,))
        .unwrap();
    revoked.unwrap();
    let mut released = initial;
    released.uploads.operations = 1;
    assert_eq!(f.local_status(f.operator, scope), Ok(released.clone()));
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    released.uploads.fenced = true;
    released.funding.fenced = true;
    released.gateways.fenced = true;
    released.reads.fenced = true;
    assert_eq!(f.local_status(f.operator, scope), Ok(released));
    assert_eq!(
        f.local_status(f.controller, scope),
        Err(LocalStatusFailure::Denied)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
