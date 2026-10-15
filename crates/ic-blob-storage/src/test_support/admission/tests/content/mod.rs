use super::*;
use crate::model::lifecycle::LifecycleError;
use crate::model::lifecycle::requests::ReferenceRequestError;
use crate::model::lifecycle::roots::RootClaimError;
use crate::model::service::upload::content::ContentLookup;
use crate::model::service::upload::content::TenantContentView;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;

fn query(input: UploadPermission) -> ContentLookup {
    ContentLookup {
        tenant: p(4),
        namespace: id(1),
        root: input.request.object.root,
    }
}

fn state(owner: &UploadAdmissions, input: UploadPermission) -> UploadRootState {
    let found = owner
        .lookup_content(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(found.request, input.request);
    found.state
}

fn assert_initial_capacity(owner: &UploadAdmissions, input: UploadPermission) {
    assert_eq!(
        owner.lookup_content(context(4), query(input)),
        Ok(Some(TenantContentView {
            request: input.request,
            state: UploadRootState::Confirmed(LifecyclePhase::Live),
        }))
    );
    let capacity = owner
        .reference_capacity(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(capacity.fresh_retains, 1);
    assert_eq!(capacity.release_reserved_receipts, 1);
    for actor in [2, 5, 6] {
        assert_eq!(
            owner.reference_capacity(context(actor), query(input)),
            Err(UploadAdmissionError::NotProject)
        );
    }
}

#[test]
fn discovery_is_tenant_scoped_passive_and_preserves_cancelled_identity() {
    let mut owner = admissions();
    let mut input = permission(1);
    // Operation identity need not equal object identity.
    input.request.id = UploadRequestId::new(id(99));
    assert_eq!(owner.lookup_content(context(4), query(input)), Ok(None));
    owner.admit(context(4), input, 1).unwrap();
    let usage = owner.catalog().usage();
    for actor in [2, 5, 6] {
        assert_eq!(
            owner.lookup_content(context(actor), query(input)),
            Err(UploadAdmissionError::NotProject)
        );
    }
    assert_eq!(
        owner.lookup_content(
            context(6),
            ContentLookup {
                tenant: p(6),
                ..query(input)
            }
        ),
        Ok(None)
    );
    assert_eq!(
        owner.lookup_content(
            UploadContext {
                service: p(9),
                ..context(4)
            },
            query(input)
        ),
        Err(UploadAdmissionError::WrongService)
    );
    assert_eq!(
        owner.lookup_content(
            context(4),
            ContentLookup {
                namespace: id(9),
                ..query(input)
            }
        ),
        Err(UploadAdmissionError::WrongNamespace)
    );
    assert_eq!(state(&owner, input), UploadRootState::Reserved);
    assert_eq!(owner.catalog().usage(), usage);
    let enrollment = owner.tenant(context(4), p(4)).unwrap();
    owner
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: enrollment,
                active: false,
            },
        )
        .unwrap();
    assert_eq!(state(&owner, input), UploadRootState::Reserved);
    owner.revoke(context(4), input.request).unwrap();
    assert_eq!(state(&owner, input), UploadRootState::Cancelled);
    assert_eq!(
        owner.admit(context(4), input, 200),
        Ok(UploadAdmission::Existing(UploadPhase::Cancelled))
    );
    assert_eq!(state(&owner, input), UploadRootState::Cancelled);
}

