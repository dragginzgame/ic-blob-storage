//! Standalone reference asset catalog over a separate experimental Blob host.
//! Publication is the owning user's decision, not an independent read-verification proof.
//! Durable drafts precede calls; cancellation is irreversible and cleanup uses original IDs.

pub mod protocol;
mod state;

use ic_blob_storage::{
    local::{Error as BlobError, ReferenceStatus, UploadState},
    protocol::{Command, Inspection, Observation, Outcome, RPC_LIMITS},
};
use ic_platform_runtime::{
    cdk::{
        self, api,
        call::{Call, WaitPolicy},
    },
    ic0,
};
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    StorageConfig,
};
use ic_wire::native;
use protocol::{DraftInput, DraftView, Error, Installation, Phase};
use serde::{Serialize, de::DeserializeOwned};
use state::State;
use std::{
    cell::{Cell, RefCell},
    mem::DropGuard,
    num::NonZeroU128,
};

const INIT: u64 = native::method_id("app_init");
const UPGRADE: u64 = native::method_id("app_post_upgrade");
const CREATE: u64 = native::method_id("app_create");
const PUBLISH: u64 = native::method_id("app_publish");
const CANCEL: u64 = native::method_id("app_cancel");
const DRAFT: u64 = native::method_id("app_draft");
const CATALOG: u64 = native::method_id("app_catalog");

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static IN_FLIGHT: Cell<u32> = const { Cell::new(0) };
}

#[cdk::init(decode_with = "decode::<Installation, INIT>")]
fn init(input: Installation) {
    STATE.with_borrow_mut(|state| {
        *state = Some(State::create(memory(true), api::canister_self(), input))
    });
}

#[cdk::pre_upgrade]
fn pre_upgrade() {
    assert_eq!(
        IN_FLIGHT.get(),
        0,
        "stop and drain application calls before upgrade"
    );
}

#[cdk::post_upgrade(decode_with = "decode::<(), UPGRADE>")]
fn post_upgrade() {
    STATE.with_borrow_mut(|state| *state = Some(State::open(memory(false), api::canister_self())));
}

#[cdk::update(
    guard = "admit",
    decode_with = "decode::<DraftInput, CREATE>",
    encode_with = "encode::<_, CREATE>"
)]
async fn app_create(input: DraftInput) -> Result<DraftView, Error> {
    let caller = api::msg_caller();
    let draft = STATE.with_borrow_mut(|state| state.as_mut().unwrap().save(caller, input))?;
    if draft.phase != Phase::Draft {
        return Ok(draft);
    }
    authorize(&draft).await?;
    // A concurrent cancellation may already have cleaned it. Never overwrite that decision.
    view(caller, draft.input.id)
}

#[cdk::update(
    guard = "admit",
    decode_with = "decode::<NonZeroU128, PUBLISH>",
    encode_with = "encode::<_, PUBLISH>"
)]
async fn app_publish(id: NonZeroU128) -> Result<DraftView, Error> {
    let caller = api::msg_caller();
    let draft = publishable(caller, id)?;
    if draft.phase == Phase::Published {
        return Ok(draft);
    }
    let observed: Result<Observation, BlobError> =
        call("blob_query", &Inspection::Upload(id)).await?;
    match observed.map_err(Error::Blob)? {
        Observation::Upload(upload)
            if upload.request == state::request(&draft)
                && matches!(
                    upload.state,
                    UploadState::Registered | UploadState::Attached
                ) => {}
        _ => return Err(Error::NotLive),
    }
    publishable(caller, id)?;
    let observed: Result<Observation, BlobError> =
        call("blob_query", &Inspection::Reference(id)).await?;
    match observed.map_err(Error::Blob)? {
        Observation::Reference(reference)
            if reference.root == draft.input.root && reference.status == ReferenceStatus::Live => {}
        _ => return Err(Error::NotLive),
    }
    // Current local cancellation/name ownership is decisive even if the reply says Live.
    STATE.with_borrow_mut(|state| state.as_mut().unwrap().publish(caller, id))
}

