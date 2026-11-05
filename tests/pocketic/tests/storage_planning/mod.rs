use super::*;
use blob_test_protocol::{
    admission::{ContentLookup, ContentState, input::ReferenceInput},
    storage::{
        ProviderFact,
        read::{RootBatchInput, RootObservation},
    },
};
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse;
use ic_blob_storage_contracts::dto::reference::capacity::ReferenceHeadroom;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure;
use ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;

impl Fixture {
    fn capacity_input(&self) -> TenantScope {
        TenantScope {
            service: self.service.into(),
            tenant: self.tenant.into(),
            namespace: 1,
        }
    }
    async fn capacity(
        &self,
        actor: Principal,
        input: TenantScope,
    ) -> Result<UploadCapacityResponse, UploadCapacityFailure> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                "blob_upload_capacity",
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
    async fn reference_headroom(
        &self,
        actor: Principal,
        input: ContentLookup,
    ) -> Result<Option<ReferenceHeadroom>, ReferenceCapacityFailure> {
        let request = ReferenceCapacityRequest {
            scope: TenantScope {
                service: input.service.into(),
                tenant: input.tenant.into(),
                namespace: input.namespace,
            },
            root: input.root,
        };
        let result: Result<ReferenceCapacityResponse, ReferenceCapacityFailure> = self
            .harness
            .pic
            .query_blob_as(
                self.service,
                actor,
                "blob_reference_capacity",
                request,
                REPLY_BYTES,
            )
            .await
            .unwrap();
        let response = result?;
        assert_eq!(response.request, request);
        assert_eq!(response.fenced, self.status().await.fenced);
        Ok(response.headroom)
    }
    async fn roots(
        &self,
        actor: Principal,
        input: &RootBatchInput,
    ) -> Result<Vec<RootObservation>, Failure> {
        self.harness
            .pic
            .query_control_as(self.service, actor, "observe_roots", (input,), REPLY_BYTES)
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered capacity, cleanup and fenced upgrade journey"
)]
async fn durable_headroom_preserves_cleanup_slots_and_billing_until_settlement() {
    let f = Fixture::new().await;
    let scope = f.capacity_input();
    assert_eq!(
        f.capacity(f.tenant, scope).await,
        Err(UploadCapacityFailure::NotEnrolled)
    );
    f.enroll(None, true).await.unwrap();
    let initial = f.capacity(f.tenant, scope).await.unwrap();
    assert_eq!(initial.remaining_bytes, 20);
    let (permission, preparation) = f.permission(u128::MAX, 1);
    let lookup = ContentLookup {
        service: f.service,
        tenant: f.tenant,
        namespace: 1,
        root: permission.request.root,
    };
    assert_eq!(f.reference_headroom(f.tenant, lookup).await, Ok(None));
    f.admit(f.tenant, permission).await.unwrap();
    assert_eq!(f.reference_headroom(f.tenant, lookup).await, Ok(None));
    assert_eq!(
        f.capacity(f.tenant, scope).await.unwrap().remaining_bytes,
        10
    );
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    assert_eq!(
        f.reference_headroom(f.tenant, lookup).await,
        Ok(Some(ReferenceHeadroom {
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
    .await
    .unwrap();
    assert_eq!(
        f.reference_headroom(f.tenant, lookup).await,
        Ok(Some(ReferenceHeadroom {
            reference_slots: 0,
            unreserved_receipts: 0,
            release_reserved_receipts: 2,
            fresh_retains: 0,
        }))
    );
    let active = f.tenant().await.unwrap();
    f.enroll(Some(active), false).await.unwrap();
    assert!(!f.capacity(f.tenant, scope).await.unwrap().enrollment.active);
    for (reference, operation) in [(1, 2), (2, 3)] {
        f.reference(ReferenceInput {
            object: permission.request,
            reference,
            operation,
            retain: false,
        })
        .await
        .unwrap();
    }
    let retired = ReferenceHeadroom {
        reference_slots: 0,
        unreserved_receipts: 0,
        release_reserved_receipts: 0,
        fresh_retains: 0,
    };
    assert_eq!(
        f.reference_headroom(f.tenant, lookup).await,
        Ok(Some(retired))
    );
    assert_eq!(
        f.capacity(f.tenant, scope).await.unwrap().remaining_bytes,
        10
    );
    f.fact(permission.request, ProviderFact::Deleted)
        .await
        .unwrap();
    let deleted = f.capacity(f.tenant, scope).await.unwrap();
    assert_eq!(
        deleted.remaining_logical_bytes,
        initial.remaining_logical_bytes
    );
    assert_eq!(
        deleted.remaining_physical_bytes,
        initial.remaining_physical_bytes
    );
    assert_eq!(
        deleted.remaining_liability_bytes,
        initial.remaining_liability_bytes - 10
    );
    assert_eq!(
        f.capacity(f.tenant, scope).await.unwrap().remaining_bytes,
        10
    );
    f.fact(permission.request, ProviderFact::Settled)
        .await
        .unwrap();
    let settled = f.capacity(f.tenant, scope).await.unwrap();
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
        assert_eq!(
            f.capacity(actor, scope).await,
            Err(UploadCapacityFailure::Denied)
        );
        assert_eq!(
            f.reference_headroom(actor, lookup).await,
            Err(ReferenceCapacityFailure::Denied)
        );
    }
    assert_eq!(
        f.reference_headroom(
            f.other,
            ContentLookup {
                tenant: f.other,
                ..lookup
            }
        )
        .await,
        Ok(None)
    );
    for changed in [
        TenantScope {
            service: f.other.into(),
            ..scope
        },
        TenantScope {
            namespace: 2,
            ..scope
        },
    ] {
        assert_eq!(
            f.capacity(f.tenant, changed).await,
            Err(UploadCapacityFailure::Binding)
        );
    }
    let before = f.status().await;
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(
        f.capacity(f.tenant, scope).await,
        Ok(UploadCapacityResponse {
            fenced: true,
            ..settled
        })
    );
    assert_eq!(
        f.reference_headroom(f.tenant, lookup).await,
        Ok(Some(retired))
    );
    assert_eq!(f.admit(f.tenant, permission).await, Err(Failure::Fenced));
    assert_eq!(
        f.status().await,
        Status {
            fenced: true,
            ..before
        }
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn operator_root_batches_are_bounded_authorized_and_preserved_after_upgrade() {
    let f = Fixture::new().await;
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(1, 1);
    f.admit(f.tenant, permission).await.unwrap();
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    f.revoke(permission).await.unwrap();
    let cancelled = f.permission(2, 2).0;
    f.admit(f.tenant, cancelled).await.unwrap();
    f.revoke(cancelled).await.unwrap();
    let query = RootBatchInput {
        service: f.service.into(),
        namespace: 1,
        roots: vec![
            permission.request.root.to_vec(),
            vec![9; 32],
            vec![],
            cancelled.request.root.to_vec(),
            permission.request.root.to_vec(),
        ],
    };
    let before = f.status().await;
    let observed = f.roots(f.operator, &query).await.unwrap();
    assert!(
        matches!(observed[0], RootObservation::Known(v) if v.request == permission.request && v.state == ContentState::Live)
    );
    assert_eq!(observed[1], RootObservation::Unknown);
    assert_eq!(observed[2], RootObservation::Malformed { bytes: 0 });
    assert!(matches!(observed[3], RootObservation::Known(v) if v.state == ContentState::Cancelled));
    assert_eq!(observed[0], observed[4]);
    let empty = RootBatchInput {
        roots: vec![],
        ..query.clone()
    };
    assert_eq!(f.roots(f.operator, &empty).await, Ok(vec![]));
    for actor in [f.tenant, f.uploader, f.controller, f.other] {
        for input in [&query, &empty] {
            assert_eq!(f.roots(actor, input).await, Err(Failure::Denied));
        }
    }
    for changed in [
        RootBatchInput {
            service: f.other.into(),
            ..empty.clone()
        },
        RootBatchInput {
            namespace: 2,
            ..empty
        },
    ] {
        assert_eq!(f.roots(f.operator, &changed).await, Err(Failure::Binding));
    }
    for roots in [vec![vec![]; 9], vec![vec![1; 257]]] {
        assert_eq!(
            f.roots(
                f.operator,
                &RootBatchInput {
                    roots,
                    ..query.clone()
                }
            )
            .await,
            Err(Failure::Capacity)
        );
    }
    assert_eq!(f.status().await, before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation("fixture_post_upgrade", f.operator),
            Some(f.controller),
        )
        .await
        .unwrap();
    assert_eq!(f.roots(f.operator, &query).await, Ok(observed));
    assert_eq!(
        f.status().await,
        Status {
            fenced: true,
            ..before
        }
    );
}
