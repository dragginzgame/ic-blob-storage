//! Native composition of trusted-context policy and scoped lifecycle mutations.
//! This does not simulate IC caller authentication, storage or provider evidence.

use std::num::{NonZeroU128, NonZeroUsize};

use candid::Principal;
use ic_blob_storage::{
    model::lifecycle::{
        BlobLifecycle, LifecycleChange, LifecycleError, ReferenceId,
        binding::{ObjectBinding, ObjectBindingMismatch, ObjectIdentity, ReferenceKey},
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
            ReferenceRequests,
        },
        roots::{RootClaimError, RootClaimOutcome, RootClaims},
    },
    policy::tenant::{TenantAccessContext, TenantAccessError, assess_tenant_access},
};

fn p(id: u8) -> Principal {
    Principal::from_slice(&[id, 1])
}
fn number(id: u128) -> NonZeroU128 {
    NonZeroU128::new(id).expect("positive ID")
}
fn identity() -> ObjectIdentity {
    ObjectIdentity {
        namespace: number(1),
        object: number(1),
        incarnation: number(1),
    }
}
fn binding() -> ObjectBinding {
    ObjectBinding::new(p(1), p(2), identity()).expect("valid binding")
}
fn key(object: ObjectBinding) -> ReferenceKey {
    ReferenceKey::new(object, ReferenceId::new(number(1)))
}
fn blob() -> BlobLifecycle {
    BlobLifecycle::from_confirmed_upload(
        100,
        key(binding()),
        NonZeroUsize::new(2).expect("positive limit"),
    )
}

#[test]
fn delayed_root_callback_cannot_be_rebound_to_a_newer_pending_deletion() {
    use ic_blob_storage::model::identity::ProviderRootHash;

    let root = ProviderRootHash::try_from([7; 32].as_slice()).expect("root");
    let mut claims = RootClaims::new(p(1), NonZeroUsize::new(1).expect("limit")).expect("service");
    claims
        .claim(root, binding())
        .expect("original root allocation");
    let mut original = blob();
    original.release(key(binding())).expect("release original");
    let callback_binding = claims.resolve(root).expect("original association");
    original
        .confirm_provider_deleted(callback_binding)
        .expect("deletion");
    original
        .confirm_billing_stopped(callback_binding)
        .expect("settlement");

    let newer_binding = ObjectBinding::new(
        p(1),
        p(2),
        ObjectIdentity {
            incarnation: number(2),
            ..identity()
        },
    )
    .expect("new incarnation");
    assert_eq!(
        claims.claim(root, newer_binding),
        Err(RootClaimError::RootAlreadyClaimed)
    );
    assert_eq!(claims.slots(), 1);
    assert_eq!(
        claims.claim(root, binding()),
        Ok(RootClaimOutcome::Existing)
    );

    // Independently construct a newer pending object to test stale correlation,
    // even though the maintained root allocator has already denied this reuse.
    let mut newer = BlobLifecycle::from_confirmed_upload(
        100,
        key(newer_binding),
        NonZeroUsize::new(1).expect("reference limit"),
    );
    newer
        .release(key(newer_binding))
        .expect("newer deletion pending");
    let before = newer.clone();
    assert_eq!(
        newer.confirm_provider_deleted(claims.resolve(root).expect("retained claim")),
        Err(LifecycleError::BindingMismatch(
            ObjectBindingMismatch::Incarnation
        ))
    );
    assert_eq!(newer, before);
    assert_eq!(newer.physical_bytes(), 100);
    assert_eq!(newer.liability_bytes(), 100);
    assert_eq!(
        original.confirm_provider_deleted(callback_binding),
        Ok(LifecycleChange::Unchanged)
    );
}

