//! Actual management snapshot operations, without an intervening upgrade hook.
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::pocket_ic::PocketIc;

pub(super) async fn take(pic: &PocketIc, service: Principal, controller: Principal) -> Vec<u8> {
    pic.stop_canister(service, Some(controller)).await.unwrap();
    let snapshot = pic
        .take_canister_snapshot(service, Some(controller), None)
        .await
        .unwrap();
    pic.start_canister(service, Some(controller)).await.unwrap();
    snapshot.id
}

pub(super) async fn load(pic: &PocketIc, service: Principal, controller: Principal, id: &[u8]) {
    pic.stop_canister(service, Some(controller)).await.unwrap();
    pic.load_canister_snapshot(service, Some(controller), id.to_vec())
        .await
        .unwrap();
    pic.start_canister(service, Some(controller)).await.unwrap();
}

pub(super) async fn delete(pic: &PocketIc, service: Principal, controller: Principal, id: Vec<u8>) {
    pic.delete_canister_snapshot(service, Some(controller), id)
        .await
        .unwrap();
}
