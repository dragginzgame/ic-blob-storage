//! Actual caller isolation and revocation over substituted transient upload facts.
#![cfg(not(target_family = "wasm"))]

mod support;
use blob_test_protocol::uploads::UploadProbeState;
use candid::Principal;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, PocketIc},
};
use support::{Harness, fixture_path};

fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    pic.query_candid_as(canister, caller, "upload_usage", ())
        .expect("usage")
}
fn active(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    pic.query_candid_as(canister, caller, "active_uploads", ())
        .expect("active uploads")
}
fn roots(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<UploadProbeState>> {
    pic.query_candid_as(canister, caller, "upload_roots", ())
        .expect("root observations")
}
fn cancel(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    pic.update_candid_as(canister, caller, "cancel_upload", (root,))
        .expect("cancel")
}

#[test]
fn real_callers_observe_only_owned_uploads_and_cannot_cancel_uncertain_work() {
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
    let observations = vec![
        UploadProbeState::Reserved,
        UploadProbeState::ExposurePossible,
        UploadProbeState::Confirmed,
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
            UploadProbeState::ExposurePossible,
            UploadProbeState::Confirmed,
            UploadProbeState::Cancelled
        ])
    );
    let revoked: bool = pic
        .update_candid_as(canister, controller, "revoke_gateway", ())
        .expect("revoke");
    assert!(revoked);
    assert_eq!(roots(pic, canister, gateway), None);
    assert_eq!(active(pic, canister, first), Some(vec![12]));
    assert_eq!(usage(pic, canister, first), Some(100));
}
