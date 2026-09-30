//! Actual management snapshot operations, without an intervening upgrade hook.
use candid::Principal;
use ic_testkit::pocket_ic::PocketIc;

pub(super) fn take(pic: &PocketIc, service: Principal, controller: Principal) -> Vec<u8> {
    pic.stop_canister(service, Some(controller)).unwrap();
    let snapshot = pic
        .take_canister_snapshot(service, Some(controller), None)
        .unwrap();
    pic.start_canister(service, Some(controller)).unwrap();
    snapshot.id
}

pub(super) fn load(pic: &PocketIc, service: Principal, controller: Principal, id: &[u8]) {
    pic.stop_canister(service, Some(controller)).unwrap();
    pic.load_canister_snapshot(service, Some(controller), id.to_vec())
        .unwrap();
    pic.start_canister(service, Some(controller)).unwrap();
}

pub(super) fn delete(pic: &PocketIc, service: Principal, controller: Principal, id: Vec<u8>) {
    pic.delete_canister_snapshot(service, Some(controller), id)
        .unwrap();
}
