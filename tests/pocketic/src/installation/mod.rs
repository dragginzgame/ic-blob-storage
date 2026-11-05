//! Shared Blob fixture installation mechanics; scenarios own authority, budgets and initialization bytes.
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::pocket_ic::{CanisterSettings, CreateCanisterParams, PocketIc};
use std::path::PathBuf;

/// Admit the scenario-selected artifact or `PocketIC` executable path.
/// # Panics
/// Refuses a missing selection or a path which is not a file.
#[must_use]
pub fn fixture_path(variable: &str) -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os(variable).expect("run make test-pocketic to provision fixture paths"),
    );
    assert!(
        path.is_file(),
        "fixture input does not exist: {}",
        path.display()
    );
    path
}

/// Create a scenario-owned canister with an explicit controller.
/// # Panics
/// Refuses platform creation failure.
#[must_use]
pub async fn controlled_canister(pic: &PocketIc, controller: Principal) -> Principal {
    pic.create_canister_with_settings(
        Some(controller),
        Some(CanisterSettings {
            controllers: Some(vec![controller]),
            ..Default::default()
        }),
    )
    .await
    .expect("create controlled fixture")
}

/// Install selected fixture bytes without choosing authority, arguments or retry policy.
/// # Panics
/// Refuses missing artifact bytes or installation failure.
pub async fn install_fixture(
    pic: &PocketIc,
    canister: Principal,
    artifact: &str,
    arguments: Vec<u8>,
    caller: Option<Principal>,
) {
    pic.install_canister(
        canister,
        std::fs::read(fixture_path(artifact)).expect("fixture Wasm"),
        arguments,
        caller,
    )
    .await
    .expect("install fixture");
}

/// Create a fixture under its scenario's explicit cycle budget, without installing it.
/// # Panics
/// Refuses platform creation failure.
#[must_use]
pub async fn funded_canister(pic: &PocketIc, cycles: u128) -> Principal {
    pic.create_canister_with_params(
        None,
        CreateCanisterParams {
            cycles: Some(cycles),
            ..Default::default()
        },
    )
    .await
    .expect("explicit fixture cycle budget")
}
