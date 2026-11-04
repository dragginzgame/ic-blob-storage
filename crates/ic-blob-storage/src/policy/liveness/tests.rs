use std::num::NonZeroU128;

use ic_platform::Principal;

use super::*;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}
fn n(id: u128) -> NonZeroU128 {
    NonZeroU128::new(id).expect("identity")
}
fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("bound")
}
fn object() -> ObjectBinding {
    ObjectBinding::new(
        p(1),
        p(2),
        ObjectIdentity {
            namespace: n(1),
            object: n(1),
            incarnation: n(1),
        },
    )
    .expect("object")
}
fn key(object: ObjectBinding, id: u128) -> ReferenceKey {
    ReferenceKey::new(object, ReferenceId::new(n(id)))
}
fn context() -> TenantAccessContext {
    TenantAccessContext {
        service: p(1),
        actor: p(2),
    }
}

#[test]
fn bounded_reads_preserve_order_duplicates_and_distinguish_released_references() {
    let first = key(object(), 1);
    let second = key(object(), 2);
    let unknown = key(object(), 3);
    let mut lifecycle = BlobLifecycle::from_confirmed_upload(100, first, bound(2));
    lifecycle.retain(second).expect("second reference");
    lifecycle.release(first).expect("release first only");
    let before = lifecycle.clone();
    let keys = [second, first, unknown, second];
    assert_eq!(
        assess_reference_liveness_batch(&lifecycle, &keys, context(), bound(4)),
        Ok(vec![true, false, false, true])
    );
    for (key, expected) in keys.into_iter().zip([true, false, false, true]) {
        assert_eq!(
            assess_reference_liveness(&lifecycle, key, context()),
            Ok(expected)
        );
    }
    assert_eq!(
        assess_reference_liveness_batch(&lifecycle, &[], context(), bound(1)),
        Ok(vec![])
    );
    assert_eq!(
        assess_reference_liveness_batch(&lifecycle, &[second; 3], context(), bound(2)),
        Err(ReferenceLivenessError::TooManyEntries {
            actual: 3,
            maximum: 2
        })
    );
    assert_eq!(lifecycle, before);
}

#[test]
fn authority_precedes_input_details_and_all_scope_mismatches_reject() {
    let object = object();
    let first = key(object, 1);
    let lifecycle = BlobLifecycle::from_confirmed_upload(100, first, bound(1));
    let before = lifecycle.clone();
    let identity = object.identity();
    let mismatches = [
        (
            ObjectBinding::new(p(9), p(2), identity).expect("service"),
            ObjectBindingMismatch::Service,
        ),
        (
            ObjectBinding::new(p(1), p(9), identity).expect("tenant"),
            ObjectBindingMismatch::Tenant,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    namespace: n(2),
                    ..identity
                },
            )
            .expect("namespace"),
            ObjectBindingMismatch::Namespace,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    object: n(2),
                    ..identity
                },
            )
            .expect("object"),
            ObjectBindingMismatch::Object,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    incarnation: n(2),
                    ..identity
                },
            )
            .expect("incarnation"),
            ObjectBindingMismatch::Incarnation,
        ),
    ];
    for (foreign, error) in mismatches {
        let foreign_key = key(foreign, 1);
        assert_eq!(
            assess_reference_liveness(&lifecycle, foreign_key, context()),
            Err(ReferenceLivenessError::Binding(error))
        );
        assert_eq!(
            assess_reference_liveness_batch(&lifecycle, &[first, foreign_key], context(), bound(2)),
            Err(ReferenceLivenessError::Binding(error))
        );
    }
    for actor in [
        Principal::anonymous(),
        Principal::management_canister(),
        p(1),
        p(9),
    ] {
        let unauthorized = TenantAccessContext { actor, ..context() };
        assert_eq!(
            assess_reference_liveness(&lifecycle, first, unauthorized),
            Err(ReferenceLivenessError::Access(TenantAccessError::NotTenant))
        );
        for keys in [&[][..], &[first, first][..]] {
            assert_eq!(
                assess_reference_liveness_batch(&lifecycle, keys, unauthorized, bound(1)),
                Err(ReferenceLivenessError::Access(TenantAccessError::NotTenant))
            );
        }
    }
    assert_eq!(
        assess_reference_liveness(
            &lifecycle,
            first,
            TenantAccessContext {
                service: p(9),
                ..context()
            }
        ),
        Err(ReferenceLivenessError::Access(
            TenantAccessError::WrongService
        ))
    );
    assert_eq!(lifecycle, before);
}
