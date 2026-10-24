//! Actual IC caller/controller checks over a test-only transient catalog.
//! Sample objects substitute provider facts; no service/provider qualification.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
mod support;

use support::{Harness, fixture_path};

use ic_platform::pocket_ic::{CanisterSettings, PocketIc};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::fake_principal;

fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    fixture_calls::query_fixture(pic, canister, caller, "usage", ())
        .map(|(reply,)| reply)
        .expect("usage query")
}

fn live(
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
    .map(|(reply,)| reply)
    .expect("liveness query")
}

fn release(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    fixture_calls::update_fixture(pic, canister, caller, "release", (root,))
        .map(|(reply,)| reply)
        .expect("release update")
}

fn pending(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    fixture_calls::query_fixture(pic, canister, caller, "pending", ())
        .map(|(reply,)| reply)
        .expect("pending query")
}

#[test]
fn actual_callers_cannot_claim_tenant_or_controller_authority_and_revocation_takes_effect() {
    let harness = Harness::new();
    let pic = &harness.pic;
    let (first, second, gateway, controller) = (
        fake_principal(1),
        fake_principal(2),
        fake_principal(3),
        fake_principal(4),
    );
    let canister = pic.create_canister_with_settings(
        Some(controller),
        Some(CanisterSettings {
            controllers: Some(vec![controller]),
            ..CanisterSettings::default()
        }),
    );
    pic.install_canister(
        canister,
        std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).expect("fixture Wasm"),
        ic_platform::candid::contract::encode_args((first, second, gateway, controller))
            .expect("init"),
        Some(controller),
    );
    assert_eq!(usage(pic, canister, first), Some(100));
    assert_eq!(usage(pic, canister, second), Some(200));
    assert_eq!(usage(pic, canister, Principal::anonymous()), None);
    assert_eq!(usage(pic, canister, controller), Some(0));
    assert_eq!(live(pic, canister, first, 1, canister, first), Some(true));
    for caller in [second, controller, gateway, Principal::anonymous()] {
        assert_eq!(live(pic, canister, caller, 1, canister, first), None);
        assert!(!release(pic, canister, caller, 1));
    }
    // Caller-supplied service and tenant fields cannot become execution context.
    assert_eq!(live(pic, canister, second, 1, canister, second), None);
    assert_eq!(live(pic, canister, first, 1, controller, first), None);
    assert_eq!(live(pic, canister, first, 9, canister, first), None);
    assert_eq!(usage(pic, canister, first), Some(100));
    assert_eq!(pending(pic, canister, gateway), Some(vec![]));
    assert!(release(pic, canister, first, 1));
    assert!(release(pic, canister, first, 1));
    assert_eq!(live(pic, canister, first, 1, canister, first), Some(false));
    assert_eq!(usage(pic, canister, first), Some(0));
    assert_eq!(usage(pic, canister, second), Some(200));
    assert_eq!(pending(pic, canister, gateway), Some(vec![1]));
    for caller in [first, second, controller, Principal::anonymous()] {
        assert_eq!(pending(pic, canister, caller), None);
    }
    let denied: bool = fixture_calls::update_fixture(pic, canister, first, "revoke_gateway", ())
        .map(|(reply,)| reply)
        .expect("operator denial");
    assert!(!denied);
    assert_eq!(pending(pic, canister, gateway), Some(vec![1]));
    let revoked: bool =
        fixture_calls::update_fixture(pic, canister, controller, "revoke_gateway", ())
            .map(|(reply,)| reply)
            .expect("operator revocation");
    assert!(revoked);
    assert_eq!(pending(pic, canister, gateway), None);
    assert_eq!(usage(pic, canister, second), Some(200));
}
