use super::*;
use blob_test_protocol::{
    admission::{
        ContentLookup, ContentState,
        input::ReferenceInput,
        planning::{AdmissionCapacity, AdmissionCapacityInput},
        release::ReferenceCapacity,
    },
    storage::{
        ProviderFact,
        read::{RootBatchInput, RootObservation},
    },
};

impl Fixture {
    fn capacity_input(&self) -> AdmissionCapacityInput {
        AdmissionCapacityInput {
            service: self.service,
            tenant: self.tenant,
            namespace: 1,
        }
    }
    fn capacity(
        &self,
        actor: Principal,
        input: AdmissionCapacityInput,
    ) -> Result<AdmissionCapacity, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "admission_capacity", (input,))
            .unwrap()
    }
    fn reference_headroom(
        &self,
        actor: Principal,
        input: ContentLookup,
    ) -> Result<Option<ReferenceCapacity>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "reference_capacity", (input,))
            .unwrap()
    }
    fn roots(
        &self,
        actor: Principal,
        input: &RootBatchInput,
    ) -> Result<Vec<RootObservation>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "observe_roots", (input,))
            .unwrap()
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered capacity, cleanup and fenced upgrade journey"
)]
fn durable_headroom_preserves_cleanup_slots_and_billing_until_settlement() {
    let f = Fixture::new();
    let scope = f.capacity_input();
    assert_eq!(f.capacity(f.tenant, scope), Err(Failure::Inactive));
    f.enroll(None, true).unwrap();
    let initial = f.capacity(f.tenant, scope).unwrap();
    assert_eq!(initial.remaining_bytes, 20);
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let lookup = ContentLookup {
        service: f.service,
        tenant: f.tenant,
        namespace: 1,
        root: permission.request.root,
    };
    assert_eq!(f.reference_headroom(f.tenant, lookup), Ok(None));
    f.admit(f.tenant, permission).unwrap();
    assert_eq!(f.reference_headroom(f.tenant, lookup), Ok(None));
    assert_eq!(f.capacity(f.tenant, scope).unwrap().remaining_bytes, 10);
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    assert_eq!(
        f.reference_headroom(f.tenant, lookup),
        Ok(Some(ReferenceCapacity {
            reference_slots: 1,
            unreserved_receipts: 2,
            release_reserved_receipts: 1,
            fresh_retains: 1,
        }))
    );
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 2,
        operation: 1,
        retain: true,
    })
    .unwrap();
    assert_eq!(
        f.reference_headroom(f.tenant, lookup),
        Ok(Some(ReferenceCapacity {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: 2,
            fresh_retains: 0,
        }))
    );
    let active = f.tenant().unwrap();
    f.enroll(Some(active), false).unwrap();
    assert!(!f.capacity(f.tenant, scope).unwrap().enrollment.active);
    for (reference, operation) in [(1, 2), (2, 3)] {
        f.reference(ReferenceInput {
            object: permission.request,
            reference,
            operation,
            retain: false,
        })
        .unwrap();
    }
    let retired = ReferenceCapacity {
        reference_slots: 0,
        unreserved_receipts: 0,
        release_reserved_receipts: 0,
        fresh_retains: 0,
    };
    assert_eq!(f.reference_headroom(f.tenant, lookup), Ok(Some(retired)));
    assert_eq!(f.capacity(f.tenant, scope).unwrap().remaining_bytes, 10);
    f.fact(permission.request, ProviderFact::Deleted).unwrap();
    assert_eq!(f.capacity(f.tenant, scope).unwrap().remaining_bytes, 10);
    f.fact(permission.request, ProviderFact::Settled).unwrap();
    let settled = f.capacity(f.tenant, scope).unwrap();
    assert_eq!(settled.remaining_bytes, 20);
    assert_eq!(settled.remaining_objects, initial.remaining_objects - 1);
    assert_eq!(
        settled.remaining_manifest_chunks,
        initial.remaining_manifest_chunks - 1
    );
    assert_eq!(
        settled.remaining_active_uploads,
        initial.remaining_active_uploads
    );
    for actor in [f.operator, f.controller, f.uploader, f.other] {
        assert_eq!(f.capacity(actor, scope), Err(Failure::Denied));
        assert_eq!(f.reference_headroom(actor, lookup), Err(Failure::Denied));
    }
    assert_eq!(
        f.reference_headroom(
            f.other,
            ContentLookup {
                tenant: f.other,
                ..lookup
            }
        ),
        Ok(None)
    );
    for changed in [
        AdmissionCapacityInput {
            service: f.other,
            ..scope
        },
        AdmissionCapacityInput {
            namespace: 2,
            ..scope
        },
    ] {
        assert_eq!(f.capacity(f.tenant, changed), Err(Failure::Binding));
    }
    let before = f.status();
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.capacity(f.tenant, scope), Ok(settled));
    assert_eq!(f.reference_headroom(f.tenant, lookup), Ok(Some(retired)));
    assert_eq!(f.admit(f.tenant, permission), Err(Failure::Fenced));
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
}

#[test]
fn operator_root_batches_are_bounded_authorized_and_preserved_after_upgrade() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    f.revoke(permission.request).unwrap();
    let cancelled = f.permission(2, 2).0;
    f.admit(f.tenant, cancelled).unwrap();
    f.revoke(cancelled.request).unwrap();
    let query = RootBatchInput {
        service: f.service,
        namespace: 1,
        roots: vec![
            permission.request.root.to_vec(),
            vec![9; 32],
            vec![],
            cancelled.request.root.to_vec(),
            permission.request.root.to_vec(),
        ],
    };
    let before = f.status();
    let observed = f.roots(f.operator, &query).unwrap();
    assert!(
        matches!(observed[0], RootObservation::Known(v) if v.request == permission.request && v.state == ContentState::ExposurePossible)
    );
    assert_eq!(observed[1], RootObservation::Unknown);
    assert_eq!(observed[2], RootObservation::Malformed { bytes: 0 });
    assert!(matches!(observed[3], RootObservation::Known(v) if v.state == ContentState::Cancelled));
    assert_eq!(observed[0], observed[4]);
    let empty = RootBatchInput {
        roots: vec![],
        ..query.clone()
    };
    assert_eq!(f.roots(f.operator, &empty), Ok(vec![]));
    for actor in [f.tenant, f.uploader, f.controller, f.other] {
        for input in [&query, &empty] {
            assert_eq!(f.roots(actor, input), Err(Failure::Denied));
        }
    }
    for changed in [
        RootBatchInput {
            service: f.other,
            ..empty.clone()
        },
        RootBatchInput {
            namespace: 2,
            ..empty
        },
    ] {
        assert_eq!(f.roots(f.operator, &changed), Err(Failure::Binding));
    }
    for roots in [vec![vec![]; 9], vec![vec![1; 257]]] {
        assert_eq!(
            f.roots(
                f.operator,
                &RootBatchInput {
                    roots,
                    ..query.clone()
                }
            ),
            Err(Failure::Capacity)
        );
    }
    assert_eq!(f.status(), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.roots(f.operator, &query), Ok(observed));
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
}
