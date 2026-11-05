//! Actual IC caller/controller checks over a test-only transient catalog.
//! Sample objects substitute provider facts; no service/provider qualification.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::installation;
mod support;

use support::Harness;

use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::PocketIc;

async fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    fixture_calls::query_fixture(pic, canister, caller, "usage", ())
        .await
        .map(|(reply,)| reply)
        .expect("usage query")
}

async fn live(
    pic: &PocketIc,
    canister: Principal,
    caller: Principal,
    root: u8,
    service: Principal,
    tenant: Principal,
) -> Option<bool> {
    fixture_calls::query_fixture(
        pic,
        canister,
        caller,
        "reference_live",
        (root, service, tenant),
    )
    .await
    .map(|(reply,)| reply)
    .expect("liveness query")
}

async fn release(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    fixture_calls::update_fixture(pic, canister, caller, "release", (root,))
        .await
        .map(|(reply,)| reply)
        .expect("release update")
}

async fn pending(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    fixture_calls::query_fixture(pic, canister, caller, "pending", ())
        .await
        .map(|(reply,)| reply)
        .expect("pending query")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_callers_cannot_claim_tenant_or_controller_authority_and_revocation_takes_effect() {
    let harness = Harness::new().await;
    let pic = &harness.pic;
    let (first, second, gateway, controller) = (
        fake_principal(1),
        fake_principal(2),
        fake_principal(3),
        fake_principal(4),
    );
    let canister = installation::controlled_canister(pic, controller).await;
    installation::install_fixture(
        pic,
        canister,
        "BLOB_AUTHORITY_PROBE_WASM",
        blob_test_protocol::fixture_wire::encode_args(
            "fixture_init",
            (first, second, gateway, controller),
        )
        .expect("init"),
        Some(controller),
    )
    .await;
    assert_eq!(usage(pic, canister, first).await, Some(100));
    assert_eq!(usage(pic, canister, second).await, Some(200));
    assert_eq!(usage(pic, canister, Principal::anonymous()).await, None);
    assert_eq!(usage(pic, canister, controller).await, Some(0));
    assert_eq!(
        live(pic, canister, first, 1, canister, first).await,
        Some(true)
    );
    for caller in [second, controller, gateway, Principal::anonymous()] {
        assert_eq!(live(pic, canister, caller, 1, canister, first).await, None);
        assert!(!release(pic, canister, caller, 1).await);
    }
    // Caller-supplied service and tenant fields cannot become execution context.
    assert_eq!(live(pic, canister, second, 1, canister, second).await, None);
    assert_eq!(live(pic, canister, first, 1, controller, first).await, None);
    assert_eq!(live(pic, canister, first, 9, canister, first).await, None);
    assert_eq!(usage(pic, canister, first).await, Some(100));
    assert_eq!(pending(pic, canister, gateway).await, Some(vec![]));
    assert!(release(pic, canister, first, 1).await);
    assert!(release(pic, canister, first, 1).await);
    assert_eq!(
        live(pic, canister, first, 1, canister, first).await,
        Some(false)
    );
    assert_eq!(usage(pic, canister, first).await, Some(0));
    assert_eq!(usage(pic, canister, second).await, Some(200));
    assert_eq!(pending(pic, canister, gateway).await, Some(vec![1]));
    for caller in [first, second, controller, Principal::anonymous()] {
        assert_eq!(pending(pic, canister, caller).await, None);
    }
    let denied: bool = fixture_calls::update_fixture(pic, canister, first, "revoke_gateway", ())
        .await
        .map(|(reply,)| reply)
        .expect("operator denial");
    assert!(!denied);
    assert_eq!(pending(pic, canister, gateway).await, Some(vec![1]));
    let revoked: bool =
        fixture_calls::update_fixture(pic, canister, controller, "revoke_gateway", ())
            .await
            .map(|(reply,)| reply)
            .expect("operator revocation");
    assert!(revoked);
    assert_eq!(pending(pic, canister, gateway).await, None);
    assert_eq!(usage(pic, canister, second).await, Some(200));
}
