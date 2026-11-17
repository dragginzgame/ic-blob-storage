//! Experimental standalone host. All domain and admission rules live in the library.
//! Exports only owned Wire methods; no Caffeine certificate, callback or network call.

#![cfg(target_arch = "wasm32")]

mod codec;

use codec::{INIT, MANAGEMENT, QUERY, UPDATE, UPGRADE};
use std::cell::RefCell;

use ic_blob_storage::local::persistence::{self, access::Access};
use ic_blob_storage::{
    local::{Error, persistence::Service},
    protocol::{Command, Inspection, Installation, ManagementView, Observation, Outcome},
};
use ic_platform_runtime::system as api;
use ic_storage::retained::{Bank, Canister, Geometry};

thread_local! {
    static SERVICE: RefCell<Option<Service>> = const { RefCell::new(None) };
}

#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = codec::unguarded,
    decode = codec::decode::<Installation, INIT>,
    max_bytes = 65536,
    on_error = codec::fail
)]
fn init(input: Installation) {
    let service = Service::create(memory(true), api::canister_self(), input)
        .expect("Blob fresh installation");
    SERVICE.with_borrow_mut(|state| *state = Some(service));
}

#[ic_platform_runtime::pre_upgrade(
    runtime = ic_platform_runtime,
    guard = codec::unguarded,
    decode = codec::empty,
    max_bytes = 0,
    on_error = codec::fail
)]
fn pre_upgrade(_: ()) {
    SERVICE.with_borrow(|state| {
        assert!(
            state.as_ref().expect("initialized Blob").is_drained(),
            "drain before upgrade"
        )
    });
    Canister::close().expect("close native Blob roots");
}

#[ic_platform_runtime::post_upgrade(
    runtime = ic_platform_runtime,
    guard = codec::unguarded,
    decode = codec::decode::<(), UPGRADE>,
    max_bytes = 65536,
    on_error = codec::fail
)]
fn post_upgrade(_: ()) {
    let service = Service::open(memory(false), api::canister_self()).expect("Blob existing state");
    assert!(service.is_drained(), "upgrade must retain closed admission");
    SERVICE.with_borrow_mut(|state| *state = Some(service));
}

#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "blob_update",
    guard = codec::admit::<UPDATE>,
    decode = codec::decode::<Command, UPDATE>,
    encode = codec::encode::<Result<Outcome, Error>, UPDATE>,
    max_bytes = 65536,
    on_error = codec::fail
)]
fn blob_update(command: Command) -> Result<Outcome, Error> {
    SERVICE.with_borrow_mut(|state| {
        state
            .as_mut()
            .expect("initialized Blob")
            .execute(api::caller(), command, api::time())
    })
}

#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "blob_query",
    guard = codec::admit::<QUERY>,
    decode = codec::decode::<Inspection, QUERY>,
    encode = codec::encode::<Result<Observation, Error>, QUERY>,
    max_bytes = 65536,
    on_error = codec::fail
)]
fn blob_query(request: Inspection) -> Result<Observation, Error> {
    SERVICE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized Blob")
            .inspect(api::caller(), request)
    })
}

// Read-only, but replicated: management callers must authenticate the update outcome.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "blob_management",
    guard = codec::admit::<MANAGEMENT>,
    decode = codec::decode::<(), MANAGEMENT>,
    encode = codec::encode::<Result<ManagementView, Error>, MANAGEMENT>,
    max_bytes = 65536,
    on_error = codec::fail
)]
fn blob_management(_: ()) -> Result<ManagementView, Error> {
    SERVICE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized Blob")
            .inspect_management(api::caller())
    })
}

fn memory(fresh: bool) -> Access {
    let geometry = Geometry {
        bank: Bank::A,
        nodes: 512,
        bytes: 262144,
    };
    let roots = [persistence::storage_root(1, 1)];
    if fresh {
        Canister::create(geometry, &roots)
    } else {
        Canister::reopen(geometry, &roots)
    }
    .expect("native Blob admission");
    Access::canister(Canister::grant(1).unwrap(), 1)
}
