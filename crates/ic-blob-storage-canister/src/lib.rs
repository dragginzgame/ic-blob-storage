//! Experimental standalone host. All domain and admission rules live in the library.
//! Exports only owned Wire methods; no Caffeine certificate, callback or network call.

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
use ic_platform_runtime::cdk::{self, api};
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    StorageConfig,
};

thread_local! {
    static SERVICE: RefCell<Option<Service<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}

#[cdk::init(decode_with = "codec::decode::<Installation, INIT>")]
fn init(input: Installation) {
    let service = Service::create(memory(true), api::canister_self(), input)
        .expect("Blob fresh installation");
    SERVICE.with_borrow_mut(|state| *state = Some(service));
}

#[cdk::pre_upgrade]
fn pre_upgrade() {
    SERVICE.with_borrow(|state| {
        assert!(
            state.as_ref().expect("initialized Blob").is_drained(),
            "drain before upgrade"
        )
    });
}

#[cdk::post_upgrade(decode_with = "codec::decode::<(), UPGRADE>")]
fn post_upgrade() {
    let service = Service::open(memory(false), api::canister_self()).expect("Blob existing state");
    assert!(service.is_drained(), "upgrade must retain closed admission");
    SERVICE.with_borrow_mut(|state| *state = Some(service));
}

#[cdk::update(
    guard = "codec::admit::<UPDATE>",
    decode_with = "codec::decode::<Command, UPDATE>",
    encode_with = "codec::encode::<Result<Outcome, Error>, UPDATE>"
)]
fn blob_update(command: Command) -> Result<Outcome, Error> {
    SERVICE.with_borrow_mut(|state| {
        state
            .as_mut()
            .expect("initialized Blob")
            .execute(api::msg_caller(), command, api::time())
    })
}

#[cdk::query(
    guard = "codec::admit::<QUERY>",
    decode_with = "codec::decode::<Inspection, QUERY>",
    encode_with = "codec::encode::<Result<Observation, Error>, QUERY>"
)]
fn blob_query(request: Inspection) -> Result<Observation, Error> {
    SERVICE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized Blob")
            .inspect(api::msg_caller(), request)
    })
}

// Read-only, but replicated: management callers must authenticate the update outcome.
#[cdk::update(
    guard = "codec::admit::<MANAGEMENT>",
    decode_with = "codec::decode::<(), MANAGEMENT>",
    encode_with = "codec::encode::<Result<ManagementView, Error>, MANAGEMENT>"
)]
fn blob_management() -> Result<ManagementView, Error> {
    SERVICE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized Blob")
            .inspect_management(api::msg_caller())
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
