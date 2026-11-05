//! Standalone operator removal delegates to the shared durable owner.
use ic_blob_storage_pocketic_tests::fixture_calls::REPLY_BYTES;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod callbacks;
mod funding_assessment;
mod native_cli;
mod sync;
use super::*;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure as Error;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest;
use ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::protocol::GATEWAY_REVOCATION_METHOD;
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_gateway_revocation_checks_scope_caller_and_restore_fence() {
    let f = Fixture::new().await;
    let input = GatewayRevocationRequest {
        scope: f.operator_scope(),
        gateway: fake_principal(9).into(),
    };
    let call = async |actor, input| -> Result<GatewayRevocationResponse, Error> {
        f.harness
            .pic
            .update_blob_as(
                f.service,
                actor,
                GATEWAY_REVOCATION_METHOD,
                input,
                REPLY_BYTES,
            )
            .await
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(call(actor, input).await, Err(Error::Denied));
    }
    for scope in [
        OperatorScope {
            service: f.tenant.into(),
            ..input.scope
        },
        OperatorScope {
            cashier: f.tenant.into(),
            ..input.scope
        },
        OperatorScope {
            payment_account: f.tenant.into(),
            ..input.scope
        },
        OperatorScope {
            namespace: 1,
            ..input.scope
        },
    ] {
        assert_eq!(
            call(f.operator, GatewayRevocationRequest { scope, ..input }).await,
            Err(Error::Binding)
        );
    }
    for gateway in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            call(
                f.operator,
                GatewayRevocationRequest {
                    gateway: gateway.into(),
                    ..input
                }
            )
            .await,
            Err(Error::Invalid)
        );
    }
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    let status = f.local_status(f.operator, input.scope).await.unwrap();
    for _ in 0..2 {
        assert_eq!(
            call(f.operator, input).await,
            Ok(GatewayRevocationResponse {
                request: input,
                removed: false
            })
        );
        assert_eq!(
            f.local_status(f.operator, input.scope).await.unwrap(),
            status
        );
    }
    // No member or pending sync exists to remove. The retired read-session
    // generation no longer makes an otherwise unchanged decision write state.
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &before,
    );
    f.upgrade(ic_blob_storage_contracts::wire::encode_request("blob_post_upgrade", ()).unwrap())
        .await
        .unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service).await.unwrap();
    assert_eq!(call(f.operator, input).await, Err(Error::Fenced));
    assert!(
        f.local_status(f.operator, input.scope)
            .await
            .unwrap()
            .gateways
            .fenced
    );
    unchanged(
        &f.harness.pic.get_stable_memory(f.service).await.unwrap(),
        &restored,
    );
}
