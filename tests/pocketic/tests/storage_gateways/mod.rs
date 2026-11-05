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
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
fn gateway_reply(principals: &[Principal]) -> Vec<u8> {
    ic_platform::candid::contract::encode_one(principals).unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn new_explicit_refresh_supersedes_failed_read_and_rejects_its_late_reply_and_cancel() {
    let f = Fixture::new().await;
    let old = f.gateway_begin().await;
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source: f.gateway_scope(),
                reply: gateway_reply(&[]),
            }
        )
        .await,
        Err(Failure::Invalid)
    );
    let current = f.gateway_begin().await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source: f.gateway_scope(),
                reply: gateway_reply(&[f.other]),
            }
        )
        .await,
        Err(Failure::Conflict)
    );
    assert_eq!(
        f.cancel_gateway_sync(f.operator, old).await,
        Err(GatewaySyncFailure::Conflict)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.gateways(
        f.operator,
        Action::Apply {
            token: current,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.other]),
        },
    )
    .await
    .unwrap();
    assert_eq!(f.gateway_view().await.members, [f.other.into()]);
    assert_eq!(f.gateway_view().await.pending_sequence, None);
}
impl Fixture {
    fn cancellation(&self, sequence: u64) -> GatewaySyncCancellation {
        GatewaySyncCancellation {
            scope: self.revocation(self.other).scope,
            sequence,
        }
    }
    async fn cancel_gateway_sync(
        &self,
        actor: Principal,
        sequence: u64,
    ) -> Result<(), GatewaySyncFailure> {
        self.harness
            .pic
            .update_blob_as(
                self.service,
                actor,
                "blob_cancel_gateway_sync",
                self.cancellation(sequence),
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
    async fn cancel_gateway_sync_trap(&self, sequence: u64) {
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
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(request).unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
    fn revocation(&self, gateway: Principal) -> GatewayRevocationRequest {
        GatewayRevocationRequest {
            scope: OperatorScope {
                service: self.service.into(),
                namespace: 1,
                cashier: self.operator.into(),
                payment_account: self.service.into(),
            },
            gateway: gateway.into(),
        }
    }
    async fn revoke_gateway(
        &self,
        actor: Principal,
        gateway: Principal,
    ) -> Result<GatewayRevocationResponse, GatewayRevocationFailure> {
        self.harness
            .pic
            .update_blob_as(
                self.service,
                actor,
                GATEWAY_REVOCATION_METHOD,
                self.revocation(gateway),
                REPLY_BYTES,
            )
            .await
            .unwrap()
    }
    async fn revoke_gateway_trap(&self, gateway: Principal) {
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
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(input).unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
    fn gateway_scope(&self) -> Scope {
        Scope {
            service: self.service.into(),
            cashier: self.operator.into(),
            namespace: 1,
        }
    }
    async fn gateways(&self, actor: Principal, action: Action) -> Result<Outcome, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "fixture_gateways",
                (Command {
                    scope: self.gateway_scope(),
                    action,
                    fault: false,
                },),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
    async fn gateway_view(&self) -> View {
        self.harness
            .pic
            .query_control_as::<(Result<View, Failure>,), _>(
                self.service,
                self.operator,
                "gateway_registry",
                (self.gateway_scope(),),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
            .unwrap()
    }
    async fn gateway_begin(&self) -> u64 {
        let Outcome::Begun(token) = self.gateways(self.operator, Action::Begin).await.unwrap()
        else {
            panic!("begin result")
        };
        token
    }
    async fn gateway_trap(&self, action: Action) {
        let error = self
            .harness
            .pic
            .update_call(
                self.service,
                self.operator,
                "fixture_gateways",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(Command {
                    scope: self.gateway_scope(),
                    action,
                    fault: true,
                })
                .unwrap(),
            )
            .await
            .unwrap_err();
        assert_eq!(
            crate::canister_rejection(&error).reject_code,
            RejectCode::CanisterError
        );
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[expect(
    clippy::too_many_lines,
    reason = "One durable-sync scenario preserves stale-token and failed-reply ordering"
)]
async fn durable_gateway_sync_cannot_undo_operator_edits_and_failed_lists_leave_it_pending() {
    let f = Fixture::new().await;
    let source = f.gateway_scope();
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let old = f.gateway_begin().await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: gateway_reply(&[])
            }
        )
        .await,
        Err(Failure::Invalid)
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source: Scope {
                    cashier: f.tenant.into(),
                    ..source
                },
                reply: gateway_reply(&[f.tenant])
            }
        )
        .await,
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
        )
        .await,
        Err(Failure::Capacity)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.revoke_gateway(f.operator, f.other).await.unwrap();
    assert_eq!(f.gateway_view().await.members, []);
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: gateway_reply(&[f.other])
            }
        )
        .await,
        Err(Failure::Conflict)
    );
    let current = f.gateway_begin().await;
    assert_eq!(
        f.cancel_gateway_sync(f.operator, old).await,
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
    .await
    .unwrap();
    assert_eq!(
        f.gateway_view().await.members,
        vec![
            ic_platform::Principal::from(f.other),
            ic_platform::Principal::from(f.tenant)
        ]
    );
    let pending = f.gateway_begin().await;
    assert_eq!(
        f.gateways(f.operator, Action::Add(f.other.into())).await,
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
        )
        .await,
        Err(Failure::Conflict)
    );
    let latest = f.gateway_begin().await;
    f.cancel_gateway_sync(f.operator, latest).await.unwrap();
    assert_eq!(f.gateway_view().await.pending_sequence, None);
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: latest,
                source,
                reply: gateway_reply(&[f.uploader])
            }
        )
        .await,
        Err(Failure::Conflict)
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_writes_roll_back_membership_and_pending_identity_together() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let before = f.gateway_view().await;
    f.gateway_trap(Action::Begin).await;
    assert_eq!(f.gateway_view().await, before);
    let token = f.gateway_begin().await;
    let pending = f.gateway_view().await;
    f.revoke_gateway_trap(f.other).await;
    assert_eq!(f.gateway_view().await, pending);
    f.cancel_gateway_sync_trap(token).await;
    assert_eq!(f.gateway_view().await, pending);
    f.gateway_trap(Action::Apply {
        token,
        source: f.gateway_scope(),
        reply: gateway_reply(&[f.tenant]),
    })
    .await;
    assert_eq!(f.gateway_view().await, pending);
    f.gateways(
        f.operator,
        Action::Apply {
            token,
            source: f.gateway_scope(),
            reply: gateway_reply(&[f.tenant]),
        },
    )
    .await
    .unwrap();
    let after = f.gateway_view().await;
    assert_eq!(after.members, vec![ic_platform::Principal::from(f.tenant)]);
    assert_eq!(after.pending_sequence, None);
    assert_eq!(after.last_sequence, 1);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_scope_is_operator_only_and_restore_retains_members_and_unresolved_sync() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let token = f.gateway_begin().await;
    let before = f.gateway_view().await;
    let bytes = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.revoke_gateway(actor, f.other).await,
            Err(GatewayRevocationFailure::Denied)
        );
        let view: Result<View, Failure> = f
            .harness
            .pic
            .query_control_as(
                f.service,
                actor,
                "gateway_registry",
                (f.gateway_scope(),),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap();
        assert_eq!(view, Err(Failure::Denied));
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
        let view: Result<View, Failure> = f
            .harness
            .pic
            .query_control_as(
                f.service,
                f.operator,
                "gateway_registry",
                (scope,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap();
        assert_eq!(view, Err(Failure::Binding));
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        bytes
    );
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
        f.gateway_view().await,
        View {
            fenced: true,
            ..before
        }
    );
    assert_eq!(
        f.revoke_gateway(f.operator, f.other).await,
        Err(GatewayRevocationFailure::Fenced)
    );
    assert_eq!(
        f.cancel_gateway_sync(f.operator, token).await,
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
        assert_eq!(f.gateways(f.operator, action).await, Err(Failure::Fenced));
    }
    assert_eq!(f.gateway_view().await.pending_sequence, Some(1));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn gateway_encoded_reply_rejections_preserve_the_pending_sync_and_cannot_bypass_revocation() {
    let f = Fixture::new().await;
    f.gateways(f.operator, Action::Add(f.other.into()))
        .await
        .unwrap();
    let token = f.gateway_begin().await;
    let source = f.gateway_scope();
    let before = f.gateway_view().await;
    let stable = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let good = ic_platform::candid::contract::encode_one(vec![f.tenant]).unwrap();
    let mut trailing = good.clone();
    trailing.push(0);
    for (reply, failure) in [
        (vec![], Failure::Invalid),
        (
            ic_platform::candid::contract::encode_one(vec![42_u64]).unwrap(),
            Failure::Invalid,
        ),
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
            )
            .await,
            Err(failure)
        );
        assert_eq!(f.gateway_view().await, before);
        assert_eq!(
            f.harness.pic.get_stable_memory(f.service).await.unwrap(),
            stable
        );
    }
    assert_eq!(
        f.gateways(
            f.tenant,
            Action::Apply {
                token,
                source,
                reply: vec![]
            }
        )
        .await,
        Err(Failure::Denied)
    );
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token,
                source: Scope {
                    cashier: f.tenant.into(),
                    ..source
                },
                reply: vec![0; 2049]
            }
        )
        .await,
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
    .await
    .unwrap();
    assert_eq!(
        f.gateway_view().await.members,
        vec![ic_platform::Principal::from(f.tenant)]
    );
    let old = f.gateway_begin().await;
    f.revoke_gateway(f.operator, f.tenant).await.unwrap();
    let current = f.gateway_begin().await;
    let pending = f.gateway_view().await;
    assert_eq!(
        f.gateways(
            f.operator,
            Action::Apply {
                token: old,
                source,
                reply: vec![0; 2049]
            }
        )
        .await,
        Err(Failure::Conflict)
    );
    assert_eq!(f.gateway_view().await, pending);
    f.cancel_gateway_sync(f.operator, current).await.unwrap();
    assert_eq!(f.gateway_view().await.members, []);
}

mod callbacks;
mod transport;
