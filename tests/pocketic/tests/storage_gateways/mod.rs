//! Actual IC persistence/rollback of gateway membership and sync correlation.
use super::*;
use blob_test_protocol::storage::gateways::{Action, Command, Outcome, Scope, View};
fn gateway_reply(principals: &[Principal]) -> Vec<u8> {
    candid::encode_one(principals).unwrap()
}
impl Fixture {
    fn gateway_scope(&self) -> Scope {
        Scope {
            service: self.service,
            cashier: self.operator,
            namespace: 1,
        }
    }
    fn gateways(&self, actor: Principal, action: Action) -> Result<Outcome, Failure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "fixture_gateways",
                (Command {
                    scope: self.gateway_scope(),
                    action,
                    fault: false,
                },),
            )
            .unwrap()
    }
    fn gateway_view(&self) -> View {
        self.harness
            .pic
            .query_candid_as::<Result<View, Failure>, _>(
                self.service,
                self.operator,
                "gateway_registry",
                (self.gateway_scope(),),
            )
            .unwrap()
            .unwrap()
    }
    fn gateway_begin(&self) -> u64 {
        let Outcome::Begun(token) = self.gateways(self.operator, Action::Begin).unwrap() else {
            panic!("begin result")
        };
        token
    }
    fn gateway_trap(&self, action: Action) {
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_gateways",
                candid::encode_one(Command {
                    scope: self.gateway_scope(),
                    action,
                    fault: true,
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
}
#[test]
fn durable_gateway_sync_cannot_undo_operator_edits_and_failed_lists_leave_it_pending() {
    let f = Fixture::new();
    let source = f.gateway_scope();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let old = f.gateway_begin();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: gateway_reply(&[])
            }
        ),
        Err(Failure::Invalid)
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source: Scope {
                    cashier: f.tenant,
                    ..source
                },
                reply: gateway_reply(&[f.tenant])
            }
        ),
        Err(Failure::Binding)
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: gateway_reply(&[f.tenant; 9])
            }
        ),
        Err(Failure::Capacity)
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.gateways(f.operator, Action::Remove(f.other)).unwrap();
    assert!(f.gateway_view().members.is_empty());
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: gateway_reply(&[f.other])
            }
        ),
        Err(Failure::Conflict)
    );
    let current = f.gateway_begin();
    assert_eq!(
        f.gateways(f.operator, Action::Cancel(old)),
        Err(Failure::Conflict)
    );
    f.gateways(
        f.operator,
        Action::Apply {
            token: current,
            source,
            reply: gateway_reply(&[f.other, f.tenant, f.other]),
        },
    )
    .unwrap();
    assert_eq!(f.gateway_view().members, vec![f.other, f.tenant]);
    let pending = f.gateway_begin();
    assert_eq!(
        f.gateways(f.operator, Action::Add(f.other)),
        Ok(Outcome::Changed(false))
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: pending,
                source,
                reply: gateway_reply(&[f.uploader])
            }
        ),
        Err(Failure::Conflict)
    );
    let latest = f.gateway_begin();
    f.gateways(f.operator, Action::Cancel(latest)).unwrap();
    assert_eq!(f.gateway_view().pending_sequence, None);
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: latest,
                source,
                reply: gateway_reply(&[f.uploader])
            }
        ),
        Err(Failure::Conflict)
    );
}
#[test]
fn gateway_writes_roll_back_membership_and_pending_identity_together() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let before = f.gateway_view();
    f.gateway_trap(Action::Begin);
    assert_eq!(f.gateway_view(), before);
    let token = f.gateway_begin();
    let pending = f.gateway_view();
    for action in [
        Action::Remove(f.other),
        Action::Cancel(token),
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    ] {
        f.gateway_trap(action);
        assert_eq!(f.gateway_view(), pending);
    }
    f.gateways(
        f.operator,
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    )
    .unwrap();
    let after = f.gateway_view();
    assert_eq!(after.members, vec![f.tenant]);
    assert_eq!(after.pending_sequence, None);
    assert_eq!(after.last_sequence, 1);
}
#[test]
fn gateway_scope_is_operator_only_and_restore_retains_members_and_unresolved_sync() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let token = f.gateway_begin();
    let before = f.gateway_view();
    let bytes = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.gateways(actor, Action::Remove(f.other)),
            Err(Failure::Denied)
        );
        let view: Result<View, Failure> = f
            .harness
            .pic
            .query_candid_as(f.service, actor, "gateway_registry", (f.gateway_scope(),))
            .unwrap();
        assert_eq!(view, Err(Failure::Denied));
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
        let view: Result<View, Failure> = f
            .harness
            .pic
            .query_candid_as(f.service, f.operator, "gateway_registry", (scope,))
            .unwrap();
        assert_eq!(view, Err(Failure::Binding));
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), bytes);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        f.gateway_view(),
        View {
            fenced: true,
            ..before
        }
    );
    for action in [
        Action::Begin,
        Action::Remove(f.other),
        Action::Cancel(token),
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    ] {
        assert_eq!(f.gateways(f.operator, action), Err(Failure::Fenced));
    }
    assert_eq!(f.gateway_view().pending_sequence, Some(1));
}

#[test]
fn gateway_encoded_reply_rejections_preserve_the_pending_sync_and_cannot_bypass_revocation() {
    let f = Fixture::new();
    f.gateways(f.operator, Action::Add(f.other)).unwrap();
    let token = f.gateway_begin();
    let source = f.gateway_scope();
    let before = f.gateway_view();
    let stable = f.harness.pic.get_stable_memory(f.service);
    let good = candid::encode_one(vec![f.tenant]).unwrap();
    let mut trailing = good.clone();
    trailing.push(0);
    for (reply, failure) in [
        (vec![], Failure::Invalid),
        (candid::encode_one(vec![42_u64]).unwrap(), Failure::Invalid),
        (trailing, Failure::Invalid),
        (vec![0; 2049], Failure::Capacity),
    ] {
        assert_eq!(
            f.gateways(
                f.operator,
                Action::Apply {
                    token,
                    source,
                    reply
                }
            ),
            Err(failure)
        );
        assert_eq!(f.gateway_view(), before);
        assert_eq!(f.harness.pic.get_stable_memory(f.service), stable);
    }
    assert_eq!(
        f.gateways(
            f.tenant,
            Action::Apply {
                token,
                source,
                reply: vec![]
            }
        ),
        Err(Failure::Denied)
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token,
                source: Scope {
                    cashier: f.tenant,
                    ..source
                },
                reply: vec![0; 2049]
            }
        ),
        Err(Failure::Binding)
    );
    f.gateways(
        f.operator,
        Action::Apply {
            token,
            source,
            reply: good,
        },
    )
    .unwrap();
    assert_eq!(f.gateway_view().members, vec![f.tenant]);
    let old = f.gateway_begin();
    f.gateways(f.operator, Action::Remove(f.tenant)).unwrap();
    let current = f.gateway_begin();
    let pending = f.gateway_view();
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: vec![0; 2049]
            }
        ),
        Err(Failure::Conflict)
    );
    assert_eq!(f.gateway_view(), pending);
    f.gateways(f.operator, Action::Cancel(current)).unwrap();
    assert!(f.gateway_view().members.is_empty());
}

mod callbacks;
mod transport;
