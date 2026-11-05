//! Canonical query-only entry point exercised through the shared replicated caller.
use super::*;
use blob_test_protocol::storage::gateways::ReplicatedInput;
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
impl Fixture {
    fn replicated_input(&self, token: u64) -> ReplicatedInput {
        ReplicatedInput {
            attempt: self.transport_input(token),
            service: self.service.into(),
            cashier: self.operator.into(),
            max_reply_bytes: 2048,
        }
    }
    async fn replicated_gateways(
        &self,
        actor: Principal,
        input: ReplicatedInput,
    ) -> Result<(), Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                actor,
                "fixture_replicated_gateway_transport",
                (input,),
                REPLY_BYTES,
            )
            .await
            .map(|(reply,)| reply)
            .unwrap()
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn canonical_query_endpoint_accepts_replicated_execution_without_attached_cycles() {
    let f = Fixture::with_gateway_source().await;
    // The maintained endpoint allocates a sequence without a private fixture
    // handle. Later fixture attempts must correlate by durable sequence.
    let refreshed: Result<
        ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncResponse,
        GatewaySyncFailure,
    > = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            "blob_sync_gateways",
            f.revocation(f.other).scope,
            REPLY_BYTES,
        )
        .await
        .unwrap();
    assert_eq!(refreshed.unwrap().sequence, 1);
    let token = f.gateway_begin().await;
    assert_eq!(token, 2);
    let source_memory = f.harness.pic.get_stable_memory(f.operator).await.unwrap();
    let service_memory = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert_eq!(
            f.replicated_gateways(actor, f.replicated_input(token))
                .await,
            Err(Failure::Denied)
        );
    }
    for input in [
        ReplicatedInput {
            service: f.tenant.into(),
            ..f.replicated_input(token)
        },
        ReplicatedInput {
            cashier: f.tenant.into(),
            ..f.replicated_input(token)
        },
    ] {
        assert_eq!(
            f.replicated_gateways(f.operator, input).await,
            Err(Failure::Binding)
        );
    }
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        service_memory
    );
    f.replicated_gateways(f.operator, f.replicated_input(token))
        .await
        .unwrap();
    assert_eq!(
        f.gateway_view().await.members,
        vec![ic_platform::Principal::from(f.other)]
    );
    assert_eq!(f.gateway_view().await.pending_sequence, None);
    assert_eq!(
        f.harness.pic.get_stable_memory(f.operator).await.unwrap(),
        source_memory
    );
    assert_eq!(f.source_requests().await, 0); // The scripted update endpoint was not used.
    assert_eq!(
        f.replicated_gateways(f.operator, f.replicated_input(token))
            .await,
        Err(Failure::Conflict)
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replicated_query_refuses_query_execution_and_preserves_pending_on_rejection_or_size_limit()
{
    let f = Fixture::with_gateway_source().await;
    f.gateways(f.operator, Action::Add(f.tenant.into()))
        .await
        .unwrap();
    let token = f.gateway_begin().await;
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    let rejected: Result<(), Failure> = f
        .harness
        .pic
        .query_control_as(
            f.service,
            f.operator,
            "fixture_nonreplicated_gateway_transport",
            (f.replicated_input(token),),
            REPLY_BYTES,
        )
        .await
        .map(|(reply,)| reply)
        .unwrap();
    assert_eq!(rejected, Err(Failure::Phase));
    assert_eq!(
        f.replicated_gateways(
            f.operator,
            ReplicatedInput {
                max_reply_bytes: 1,
                ..f.replicated_input(token)
            }
        )
        .await,
        Err(Failure::Capacity)
    );
    f.source_mode(SourceMode::Reject).await;
    assert_eq!(
        f.replicated_gateways(f.operator, f.replicated_input(token))
            .await,
        Err(Failure::Transport)
    );
    assert_eq!(
        f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        before
    );
    f.cancel_gateway_sync(f.operator, token).await.unwrap();
    f.source_mode(SourceMode::Valid).await;
    let next = f.gateway_begin().await;
    f.replicated_gateways(f.operator, f.replicated_input(next))
        .await
        .unwrap();
    assert_eq!(
        f.gateway_view().await.members,
        vec![ic_platform::Principal::from(f.other)]
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replicated_query_callback_trap_preserves_pending_and_restored_transport_stays_fenced() {
    let f = Fixture::with_gateway_source().await;
    f.gateways(f.operator, Action::Add(f.tenant.into()))
        .await
        .unwrap();
    let token = f.gateway_begin().await;
    let before = f.gateway_view().await;
    let mut input = f.replicated_input(token);
    input.attempt.callback_fault = true;
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "fixture_replicated_gateway_transport",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(input).unwrap(),
        )
        .await
        .unwrap_err();
    assert_eq!(
        crate::canister_rejection(&error).reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.gateway_view().await, before);
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
        f.replicated_gateways(f.operator, f.replicated_input(token))
            .await,
        Err(Failure::Fenced)
    );
    assert_eq!(
        f.gateway_view().await,
        View {
            fenced: true,
            ..before
        }
    );
}
