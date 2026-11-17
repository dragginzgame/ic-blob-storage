//! Experimental standalone host. All domain and admission rules live in the library.
//! Exports only owned Wire methods; no Caffeine certificate, callback or network call.

#![cfg(target_arch = "wasm32")]

mod codec;

use codec::{INIT, MANAGEMENT, QUERY, UPDATE, UPGRADE};
use std::cell::RefCell;

use ic_blob_storage::{
    local::{
        Error,
        persistence::{STORAGE_AUTHORITY, STORAGE_KEY, Service},
    },
    protocol::{Command, Inspection, Installation, ManagementView, Observation, Outcome},
};
use ic_platform_runtime::system as api;
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    StorageConfig,
};

thread_local! {
    static SERVICE: RefCell<Option<Service<DefaultMemoryImpl>>> = const { RefCell::new(None) };
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

fn memory(fresh: bool) -> RuntimeMemory<DefaultMemoryImpl> {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(
        backing.size() == 0,
        fresh,
        "fresh install and reopen are distinct"
    );
    let declarations = SealedDeclarationSnapshot::new(vec![
        MemoryRequest::new(STORAGE_AUTHORITY, STORAGE_KEY).unwrap(),
    ])
    .unwrap();
    let grants = AllocationGrants::new(vec![
        MemoryAuthority::new(STORAGE_AUTHORITY, "blob.").unwrap(),
    ])
    .unwrap();
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        StorageConfig {
            allocations: 1,
            extents: 1,
            pages: 16,
            directory_bytes: 4096,
        },
    )
    .expect("storage ownership");
    runtime
        .bootstrap(&declarations, &grants, &GenericAllocationPolicy)
        .expect("storage admission");
    runtime
        .open_memory(&StableKey::parse(STORAGE_KEY).unwrap())
        .expect("Blob lease")
}