#[test]
fn overlapping_releases_recover_receipts_and_cleanup_at_history_capacity() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 2).unwrap();
    assert_eq!(state(&owner, input), UploadRootState::ExposurePossible);
    owner.confirm_upload(input.request).unwrap(); // Trusted local fact, not provider evidence.
    let first = input.request.object.first;
    let second = ReferenceKey::new(first.object(), ReferenceId::new(id(2)));
    let retain = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Retain(second),
    };
    let release_first = ReferenceRequest {
        id: ReferenceRequestId::new(id(2)),
        operation: ReferenceOperation::Release(first),
    };
    let release_second = ReferenceRequest {
        id: ReferenceRequestId::new(id(3)),
        operation: ReferenceOperation::Release(second),
    };
    let root = input.request.object.root;
    let original_usage = owner.catalog().usage();
    assert_initial_capacity(&owner, input);
    let changed = ReferenceRequestOutcome::Recorded {
        result: Ok(LifecycleChange::Changed),
    };
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(changed.clone())
    );
    // A lost retain reply is recovered without another reference or upload.
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert_eq!(
        owner.catalog().usage().physical_bytes,
        original_usage.physical_bytes
    );
    assert_eq!(
        owner.catalog().usage().logical_bytes,
        original_usage.logical_bytes
    );
    let rejected = ReferenceRequest {
        id: ReferenceRequestId::new(id(4)),
        operation: ReferenceOperation::Retain(ReferenceKey::new(
            first.object(),
            ReferenceId::new(id(3)),
        )),
    };
    assert_eq!(
        owner.apply_reference(context(4), root, rejected),
        Err(UploadAdmissionError::Reference(CatalogError::Request(
            ReferenceRequestError::ReceiptLimitReached
        )))
    );
    // Failed/abandoned publication still owns its retained reference until cleanup.
    let full = owner
        .reference_capacity(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(full.fresh_retains, 0);
    assert_eq!(full.unreserved_receipts, 0);
    assert_eq!(full.release_reserved_receipts, 2);
    assert_eq!(
        owner.apply_reference(context(4), root, release_first),
        Ok(changed.clone())
    );
    assert_eq!(
        state(&owner, input),
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    assert_eq!(
        owner.apply_reference(context(4), root, release_second),
        Ok(changed)
    );
    assert_eq!(
        state(&owner, input),
        UploadRootState::Confirmed(LifecyclePhase::DeletionPending)
    );
    assert_eq!(owner.catalog().usage().logical_bytes, 0);
    assert_eq!(
        owner.catalog().usage().physical_bytes,
        original_usage.physical_bytes
    );
    // Replayed success is the old receipt, not current liveness or resurrection.
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    settle_and_check_retained_identity(&mut owner, input);
}

fn settle_and_check_retained_identity(owner: &mut UploadAdmissions, input: UploadPermission) {
    let root = input.request.object.root;
    let first = input.request.object.first;
    let pending_usage = owner.catalog().usage();
    owner
        .confirm_provider_deleted(root, first.object())
        .unwrap();
    assert_eq!(
        state(owner, input),
        UploadRootState::Confirmed(LifecyclePhase::ProviderDeleted)
    );
    assert_eq!(owner.catalog().usage().physical_bytes, 0);
    assert_eq!(
        owner.catalog().usage().liability_bytes,
        pending_usage.liability_bytes
    );
    owner.confirm_billing_stopped(root, first.object()).unwrap();
    assert_eq!(
        state(owner, input),
        UploadRootState::Confirmed(LifecyclePhase::Settled)
    );
    assert_eq!(owner.catalog().usage().liability_bytes, 0);
    let full = owner
        .reference_capacity(context(4), query(input))
        .unwrap()
        .unwrap();
    assert_eq!(full.fresh_retains, 0);
    assert_eq!(full.unreserved_receipts, 0);
    assert_eq!(full.release_reserved_receipts, 0);
    let settled_usage = owner.catalog().usage();
    let mut reintroduced = permission(2);
    reintroduced.request.object.root = root;
    assert_eq!(
        owner.admit(context(4), reintroduced, 3),
        Err(UploadAdmissionError::Catalog(UploadError::Root(
            RootClaimError::RootAlreadyClaimed
        )))
    );
    assert_eq!(owner.catalog().usage(), settled_usage);
    assert_eq!(
        owner.admit(context(4), input, 200),
        Ok(UploadAdmission::Existing(UploadPhase::Confirmed))
    );
    assert_eq!(
        state(owner, input),
        UploadRootState::Confirmed(LifecyclePhase::Settled)
    );
}

#[test]
fn stale_live_discovery_does_not_authorize_a_later_retain() {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 2).unwrap();
    owner.confirm_upload(input.request).unwrap();
    assert_eq!(
        state(&owner, input),
        UploadRootState::Confirmed(LifecyclePhase::Live)
    );
    let root = input.request.object.root;
    let first = input.request.object.first;
    owner
        .apply_reference(
            context(4),
            root,
            ReferenceRequest {
                id: ReferenceRequestId::new(id(1)),
                operation: ReferenceOperation::Release(first),
            },
        )
        .unwrap();
    let retain = ReferenceRequest {
        id: ReferenceRequestId::new(id(2)),
        operation: ReferenceOperation::Retain(ReferenceKey::new(
            first.object(),
            ReferenceId::new(id(2)),
        )),
    };
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(ReferenceRequestOutcome::Recorded {
            result: Err(LifecycleError::DeletionAlreadyQueued)
        })
    );
    assert_eq!(
        state(&owner, input),
        UploadRootState::Confirmed(LifecyclePhase::DeletionPending)
    );
}
