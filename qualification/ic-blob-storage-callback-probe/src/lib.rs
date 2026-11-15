//! Local provider candidate: callbacks and a certified-ingress reply, with no provider calls.
//! Never deploy this qualification artifact on a live funded owner.

mod external;

use ic_blob_storage::{
    local::{
        Error,
        persistence::{
            self, Service,
            callbacks::{self, Callbacks},
        },
    },
    protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
};
use ic_platform_runtime::{
    cdk::{self, api},
    ic0,
};
use ic_platform_types::Principal;
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, SealedDeclarationSnapshot, StableKey, StorageConfig,
};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::cell::RefCell;

const INIT: u64 = wire::method_id("callback_init");
const UPGRADE: u64 = wire::method_id("callback_upgrade");
const UPDATE: u64 = wire::method_id("blob_update");
const QUERY: u64 = wire::method_id("blob_query");

thread_local! {
    static STATE: RefCell<Option<Callbacks<DefaultMemoryImpl>>> = const { RefCell::new(None) };
}

#[cdk::init(decode_with = "decode::<(Installation, Principal), INIT>")]
fn init((input, gateway): (Installation, Principal)) {
    let runtime = memory(true);
    let service = Service::create(
        runtime
            .open_memory(&StableKey::parse(persistence::STORAGE_KEY).unwrap())
            .unwrap(),
        api::canister_self(),
        input,
    )
    .unwrap();
    let callbacks = Callbacks::create(
        service,
        runtime
            .open_memory(&StableKey::parse(callbacks::STORAGE_KEY).unwrap())
            .unwrap(),
        gateway,
    )
    .unwrap();
    STATE.with_borrow_mut(|state| *state = Some(callbacks));
}

#[cdk::pre_upgrade]
fn pre_upgrade() {
    STATE.with_borrow(|state| {
        assert!(
            state.as_ref().unwrap().service().is_drained(),
            "drain before upgrade"
        )
    });
}

#[cdk::post_upgrade(decode_with = "decode::<(), UPGRADE>")]
fn post_upgrade() {
    let runtime = memory(false);
    let service = Service::open(
        runtime
            .open_memory(&StableKey::parse(persistence::STORAGE_KEY).unwrap())
            .unwrap(),
        api::canister_self(),
    )
    .unwrap();
    assert!(service.is_drained(), "retain closed admission");
    let callbacks = Callbacks::open(
        service,
        runtime
            .open_memory(&StableKey::parse(callbacks::STORAGE_KEY).unwrap())
            .unwrap(),
    )
    .unwrap();
    STATE.with_borrow_mut(|state| *state = Some(callbacks));
}

#[cdk::update(
    guard = "admit",
    decode_with = "decode::<Command, UPDATE>",
    encode_with = "encode::<_, UPDATE>"
)]
fn blob_update(command: Command) -> Result<Outcome, Error> {
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::msg_caller(), command, api::time())
    })
}

#[cdk::query(
    guard = "admit",
    decode_with = "decode::<Inspection, QUERY>",
    encode_with = "encode::<_, QUERY>"
)]
fn blob_query(input: Inspection) -> Result<Observation, Error> {
    STATE.with_borrow(|state| {
        state
            .as_ref()
            .unwrap()
            .service()
            .inspect(api::msg_caller(), input)
    })
}

#[cdk::update(
    name = "_immutableObjectStorageCreateCertificate",
    guard = "external::admit",
    decode_with = "external::certificate_root",
    encode_with = "external::certificate_reply"
)]
fn issue(root: [u8; 32]) -> [u8; 32] {
    // No await: actual caller, original grant, first reference and exposure are
    // committed by Service in the same execution that returns the provider record.
    // A reply-encoding trap must roll back this execution, not return an error record.
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::msg_caller(), Command::Register(root), api::time())
            .unwrap()
    });
    root
}

#[cdk::query(
    name = "_immutableObjectStorageBlobsAreLive",
    guard = "external::admit",
    decode_with = "external::roots",
    encode_with = "external::liveness"
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers decoded callback arguments"
)]
fn liveness(roots: Vec<[u8; 32]>) -> Vec<bool> {
    STATE.with_borrow(|state| state.as_ref().unwrap().liveness(&roots).unwrap())
}

#[cdk::query(
    name = "_immutableObjectStorageBlobsToDelete",
    guard = "external::admit",
    decode_with = "external::empty",
    encode_with = "external::candidates"
)]
fn candidates(_: ()) -> Vec<[u8; 32]> {
    STATE.with_borrow(|state| {
        state
            .as_ref()
            .unwrap()
            .candidates(api::msg_caller())
            .unwrap()
    })
}

#[cdk::update(
    name = "_immutableObjectStorageConfirmBlobDeletion",
    guard = "external::admit",
    decode_with = "external::roots",
    encode_with = "external::done"
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers decoded callback arguments"
)]
fn acknowledge(roots: Vec<[u8; 32]>) {
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .acknowledge(api::msg_caller(), &roots)
            .unwrap()
    });
}

fn memory(fresh: bool) -> MemoryRuntime<DefaultMemoryImpl> {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(backing.size() == 0, fresh);
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        StorageConfig {
            allocations: 2,
            extents: 2,
            pages: 32,
        },
    )
    .unwrap();
    runtime
        .bootstrap(
            &SealedDeclarationSnapshot::new(vec![
                MemoryRequest::new("blob", persistence::STORAGE_KEY).unwrap(),
                MemoryRequest::new("blob", callbacks::STORAGE_KEY).unwrap(),
            ])
            .unwrap(),
            &AllocationGrants::new(vec![MemoryAuthority::new("blob", "blob.").unwrap()]).unwrap(),
            &GenericAllocationPolicy,
        )
        .unwrap();
    runtime
}

fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > RPC_LIMITS.bytes {
        Err("Wire input limit".into())
    } else {
        Ok(())
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime owns the argument buffer"
)]
fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    admit().unwrap();
    wire::decode(
        &bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    wire::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .unwrap()
}
