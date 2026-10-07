//! Actual IC caller/controller checks over a test-only transient catalog.
//! Sample objects substitute provider facts; no service/provider qualification.

#![cfg(not(target_family = "wasm"))]

mod support;

use support::{Harness, fixture_path};

use candid::Principal;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, PocketIc},
};

fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    pic.query_candid_as(canister, caller, "usage", ())
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
    pic.query_candid_as(canister, caller, "reference_live", (root, service, tenant))
        .expect("liveness query")
}

fn release(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    pic.update_candid_as(canister, caller, "release", (root,))
        .expect("release update")
}

fn pending(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    pic.query_candid_as(canister, caller, "pending", ())
        .expect("pending query")
}

#[test]
fn actual_callers_cannot_claim_tenant_or_controller_authority_and_revocation_takes_effect() {
    let harness = Harness::new();
    let pic = &harness.pic;
    let (first, second, gateway, controller) = (
        Fake::principal(1),
        Fake::principal(2),
        Fake::principal(3),
        Fake::principal(4),
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
        candid::encode_args((first, second, gateway, controller)).expect("init"),
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
    let denied: bool = pic
        .update_candid_as(canister, first, "revoke_gateway", ())
        .expect("operator denial");
    assert!(!denied);
    assert_eq!(pending(pic, canister, gateway), Some(vec![1]));
    let revoked: bool = pic
        .update_candid_as(canister, controller, "revoke_gateway", ())
        .expect("operator revocation");
    assert!(revoked);
    assert_eq!(pending(pic, canister, gateway), None);
    assert_eq!(usage(pic, canister, second), Some(200));
}
