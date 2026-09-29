//! Actual caller, live membership and restored fencing of phase-only observations.
use super::*;
use blob_test_protocol::{
    admission::{ContentState, input::ReferenceInput},
    storage::{
        ProviderFact,
        gateways::{RootView, RootsInput},
    },
};
impl Fixture {
    fn gateway_roots(
        &self,
        actor: Principal,
        input: &RootsInput,
    ) -> Result<Vec<RootView>, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "fixture_gateway_roots", (input,))
            .unwrap()
    }
    fn gateway_phase(&self, root: [u8; 32], state: ContentState) {
        let before = self.harness.pic.get_stable_memory(self.service);
        assert_eq!(
            self.gateway_roots(
                self.other,
                &RootsInput {
                    scope: self.gateway_scope(),
                    roots: vec![root.to_vec()],
                }
            ),
            Ok(vec![RootView::Known(state)])
        );
        assert_eq!(self.harness.pic.get_stable_memory(self.service), before);
    }
}
#[test]
fn gateway_root_reads_check_scope_and_caller_even_without_known_roots() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
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
            assert_eq!(f.gateway_roots(actor, &input), Err(Failure::Denied));
        }
        assert!(f.gateway_roots(f.other, &input).is_ok());
    }
    for scope in [
        Scope {
            service: f.tenant,
            ..f.gateway_scope()
        },
        Scope {
            cashier: f.tenant,
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
            ),
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
            ),
            Err(Failure::Capacity)
        );
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
#[test]
fn gateway_root_reads_preserve_uncertainty_and_every_cleanup_phase() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    f.enroll(None, true).unwrap();
    let (permission, preparation) = f.permission(1, 1);
    let root = permission.request.root;
    f.admit(f.tenant, permission).unwrap();
    f.gateway_phase(root, ContentState::Reserved);
    f.prepare(&preparation).unwrap();
    f.expose(permission.request).unwrap();
    f.revoke(permission).unwrap();
    f.gateway_phase(root, ContentState::ExposurePossible);
    let cancelled = f.permission(2, 2).0;
    f.admit(f.tenant, cancelled).unwrap();
    f.revoke(cancelled).unwrap();
    let before = f.status();
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
        ),
        Ok(vec![
            RootView::Known(ContentState::ExposurePossible),
            RootView::Unknown,
            RootView::Malformed { bytes: 0 },
            RootView::Known(ContentState::Cancelled),
            RootView::Known(ContentState::ExposurePossible)
        ])
    );
    assert_eq!(f.status(), before);
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    f.gateway_phase(root, ContentState::Live);
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    f.gateway_phase(root, ContentState::Live);
    f.reference(ReferenceInput {
        object: permission.request,
        reference: 1,
        operation: 1,
        retain: false,
    })
    .unwrap();
    f.gateway_phase(root, ContentState::DeletionPending);
    f.fact(permission.request, ProviderFact::Deleted).unwrap();
    f.gateway_phase(root, ContentState::ProviderDeleted);
    f.fact(permission.request, ProviderFact::Settled).unwrap();
    f.gateway_phase(root, ContentState::Settled);
}
#[test]
fn gateway_root_reads_use_current_membership_and_refuse_restored_instances() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let input = RootsInput {
        scope: f.gateway_scope(),
        roots: vec![],
    };
    assert_eq!(f.gateway_roots(f.other, &input), Ok(vec![]));
    f.revoke_gateway(f.operator, f.other).unwrap();
    assert_eq!(f.gateway_roots(f.other, &input), Err(Failure::Denied));
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    assert_eq!(f.gateway_roots(f.other, &input), Ok(vec![]));
    let token = f.gateway_begin();
    f.gateways(
        f.operator,
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.uploader]),
        },
    )
    .unwrap();
    assert_eq!(f.gateway_roots(f.other, &input), Err(Failure::Denied));
    assert_eq!(f.gateway_roots(f.uploader, &input), Ok(vec![]));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(f.gateway_roots(f.uploader, &input), Err(Failure::Fenced));
    assert_eq!(f.gateway_view().members, vec![f.uploader]);
}
