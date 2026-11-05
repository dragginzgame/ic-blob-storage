//! Actual caller isolation and revocation over substituted transient upload facts.
#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
use ic_blob_storage_pocketic_tests::installation;
mod support;
use blob_test_protocol::uploads::UploadProbeState;
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::PocketIc;
use support::Harness;

async fn usage(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<u128> {
    fixture_calls::query_fixture(pic, canister, caller, "upload_usage", ())
        .await
        .map(|(reply,)| reply)
        .expect("usage")
}
async fn active(pic: &PocketIc, canister: Principal, caller: Principal) -> Option<Vec<u8>> {
    fixture_calls::query_fixture(pic, canister, caller, "active_uploads", ())
        .await
        .map(|(reply,)| reply)
        .expect("active uploads")
}
async fn roots(
    pic: &PocketIc,
    canister: Principal,
    caller: Principal,
) -> Option<Vec<UploadProbeState>> {
    fixture_calls::query_fixture(pic, canister, caller, "upload_roots", ())
        .await
        .map(|(reply,)| reply)
        .expect("root observations")
}
async fn cancel(pic: &PocketIc, canister: Principal, caller: Principal, root: u8) -> bool {
    fixture_calls::update_fixture(pic, canister, caller, "cancel_upload", (root,))
        .await
        .map(|(reply,)| reply)
        .expect("cancel")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn real_callers_observe_only_owned_reservations_and_cannot_cancel_registered_work() {
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
    let observations = vec![
        UploadProbeState::Reserved,
        UploadProbeState::Registered,
        UploadProbeState::Registered,
        UploadProbeState::Cancelled,
    ];
    assert_eq!(roots(pic, canister, gateway).await, Some(observations));
    assert_eq!(active(pic, canister, first).await, Some(vec![11]));
    assert_eq!(active(pic, canister, second).await, Some(vec![]));
    assert_eq!(usage(pic, canister, first).await, Some(200));
    assert_eq!(usage(pic, canister, second).await, Some(100));
    assert_eq!(active(pic, canister, Principal::anonymous()).await, None);
    assert_eq!(usage(pic, canister, Principal::anonymous()).await, None);
    for caller in [controller, gateway] {
        assert_eq!(active(pic, canister, caller).await, Some(vec![]));
        assert_eq!(usage(pic, canister, caller).await, Some(0));
    }
    for caller in [second, controller, gateway, Principal::anonymous()] {
        assert!(!cancel(pic, canister, caller, 11).await);
        assert!(!cancel(pic, canister, caller, 12).await);
    }
    for caller in [first, second, controller, Principal::anonymous()] {
        assert_eq!(roots(pic, canister, caller).await, None);
    }
    assert!(!cancel(pic, canister, first, 12).await); // Registered references require lifecycle release.
    assert!(!cancel(pic, canister, second, 13).await); // The same rule applies to either tenant.
    assert!(!cancel(pic, canister, first, 99).await);
    assert!(cancel(pic, canister, first, 11).await);
    assert!(cancel(pic, canister, first, 11).await);
    assert_eq!(active(pic, canister, first).await, Some(vec![]));
    assert_eq!(usage(pic, canister, first).await, Some(100));
    assert_eq!(usage(pic, canister, second).await, Some(100));
    assert_eq!(
        roots(pic, canister, gateway).await,
        Some(vec![
            UploadProbeState::Cancelled,
            UploadProbeState::Registered,
            UploadProbeState::Registered,
            UploadProbeState::Cancelled
        ])
    );
    let revoked: bool =
        fixture_calls::update_fixture(pic, canister, controller, "revoke_gateway", ())
            .await
            .map(|(reply,)| reply)
            .expect("revoke");
    assert!(revoked);
    assert_eq!(roots(pic, canister, gateway).await, None);
    assert_eq!(active(pic, canister, first).await, Some(vec![]));
    assert_eq!(usage(pic, canister, first).await, Some(100));
}
