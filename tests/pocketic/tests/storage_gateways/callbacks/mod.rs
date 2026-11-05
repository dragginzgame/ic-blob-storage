//! Actual caller, live membership and restored fencing of phase-only observations.
use super::*;
use blob_test_protocol::{
    admission::{ContentState, input::ReferenceInput},
    storage::{
        ProviderFact,
        gateways::{RootView, RootsInput},
    },
};
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
impl Fixture {
    async fn gateway_roots(
        &self,
        actor: Principal,
        input: &RootsInput,
    ) -> Result<Vec<RootView>, Failure> {
        self.harness
            .pic
            .query_control_as(
                self.service,
                actor,
                "fixture_gateway_roots",
                (input,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn gateway_phase(&self, root: [u8; 32], state: ContentState) {
        let before = self
            .harness
            .pic
            .get_stable_memory(self.service)
            .await
            .unwrap();
        assert_eq!(
            self.gateway_roots(
                self.other,
                &RootsInput {
                    scope: self.gateway_scope(),
                    roots: vec![root.to_vec()],
                }
            )
            .await,
            Ok(vec![RootView::Known(state)])
        );
        assert_eq!(
            self.harness
                .pic
                .get_stable_memory(self.service)
                .await
                .unwrap(),
            before
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_root_reads_check_scope_and_caller_even_without_known_roots() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for roots in [vec![], vec![vec![9; 32]], vec![vec![]]] {
        let input = RootsInput {
            scope: f.gateway_scope(),
            roots,
        };
        for actor in [
            f.operator,
            f.controller,
            f.tenant,
            f.uploader,
            Principal::anonymous(),
        ] {
            assert_eq!(f.gateway_roots(actor, &input).await, Err(Failure::Denied));
        }
        assert!(f.gateway_roots(f.other, &input).await.is_ok());
    }
    for scope in [
        Scope {
            service: f.tenant.into(),
            ..f.gateway_scope()
        },
        Scope {
            cashier: f.tenant.into(),
            ..f.gateway_scope()
        },
        Scope {
            namespace: 2,
            ..f.gateway_scope()
        },
    ] {
        assert_eq!(
            f.gateway_roots(
                f.other,
                &RootsInput {
                    scope,
                    roots: vec![]
                }
            )
            .await,
            Err(Failure::Binding)
        );
    }
    for roots in [vec![vec![]; 9], vec![vec![1; 257]]] {
        assert_eq!(
            f.gateway_roots(
                f.other,
                &RootsInput {
                    scope: f.gateway_scope(),
                    roots
                }
            )
            .await,
            Err(Failure::Capacity)
        );
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_root_reads_preserve_uncertainty_and_every_cleanup_phase() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    f.enroll(None, true).await.unwrap();
    let (permission, preparation) = f.permission(1, 1);
    let root = permission.request.root;
    f.admit(f.tenant, permission).await.unwrap();
    f.gateway_phase(root, ContentState::Reserved).await;
    f.prepare(&preparation).await.unwrap();
    f.expose(permission.request).await.unwrap();
    f.revoke(permission).await.unwrap();
    f.gateway_phase(root, ContentState::Live).await;
    let cancelled = f.permission(2, 2).0;
    f.admit(f.tenant, cancelled).await.unwrap();
    f.revoke(cancelled).await.unwrap();
    let before = f.status().await;
    assert_eq!(
        f.gateway_roots(
            f.other,
            &RootsInput {
                scope: f.gateway_scope(),
                roots: vec![
                    root.to_vec(),
                    vec![9; 32],
                    vec![],
                    cancelled.request.root.to_vec(),
                    root.to_vec()
                ],
            }
        )
        .await,
        Ok(vec![
            RootView::Known(ContentState::Live),
            RootView::Unknown,
            RootView::Malformed { bytes: 0 },
            RootView::Known(ContentState::Cancelled),
            RootView::Known(ContentState::Live)
        ])
    );
    assert_eq!(f.status().await, before);
    f.gateway_phase(root, ContentState::Live).await;
    f.enroll(Some(f.tenant().await.unwrap()), false)
        .await
        .unwrap();
    f.gateway_phase(root, ContentState::Live).await;
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .await
    .unwrap();
    f.gateway_phase(root, ContentState::DeletionPending).await;
    f.fact(permission.request, ProviderFact::Deleted)
        .await
        .unwrap();
    f.gateway_phase(root, ContentState::ProviderDeleted).await;
    f.fact(permission.request, ProviderFact::Settled)
        .await
        .unwrap();
    f.gateway_phase(root, ContentState::Settled).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_root_reads_use_current_membership_and_refuse_restored_instances() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let input = RootsInput {
        scope: f.gateway_scope(),
        roots: vec![],
    };
    assert_eq!(f.gateway_roots(f.other, &input).await, Ok(vec![]));
    f.revoke_gateway(f.operator, f.other).await.unwrap();
    assert_eq!(f.gateway_roots(f.other, &input).await, Err(Failure::Denied));
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    assert_eq!(f.gateway_roots(f.other, &input).await, Ok(vec![]));
    let token = f.gateway_begin().await;
    f.gateways(
        f.operator,
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.uploader]),
        },
    )
    .await
    .unwrap();
    assert_eq!(f.gateway_roots(f.other, &input).await, Err(Failure::Denied));
    assert_eq!(f.gateway_roots(f.uploader, &input).await, Ok(vec![]));
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
        f.gateway_roots(f.uploader, &input).await,
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.gateway_view().await.members,
        vec![ic_platform::Principal::from(f.uploader)]
    );
}
