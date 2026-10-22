//! Actual caller isolation and revocation over substituted transient upload facts.
#![cfg(not(target_family = "wasm"))]

mod support;
use blob_test_protocol::uploads::UploadProbeState;
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_platform::pocket_ic::{CanisterSettings, PocketIc};
use ic_testkit::fake_principal;
use support::{Harness, fixture_path};

fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    crate::support::query_fixture(pic, canister, caller, "upload_usage", ())
        .map(|(reply,)| reply)
        .expect("usage")
}
fn active(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    crate::support::query_fixture(pic, canister, caller, "active_uploads", ())
        .map(|(reply,)| reply)
        .expect("active uploads")
}
fn roots(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<UploadProbeState>> {
    crate::support::query_fixture(pic, canister, caller, "upload_roots", ())
        .map(|(reply,)| reply)
        .expect("root observations")
}
fn cancel(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    crate::support::update_fixture(pic, canister, caller, "cancel_upload", (root,))
        .map(|(reply,)| reply)
        .expect("cancel")
}

#[test]
fn real_callers_observe_only_owned_uploads_and_cannot_cancel_uncertain_work() {
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
    let observations = vec![
        UploadProbeState::Reserved,
        UploadProbeState::Registered,
        UploadProbeState::Registered,
        UploadProbeState::Cancelled,
    ];
    assert_eq!(roots(pic, canister, gateway), Some(observations));
    assert_eq!(active(pic, canister, first), Some(vec![11, 12]));
    assert_eq!(active(pic, canister, second), Some(vec![]));
    assert_eq!(usage(pic, canister, first), Some(200));
    assert_eq!(usage(pic, canister, second), Some(100));
    assert_eq!(active(pic, canister, Principal::anonymous()), None);
    assert_eq!(usage(pic, canister, Principal::anonymous()), None);
    for caller in [controller, gateway] {
        assert_eq!(active(pic, canister, caller), Some(vec![]));
        assert_eq!(usage(pic, canister, caller), Some(0));
    }
    for caller in [second, controller, gateway, Principal::anonymous()] {
        assert!(!cancel(pic, canister, caller, 11));
        assert!(!cancel(pic, canister, caller, 12));
    }
    for caller in [first, second, controller, Principal::anonymous()] {
        assert_eq!(roots(pic, canister, caller), None);
    }
    assert!(!cancel(pic, canister, first, 12)); // Real owner still cannot release uncertainty.
    assert!(!cancel(pic, canister, second, 13)); // Confirmed object's lifecycle owns release.
    assert!(!cancel(pic, canister, first, 99));
    assert!(cancel(pic, canister, first, 11));
    assert!(cancel(pic, canister, first, 11));
    assert_eq!(active(pic, canister, first), Some(vec![12]));
    assert_eq!(usage(pic, canister, first), Some(100));
    assert_eq!(usage(pic, canister, second), Some(100));
    assert_eq!(
        roots(pic, canister, gateway),
        Some(vec![
            UploadProbeState::Cancelled,
            UploadProbeState::Registered,
            UploadProbeState::Registered,
            UploadProbeState::Cancelled
        ])
    );
    let revoked: bool =
        crate::support::update_fixture(pic, canister, controller, "revoke_gateway", ())
            .map(|(reply,)| reply)
            .expect("revoke");
    assert!(revoked);
    assert_eq!(roots(pic, canister, gateway), None);
    assert_eq!(active(pic, canister, first), Some(vec![12]));
    assert_eq!(usage(pic, canister, first), Some(100));
}
