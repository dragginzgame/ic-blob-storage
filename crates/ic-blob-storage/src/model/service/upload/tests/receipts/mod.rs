use super::*;
use crate::model::lifecycle::{LifecycleError, requests::ReferenceRequestError};

fn confirmed() -> (UploadAdmissions, UploadPermission) {
    let mut owner = admissions();
    let input = permission(1);
    owner.admit(context(4), input, 1).unwrap();
    prepare(&mut owner, &input);
    owner.expose(context(5), input.request, 2).unwrap();
    owner.confirm_upload(input.request).unwrap();
    (owner, input)
}

#[test]
fn receipt_reads_preserve_success_after_release_suspension_and_settlement() {
    let (mut owner, input) = confirmed();
    let root = input.request.object.root;
    let first = input.request.object.first;
    let second = ReferenceKey::new(first.object(), ReferenceId::new(id(2)));
    let retain = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Retain(second),
    };
    let initial = owner.catalog().usage();
    assert_eq!(owner.reference_receipt(context(4), root, retain), Ok(None));
    assert_eq!(owner.catalog().usage(), initial);
    owner.apply_reference(context(4), root, retain).unwrap();
    let expected = Some(ReferenceReceiptView {
        result: Ok(LifecycleChange::Changed),
    });
    assert_eq!(
        owner.reference_receipt(context(4), root, retain),
        Ok(expected)
    );
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
    for (n, reference) in [(2, second), (3, first)] {
        owner
            .apply_reference(
                context(4),
                root,
                ReferenceRequest {
                    id: ReferenceRequestId::new(id(n)),
                    operation: ReferenceOperation::Release(reference),
                },
            )
            .unwrap();
    }
    owner
        .confirm_provider_deleted(root, first.object())
        .unwrap();
    owner.confirm_billing_stopped(root, first.object()).unwrap();
    let settled = owner.catalog().usage();
    assert_eq!(
        owner.reference_receipt(context(4), root, retain),
        Ok(expected)
    );
    assert_eq!(
        owner.reference_receipt(
            context(4),
            root,
            ReferenceRequest {
                id: ReferenceRequestId::new(id(4)),
                ..retain
            }
        ),
        Ok(None)
    );
    assert_eq!(owner.catalog().usage(), settled);
    assert_eq!(
        owner.apply_reference(context(4), root, retain),
        Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        })
    );
    assert_eq!(owner.catalog().usage(), settled);
    assert_eq!(
        owner.reference_receipt(
            context(4),
            root,
            ReferenceRequest {
                operation: ReferenceOperation::Release(second),
                ..retain
            }
        ),
        Err(UploadAdmissionError::Reference(CatalogError::Request(
            ReferenceRequestError::RequestConflict
        )))
    );
}

#[test]
fn failed_receipts_remain_typed_and_reads_check_authority_even_for_absent_ids() {
    let (mut owner, input) = confirmed();
    let root = input.request.object.root;
    let missing = ReferenceKey::new(input.request.object.first.object(), ReferenceId::new(id(9)));
    let request = ReferenceRequest {
        id: ReferenceRequestId::new(id(1)),
        operation: ReferenceOperation::Release(missing),
    };
    owner.apply_reference(context(4), root, request).unwrap();
    let before = owner.catalog().usage();
    assert_eq!(
        owner.reference_receipt(context(4), root, request),
        Ok(Some(ReferenceReceiptView {
            result: Err(LifecycleError::UnknownReference)
        }))
    );
    for actor in [2, 5, 6] {
        assert_eq!(
            owner.reference_receipt(context(actor), root, request),
            Err(UploadAdmissionError::NotProject)
        );
    }
    let wrong = ReferenceKey::new(
        ObjectBinding::new(
            p(1),
            p(4),
            ObjectIdentity {
                incarnation: id(2),
                ..missing.object().identity()
            },
        )
        .unwrap(),
        ReferenceId::new(id(9)),
    );
    assert!(matches!(
        owner.reference_receipt(
            context(4),
            root,
            ReferenceRequest {
                id: ReferenceRequestId::new(id(2)),
                operation: ReferenceOperation::Release(wrong)
            }
        ),
        Err(UploadAdmissionError::Reference(CatalogError::Request(
            ReferenceRequestError::BindingMismatch(_)
        )))
    ));
    assert_eq!(
        owner.reference_receipt(context(4), permission(2).request.object.root, request),
        Err(UploadAdmissionError::Reference(CatalogError::UnknownRoot))
    );
    assert_eq!(owner.catalog().usage(), before);
}
