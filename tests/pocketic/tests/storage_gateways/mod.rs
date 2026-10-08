//! Actual IC persistence/rollback of gateway membership and sync correlation.
use super::*;
use blob_test_protocol::storage::gateways::{Action, Command, Outcome, Scope, View};
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation;
use ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::protocol::GATEWAY_REVOCATION_METHOD;
fn gateway_reply(principals: &[Principal]) -> Vec<u8> {
    candid::encode_one(principals).unwrap()
}
impl Fixture {
    fn cancellation(&self, sequence: u64) -> GatewaySyncCancellation {
        GatewaySyncCancellation {
            scope: self.revocation(self.other).scope,
            sequence,
        }
    }
    fn cancel_gateway_sync(
        &self,
        actor: Principal,
        sequence: u64,
    ) -> Result<(), GatewaySyncFailure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "blob_cancel_gateway_sync",
                (self.cancellation(sequence),),
            )
            .unwrap()
    }
    fn cancel_gateway_sync_trap(&self, sequence: u64) {
        let request = blob_test_protocol::storage::gateways::FaultCancellation {
            request: self.cancellation(sequence),
            fault: true,
        };
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_cancel_gateway_sync",
                candid::encode_one(request).unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
    fn revocation(&self, gateway: Principal) -> GatewayRevocationRequest {
        GatewayRevocationRequest {
            scope: OperatorScope {
                service: self.service,
                namespace: 1,
                cashier: self.operator,
                payment_account: self.service,
            },
            gateway,
        }
    }
    fn revoke_gateway(
        &self,
        actor: Principal,
        gateway: Principal,
    ) -> Result<GatewayRevocationResponse, GatewayRevocationFailure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                GATEWAY_REVOCATION_METHOD,
                (self.revocation(gateway),),
            )
            .unwrap()
    }
    fn revoke_gateway_trap(&self, gateway: Principal) {
        let input = blob_test_protocol::storage::gateways::FaultRevocation {
            request: self.revocation(gateway),
            fault: true,
        };
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_revoke_gateway",
                candid::encode_one(input).unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
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
    f.revoke_gateway(f.operator, f.other).unwrap();
    assert_eq!(f.gateway_view().members, []);
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
        f.cancel_gateway_sync(f.operator, old),
        Err(GatewaySyncFailure::Conflict)
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
    f.cancel_gateway_sync(f.operator, latest).unwrap();
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
    f.revoke_gateway_trap(f.other);
    assert_eq!(f.gateway_view(), pending);
    f.cancel_gateway_sync_trap(token);
    assert_eq!(f.gateway_view(), pending);
    f.gateway_trap(Action::Apply {
        token,
        source: f.gateway_scope(),
        reply: gateway_reply(&[f.tenant]),
    });
    assert_eq!(f.gateway_view(), pending);
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
            f.revoke_gateway(actor, f.other),
            Err(GatewayRevocationFailure::Denied)
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
            Fixture::installation(f.operator),
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
    assert_eq!(
        f.revoke_gateway(f.operator, f.other),
        Err(GatewayRevocationFailure::Fenced)
    );
    assert_eq!(
        f.cancel_gateway_sync(f.operator, token),
        Err(GatewaySyncFailure::Fenced)
    );
    for action in [
        Action::Begin,
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
    f.revoke_gateway(f.operator, f.tenant).unwrap();
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
    f.cancel_gateway_sync(f.operator, current).unwrap();
    assert_eq!(f.gateway_view().members, []);
}

mod callbacks;
mod transport;