#[cdk::update(
    guard = "admit",
    decode_with = "decode::<NonZeroU128, CANCEL>",
    encode_with = "encode::<_, CANCEL>"
)]
async fn app_cancel(id: NonZeroU128) -> Result<DraftView, Error> {
    let caller = api::msg_caller();
    let draft = STATE.with_borrow_mut(|state| state.as_mut().unwrap().cancel(caller, id))?;
    if draft.phase == Phase::Cleaned {
        return Ok(draft);
    }
    // Resolve even an authorization whose reply was lost, using the exact saved declaration.
    // UnknownReference alone would not prevent a delayed authorization creating the use.
    match authorize(&draft).await {
        Ok(()) => {
            let released: Result<Outcome, BlobError> =
                call("blob_update", &Command::Release(id)).await?;
            if released.map_err(Error::Blob)? != Outcome::Done {
                return Err(Error::Uncertain);
            }
        }
        // Authorize resolves retained IDs before checking expiry. This refusal means
        // never admitted; Blob's monotonic time prevents the immutable request admitting later.
        Err(Error::Blob(BlobError::Expired)) => {}
        Err(error) => return Err(error),
    }
    STATE.with_borrow_mut(|state| state.as_mut().unwrap().cleaned(caller, id))
}

#[cdk::query(
    guard = "admit",
    decode_with = "decode::<NonZeroU128, DRAFT>",
    encode_with = "encode::<_, DRAFT>"
)]
fn app_draft(id: NonZeroU128) -> Result<DraftView, Error> {
    view(api::msg_caller(), id)
}

#[cdk::query(
    guard = "admit",
    decode_with = "decode::<(), CATALOG>",
    encode_with = "encode::<_, CATALOG>"
)]
fn app_catalog() -> Result<Vec<DraftView>, Error> {
    STATE.with_borrow(|state| state.as_ref().unwrap().catalog(api::msg_caller()))
}

fn view(caller: ic_platform_types::Principal, id: NonZeroU128) -> Result<DraftView, Error> {
    STATE.with_borrow(|state| state.as_ref().unwrap().view(caller, id))
}

fn publishable(caller: ic_platform_types::Principal, id: NonZeroU128) -> Result<DraftView, Error> {
    let draft = view(caller, id)?;
    if matches!(draft.phase, Phase::Cancelled | Phase::Cleaned) {
        return Err(Error::Cancelled);
    }
    Ok(draft)
}

async fn authorize(draft: &DraftView) -> Result<(), Error> {
    let result: Result<Outcome, BlobError> =
        call("blob_update", &state::authorization(draft)).await?;
    match result.map_err(Error::Blob)? {
        Outcome::Upload(view) if view.request == state::request(draft) => Ok(()),
        _ => Err(Error::Uncertain),
    }
}

async fn call<T: Serialize, R: DeserializeOwned>(method: &str, request: &T) -> Result<R, Error> {
    let blob = STATE.with_borrow(|state| state.as_ref().unwrap().blob());
    let bytes = native::encode_request(native::method_id(method), request, RPC_LIMITS)
        .map_err(|_| Error::Invalid)?;
    IN_FLIGHT.set(
        IN_FLIGHT
            .get()
            .checked_add(1)
            .expect("bounded active calls"),
    );
    // Runtime's protected executor drops this guard on callback cleanup as well.
    let _flight = DropGuard::new((), |()| {
        IN_FLIGHT.set(IN_FLIGHT.get().checked_sub(1).unwrap())
    });
    let reply = Call::bounded_wait(blob, method)
        .with_wait_policy(WaitPolicy::Bounded {
            timeout_seconds: 30,
        })
        .take_raw_args(bytes)
        .await
        .map_err(|_| Error::Uncertain)?;
    native::decode_response(native::method_id(method), &reply, RPC_LIMITS)
        .map_err(|_| Error::Uncertain)
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Platform guard signature; oversized transport traps"
)]
fn admit() -> Result<(), String> {
    if ic0::msg_arg_data_size() > RPC_LIMITS.bytes {
        api::trap("application request too large");
    }
    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Runtime codec hook owns ingress"
)]
fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    native::decode_request(ID, &bytes, RPC_LIMITS).unwrap()
}

fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    native::encode_response(ID, &value, RPC_LIMITS).unwrap()
}

fn memory(fresh: bool) -> RuntimeMemory<DefaultMemoryImpl> {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(
        backing.size() == 0,
        fresh,
        "fresh and existing application are distinct"
    );
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        StorageConfig {
            allocations: 1,
            extents: 1,
            pages: 16,
        },
    )
    .unwrap();
    runtime
        .bootstrap(
            &SealedDeclarationSnapshot::new(vec![
                MemoryRequest::new("catalog", "catalog.control.v1").unwrap(),
            ])
            .unwrap(),
            &AllocationGrants::new(vec![MemoryAuthority::new("catalog", "catalog.").unwrap()])
                .unwrap(),
            &GenericAllocationPolicy,
        )
        .unwrap();
    runtime
        .open_memory(&StableKey::parse("catalog.control.v1").unwrap())
        .unwrap()
}
