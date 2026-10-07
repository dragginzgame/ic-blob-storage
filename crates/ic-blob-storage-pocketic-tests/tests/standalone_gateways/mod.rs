//! Standalone operator removal delegates to the shared durable owner.
mod funding_assessment;
mod native_cli;
mod sync;
use super::*;
use ic_blob_storage::{
    dto::{
        gateway::{
            GatewayRevocationFailure as Error, GatewayRevocationRequest, GatewayRevocationResponse,
        },
        operator::OperatorScope,
    },
    ops::service::gateways::revocation::GATEWAY_REVOCATION_METHOD,
};
#[test]
fn standalone_gateway_revocation_checks_scope_caller_and_restore_fence() {
    let f = Fixture::new();
    let input = GatewayRevocationRequest {
        scope: f.operator_scope(),
        gateway: Fake::principal(9),
    };
    let call = |actor, input| -> Result<GatewayRevocationResponse, Error> {
        f.harness
            .pic
            .update_candid_as(f.service, actor, GATEWAY_REVOCATION_METHOD, (input,))
            .unwrap()
    };
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.tenant, f.uploader, Principal::anonymous()] {
        assert_eq!(call(actor, input), Err(Error::Denied));
    }
    for scope in [
        OperatorScope {
            service: f.tenant,
            ..input.scope
        },
        OperatorScope {
            cashier: f.tenant,
            ..input.scope
        },
        OperatorScope {
            payment_account: f.tenant,
            ..input.scope
        },
        OperatorScope {
            namespace: 1,
            ..input.scope
        },
    ] {
        assert_eq!(
            call(f.operator, GatewayRevocationRequest { scope, ..input }),
            Err(Error::Binding)
        );
    }
    for gateway in [Principal::anonymous(), Principal::management_canister()] {
        assert_eq!(
            call(f.operator, GatewayRevocationRequest { gateway, ..input }),
            Err(Error::Invalid)
        );
    }
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    let status = f.local_status(f.operator, input.scope).unwrap();
    for _ in 0..2 {
        assert_eq!(
            call(f.operator, input),
            Ok(GatewayRevocationResponse {
                request: input,
                removed: false
            })
        );
        assert_eq!(f.local_status(f.operator, input.scope).unwrap(), status);
    }
    // An absent revocation still advances read invalidation; it is a committed
    // decision, not a passive query or an automatically replayed receipt.
    assert!(
        !f.harness.pic.get_stable_memory(f.service).eq(&before),
        "revocation did not advance durable invalidation"
    );
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(call(f.operator, input), Err(Error::Fenced));
    assert!(
        f.local_status(f.operator, input.scope)
            .unwrap()
            .gateways
            .fenced
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}
