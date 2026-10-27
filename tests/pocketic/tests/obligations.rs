//! Actual caller isolation and separate release/deletion/billing observations.
//! Confirmation facts are supplied by a test operator, never a deployed provider.

#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_pocketic_tests::fixture_calls;
mod support;

use blob_test_protocol::obligations::{
    ObligationProbeFact, ObligationProbePhase, ObligationProbeView,
};
use ic_platform::principal::ExternalPrincipal as Principal;

use ic_platform::pocket_ic::{CanisterSettings, PocketIc};
use ic_testkit::fake_principal;
use support::{Harness, fixture_path};

fn objects(
    pic: &PocketIc,
    canister: Principal,
    caller: Principal,
) -> Option<Vec<ObligationProbeView>> {
    fixture_calls::query_fixture(pic, canister, caller, "unsettled_objects", ())
        .map(|(reply,)| reply)
        .expect("tenant query")
}
fn confirm(
    pic: &PocketIc,
    canister: Principal,
    caller: Principal,
    fact: ObligationProbeFact,
) -> bool {
    fixture_calls::update_fixture(pic, canister, caller, "confirm_obligation", (1_u8, fact))
        .map(|(reply,)| reply)
        .expect("fixture fact")
}

fn install(
    pic: &PocketIc,
    first: Principal,
    second: Principal,
    gateway: Principal,
    controller: Principal,
) -> Principal {
    let canister = pic.create_canister_with_settings(
        Some(controller),
        Some(CanisterSettings {
            controllers: Some(vec![controller]),
            ..CanisterSettings::default()
        }),
    );
    pic.install_canister(
        canister,
        std::fs::read(fixture_path("BLOB_AUTHORITY_PROBE_WASM")).expect("Wasm"),
        blob_test_protocol::fixture_wire::encode_args(
            "fixture_init",
            (first, second, gateway, controller),
        )
        .expect("init"),
        Some(controller),
    );
    canister
}

#[test]
fn tenant_observes_billing_after_physical_deletion_without_controller_override() {
    let harness = Harness::new();
    let pic = &harness.pic;
    let (first, second, gateway, controller) = (
        fake_principal(1),
        fake_principal(2),
        fake_principal(3),
        fake_principal(4),
    );
    let canister = install(pic, first, second, gateway, controller);
    let first_live = ObligationProbeView {
        root: 1,
        phase: ObligationProbePhase::Live,
        physical_bytes: 100,
        liability_bytes: 100,
    };
    let second_live = ObligationProbeView {
        root: 2,
        phase: ObligationProbePhase::Live,
        physical_bytes: 200,
        liability_bytes: 200,
    };
    assert_eq!(objects(pic, canister, first), Some(vec![first_live]));
    assert_eq!(objects(pic, canister, second), Some(vec![second_live]));
    for actor in [controller, gateway, fake_principal(9)] {
        assert_eq!(objects(pic, canister, actor), Some(vec![]));
    }
    assert_eq!(objects(pic, canister, Principal::anonymous()), None);
    for actor in [first, second, gateway, Principal::anonymous()] {
        for fact in [
            ObligationProbeFact::Deleted,
            ObligationProbeFact::BillingStopped,
        ] {
            assert!(!confirm(pic, canister, actor, fact));
        }
    }
    assert!(!confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::Deleted
    ));
    assert_eq!(objects(pic, canister, first), Some(vec![first_live]));
    let released: bool = fixture_calls::update_fixture(pic, canister, first, "release", (1_u8,))
        .map(|(reply,)| reply)
        .expect("release");
    assert!(released);
    assert_eq!(
        objects(pic, canister, first),
        Some(vec![ObligationProbeView {
            phase: ObligationProbePhase::DeletionPending,
            ..first_live
        }])
    );
    assert!(!confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::BillingStopped
    ));
    assert!(confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::Deleted
    ));
    let deleted = ObligationProbeView {
        phase: ObligationProbePhase::ProviderDeleted,
        physical_bytes: 0,
        ..first_live
    };
    assert_eq!(objects(pic, canister, first), Some(vec![deleted]));
    let pending: Option<Vec<u8>> =
        fixture_calls::query_fixture(pic, canister, gateway, "pending", ())
            .map(|(reply,)| reply)
            .expect("deletion queue");
    assert_eq!(pending, Some(vec![]));
    assert!(confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::Deleted
    ));
    assert_eq!(objects(pic, canister, first), Some(vec![deleted]));
    assert!(confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::BillingStopped
    ));
    assert!(confirm(
        pic,
        canister,
        controller,
        ObligationProbeFact::BillingStopped
    ));
    assert_eq!(objects(pic, canister, first), Some(vec![]));
    assert_eq!(objects(pic, canister, second), Some(vec![second_live]));
    assert_eq!(objects(pic, canister, controller), Some(vec![]));
}