#[test]
fn every_scope_mismatch_rejects_before_mutation_or_idempotent_replay() {
    let expected = binding();
    let cases = [
        (
            ObjectBinding::new(p(3), p(2), identity()).expect("other service"),
            ObjectBindingMismatch::Service,
        ),
        (
            ObjectBinding::new(p(1), p(3), identity()).expect("other tenant"),
            ObjectBindingMismatch::Tenant,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    namespace: number(2),
                    ..identity()
                },
            )
            .expect("other namespace"),
            ObjectBindingMismatch::Namespace,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    object: number(2),
                    ..identity()
                },
            )
            .expect("other object"),
            ObjectBindingMismatch::Object,
        ),
        (
            ObjectBinding::new(
                p(1),
                p(2),
                ObjectIdentity {
                    incarnation: number(2),
                    ..identity()
                },
            )
            .expect("other incarnation"),
            ObjectBindingMismatch::Incarnation,
        ),
    ];
    let mut value = blob();
    // Check live, queued, deleted and settled states, including replay paths.
    for stage in 0..4 {
        let before = value.clone();
        for (other, mismatch) in cases {
            let error = Err(LifecycleError::BindingMismatch(mismatch));
            assert_eq!(value.retain(key(other)), error);
            assert_eq!(value.release(key(other)), error);
            assert_eq!(value.confirm_provider_deleted(other), error);
            assert_eq!(value.confirm_billing_stopped(other), error);
            assert_eq!(value, before);
        }
        match stage {
            0 => {
                value.release(key(expected)).expect("release");
            }
            1 => {
                value.confirm_provider_deleted(expected).expect("delete");
            }
            2 => {
                value.confirm_billing_stopped(expected).expect("settle");
            }
            _ => {}
        }
    }
}

#[test]
fn a_matching_reference_key_does_not_authorize_another_actor_or_service() {
    let mut value = blob();
    let request = key(binding());
    let before = value.clone();
    for actor in [
        Principal::anonymous(),
        Principal::management_canister(),
        p(1),
        p(3),
    ] {
        // p(1) is even the owning service principal, but not the tenant p(2).
        assert_eq!(
            assess_tenant_access(
                value.binding(),
                TenantAccessContext {
                    service: p(1),
                    actor
                }
            ),
            Err(TenantAccessError::NotTenant),
        );
    }
    assert_eq!(
        assess_tenant_access(
            value.binding(),
            TenantAccessContext {
                service: p(3),
                actor: p(2)
            }
        ),
        Err(TenantAccessError::WrongService),
    );
    assert_eq!(value, before);
    assess_tenant_access(
        value.binding(),
        TenantAccessContext {
            service: p(1),
            actor: p(2),
        },
    )
    .expect("direct tenant in the correct service");
    assert_eq!(value.release(request), Ok(LifecycleChange::Changed));
    assert_eq!(value.release(request), Ok(LifecycleChange::Unchanged));
}

#[test]
fn receipt_replay_still_requires_current_tenant_access() {
    let mut state = ReferenceRequests::new(blob(), NonZeroUsize::new(2).expect("receipt limit"))
        .expect("room for release");
    let request = ReferenceRequest {
        id: ReferenceRequestId::new(number(1)),
        operation: ReferenceOperation::Release(key(binding())),
    };
    // Native composition only: an endpoint must supply these trusted values.
    let apply = |state: &mut ReferenceRequests, context: TenantAccessContext| {
        assess_tenant_access(state.lifecycle().binding(), context)?;
        Ok::<_, TenantAccessError>(state.apply(context.actor, request))
    };
    let owner = TenantAccessContext {
        service: p(1),
        actor: p(2),
    };
    assert_eq!(
        apply(&mut state, owner),
        Ok(Ok(ReferenceRequestOutcome::Recorded {
            result: Ok(LifecycleChange::Changed)
        }))
    );
    let before = state.clone();
    assert_eq!(
        apply(
            &mut state,
            TenantAccessContext {
                actor: p(3),
                ..owner
            }
        ),
        Err(TenantAccessError::NotTenant)
    );
    assert_eq!(
        apply(
            &mut state,
            TenantAccessContext {
                service: p(3),
                ..owner
            }
        ),
        Err(TenantAccessError::WrongService)
    );
    assert_eq!(state, before);
    assert_eq!(
        apply(&mut state, owner),
        Ok(Ok(ReferenceRequestOutcome::Replayed {
            result: Ok(LifecycleChange::Changed)
        }))
    );
    assert_eq!(state, before);
}
