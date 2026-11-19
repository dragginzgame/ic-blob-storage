use crate::external;
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
use ic_platform_runtime::{raw as ic0, system as api};
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

/// Install the probe with its Blob configuration and callback gateway.
#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(Installation, Principal), INIT>,
    max_bytes = 65536,
    on_error = fail
)]
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

/// Require drained admission before upgrading the probe.
#[ic_platform_runtime::pre_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 0,
    on_error = fail
)]
fn pre_upgrade(_: ()) {
    STATE.with_borrow(|state| {
        assert!(
            state.as_ref().unwrap().service().is_drained(),
            "drain before upgrade"
        )
    });
}

/// Reopen the existing control records and callback binding.
#[ic_platform_runtime::post_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(), UPGRADE>,
    max_bytes = 65536,
    on_error = fail
)]
fn post_upgrade(_: ()) {
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

/// Exercise a local Blob command with the actual caller.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "blob_update",
    guard = admit,
    decode = decode::<Command, UPDATE>,
    encode = encode::<_, UPDATE>,
    max_bytes = 65536,
    on_error = fail
)]
fn blob_update(command: Command) -> Result<Outcome, Error> {
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::caller(), command, api::time())
    })
}

/// Read the local Blob records visible to the caller.
#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "blob_query",
    guard = admit,
    decode = decode::<Inspection, QUERY>,
    encode = encode::<_, QUERY>,
    max_bytes = 65536,
    on_error = fail
)]
fn blob_query(input: Inspection) -> Result<Observation, Error> {
    STATE.with_borrow(|state| {
        state
            .as_ref()
            .unwrap()
            .service()
            .inspect(api::caller(), input)
    })
}

/// Register the bound uploader’s root and return the candidate certificate response.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageCreateCertificate",
    guard = admit_callback,
    decode = external::certificate_root,
    encode = external::certificate_reply,
    max_bytes = 4096,
    on_error = fail
)]
fn issue(root: [u8; 32]) -> [u8; 32] {
    // No await: actual caller, original grant, first reference and exposure are
    // committed by Service in the same execution that returns the provider record.
    // A reply-encoding trap must roll back this execution, not return an error record.
    STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .execute(api::caller(), Command::Register(root), api::time())
            .unwrap()
    });
    root
}

/// Report the probe’s retention decision for each requested root.
#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageBlobsAreLive",
    guard = admit_callback,
    decode = external::roots,
    encode = external::liveness,
    max_bytes = 4096,
    on_error = fail
)]
#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime transfers decoded callback arguments"
)]
fn liveness(roots: Vec<[u8; 32]>) -> Vec<bool> {
    STATE.with_borrow(|state| state.as_ref().unwrap().liveness(&roots).unwrap())
}

/// Return release candidates to the configured gateway.
#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageBlobsToDelete",
    guard = admit_callback,
    decode = external::empty,
    encode = external::candidates,
    max_bytes = 4096,
    on_error = fail
)]
fn candidates(_: ()) -> Vec<[u8; 32]> {
    STATE.with_borrow(|state| state.as_ref().unwrap().candidates(api::caller()).unwrap())
}

/// Record the gateway’s acknowledgment for candidate roots.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "_immutableObjectStorageConfirmBlobDeletion",
    guard = admit_callback,
    decode = external::roots,
    encode = external::done,
    max_bytes = 4096,
    on_error = fail
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
            .acknowledge(api::caller(), &roots)
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
            directory_bytes: 4096,
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

/// Bound the Wire request before the service authorizes the caller.
fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > RPC_LIMITS.bytes {
        Err("Wire input limit".into())
    } else {
        Ok(())
    }
}

fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
    admit().unwrap();
    wire::decode(
        bytes,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Result<Vec<u8>, wire::Error> {
    wire::encode(
        &value,
        wire::Encoding::Frame {
            method: ID,
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
}

/// Accept system lifecycle invocations.
#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime guard signature; authorization belongs to the handler"
)]
fn unguarded() -> Result<(), std::convert::Infallible> {
    Ok(())
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Runtime lifecycle decoder signature; pre-upgrade has no arguments"
)]
fn empty(_: &[u8]) -> Result<(), std::convert::Infallible> {
    Ok(())
}

fn fail(error: impl std::fmt::Debug) -> ! {
    api::trap(&format!("entrypoint failed: {error:?}"))
}

/// Bound the provider request before decoding. Each callback applies its own access policy.
fn admit_callback() -> Result<(), String> {
    if ic0::msg_arg_data_size() > external::BYTES {
        Err("callback input limit".into())
    } else {
        Ok(())
    }
}
