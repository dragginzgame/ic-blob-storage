//! Canonical query-only entry point exercised through the shared replicated caller.
use super::*;
use blob_test_protocol::storage::gateways::ReplicatedInput;
impl Fixture {
    fn replicated_input(&self, token: u64) -> ReplicatedInput {
        ReplicatedInput {
            attempt: self.transport_input(token),
            service: self.service,
            cashier: self.operator,
            max_reply_bytes: 2048,
        }
    }
    fn replicated_gateways(&self, actor: Principal, input: ReplicatedInput) -> Result<(), Failure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "fixture_replicated_gateway_transport",
                (input,),
            )
            .unwrap()
    }
}
#[test]
fn canonical_query_endpoint_accepts_replicated_execution_without_attached_cycles() {
    let f = Fixture::with_gateway_source();
    // The maintained endpoint allocates a sequence without a private fixture
    // handle. Later fixture attempts must correlate by durable sequence.
    let refreshed: Result<
        ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncResponse,
        GatewaySyncFailure,
    > = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.operator,
            "blob_sync_gateways",
            (f.revocation(f.other).scope,),
        )
        .unwrap();
    assert_eq!(refreshed.unwrap().sequence, 1);
    let token = f.gateway_begin();
    assert_eq!(token, 2);
    let source_memory = f.harness.pic.get_stable_memory(f.operator);
    let service_memory = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert_eq!(
            f.replicated_gateways(actor, f.replicated_input(token)),
            Err(Failure::Denied)
        );
    }
    for input in [
        ReplicatedInput {
            service: f.tenant,
            ..f.replicated_input(token)
        },
        ReplicatedInput {
            cashier: f.tenant,
            ..f.replicated_input(token)
        },
    ] {
        assert_eq!(
            f.replicated_gateways(f.operator, input),
            Err(Failure::Binding)
        );
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), service_memory);
    f.replicated_gateways(f.operator, f.replicated_input(token))
        .unwrap();
    assert_eq!(f.gateway_view().members, vec![f.other]);
    assert_eq!(f.gateway_view().pending_sequence, None);
    assert_eq!(f.harness.pic.get_stable_memory(f.operator), source_memory);
    assert_eq!(f.source_requests(), 0); // The scripted update endpoint was not used.
    assert_eq!(
        f.replicated_gateways(f.operator, f.replicated_input(token)),
        Err(Failure::Conflict)
    );
}
#[test]
fn replicated_query_refuses_query_execution_and_preserves_pending_on_rejection_or_size_limit() {
    let f = Fixture::with_gateway_source();
    f.gateways(f.operator, Action::Add(f.tenant)).unwrap();
    let token = f.gateway_begin();
    let before = f.harness.pic.get_stable_memory(f.service);
    let rejected: Result<(), Failure> = f
        .harness
        .pic
        .query_candid_as(
            f.service,
            f.operator,
            "fixture_nonreplicated_gateway_transport",
            (f.replicated_input(token),),
        )
        .unwrap();
    assert_eq!(rejected, Err(Failure::Phase));
    assert_eq!(
        f.replicated_gateways(
            f.operator,
            ReplicatedInput {
                max_reply_bytes: 1,
                ..f.replicated_input(token)
            }
        ),
        Err(Failure::Capacity)
    );
    f.source_mode(SourceMode::Reject);
    assert_eq!(
        f.replicated_gateways(f.operator, f.replicated_input(token)),
        Err(Failure::Transport)
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.cancel_gateway_sync(f.operator, token).unwrap();
    f.source_mode(SourceMode::Valid);
    let next = f.gateway_begin();
    f.replicated_gateways(f.operator, f.replicated_input(next))
        .unwrap();
    assert_eq!(f.gateway_view().members, vec![f.other]);
}
#[test]
fn replicated_query_callback_trap_preserves_pending_and_restored_transport_stays_fenced() {
    let f = Fixture::with_gateway_source();
    f.gateways(f.operator, Action::Add(f.tenant)).unwrap();
    let token = f.gateway_begin();
    let before = f.gateway_view();
    let mut input = f.replicated_input(token);
    input.attempt.callback_fault = true;
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_replicated_gateway_transport",
            candid::encode_one(input).unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.gateway_view(), before);
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
        f.replicated_gateways(f.operator, f.replicated_input(token)),
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.gateway_view(),
        View {
            fenced: true,
            ..before
        }
    );
}
