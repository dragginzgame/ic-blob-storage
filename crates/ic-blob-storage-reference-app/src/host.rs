use crate::{
    protocol::{DraftInput, DraftView, Error, Installation, Phase},
    state::{self, State},
};
use ic_blob_storage::{
    local::{Error as BlobError, ReferenceStatus, UploadState},
    protocol::{Command, Inspection, Observation, Outcome, RPC_LIMITS},
};
use ic_platform_runtime::{
    call::{Call, Outcome as CallOutcome, Wait},
    raw as ic0, system as api,
};
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    StorageConfig,
};
use ic_wire as wire;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    cell::{Cell, RefCell},
    mem::DropGuard,
    num::NonZeroU128,
};

const INIT: u64 = wire::method_id("app_init");
const UPGRADE: u64 = wire::method_id("app_post_upgrade");
const CREATE: u64 = wire::method_id("app_create");
const PUBLISH: u64 = wire::method_id("app_publish");
const CANCEL: u64 = wire::method_id("app_cancel");
const DRAFT: u64 = wire::method_id("app_draft");
const CATALOG: u64 = wire::method_id("app_catalog");

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    static IN_FLIGHT: Cell<u32> = const { Cell::new(0) };
}

/// Create the application catalog bound to the configured Blob canister.
#[ic_platform_runtime::init(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<Installation, INIT>,
    max_bytes = 65536,
    on_error = fail
)]
fn init(input: Installation) {
    STATE.with_borrow_mut(|state| {
        *state = Some(State::create(memory(true), api::canister_self(), input))
    });
}

/// Require outstanding application calls to finish before upgrade.
#[ic_platform_runtime::pre_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = empty,
    max_bytes = 0,
    on_error = fail
)]
fn pre_upgrade(_: ()) {
    assert_eq!(
        IN_FLIGHT.get(),
        0,
        "stop and drain application calls before upgrade"
    );
}

/// Reopen the existing catalog and its Blob binding.
#[ic_platform_runtime::post_upgrade(
    runtime = ic_platform_runtime,
    guard = unguarded,
    decode = decode::<(), UPGRADE>,
    max_bytes = 65536,
    on_error = fail
)]
fn post_upgrade(_: ()) {
    STATE.with_borrow_mut(|state| *state = Some(State::open(memory(false), api::canister_self())));
}

/// Save the caller’s immutable draft and request its Blob upload grant.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "app_create",
    guard = admit,
    decode = decode::<DraftInput, CREATE>,
    encode = encode::<_, CREATE>,
    max_bytes = 65536,
    on_error = fail
)]
async fn app_create(input: DraftInput) -> Result<DraftView, Error> {
    let caller = api::caller();
    let draft = STATE.with_borrow_mut(|state| state.as_mut().unwrap().save(caller, input))?;
    if draft.phase != Phase::Draft {
        return Ok(draft);
    }
    authorize(&draft).await?;
    // A concurrent cancellation may already have cleaned it. Never overwrite that decision.
    view(caller, draft.input.id)
}

/// Publish the caller’s draft in the catalog after checking its Blob grant and reference.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "app_publish",
    guard = admit,
    decode = decode::<NonZeroU128, PUBLISH>,
    encode = encode::<_, PUBLISH>,
    max_bytes = 65536,
    on_error = fail
)]
async fn app_publish(id: NonZeroU128) -> Result<DraftView, Error> {
    let caller = api::caller();
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

/// Cancel the caller’s draft and resolve its outstanding Blob reference.
#[ic_platform_runtime::update(
    runtime = ic_platform_runtime,
    name = "app_cancel",
    guard = admit,
    decode = decode::<NonZeroU128, CANCEL>,
    encode = encode::<_, CANCEL>,
    max_bytes = 65536,
    on_error = fail
)]
async fn app_cancel(id: NonZeroU128) -> Result<DraftView, Error> {
    let caller = api::caller();
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

/// Return one draft owned by the caller.
#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "app_draft",
    guard = admit,
    decode = decode::<NonZeroU128, DRAFT>,
    encode = encode::<_, DRAFT>,
    max_bytes = 65536,
    on_error = fail
)]
fn app_draft(id: NonZeroU128) -> Result<DraftView, Error> {
    view(api::caller(), id)
}

/// List the caller’s published catalog entries.
#[ic_platform_runtime::query(
    runtime = ic_platform_runtime,
    name = "app_catalog",
    guard = admit,
    decode = decode::<(), CATALOG>,
    encode = encode::<_, CATALOG>,
    max_bytes = 65536,
    on_error = fail
)]
fn app_catalog(_: ()) -> Result<Vec<DraftView>, Error> {
    STATE.with_borrow(|state| state.as_ref().unwrap().catalog(api::caller()))
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
    let bytes = wire::encode(
        request,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Request,
        },
        RPC_LIMITS,
    )
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
    let completion = Call::new(blob, method, Wait::Bounded { seconds: 30 })
        .owned_args(bytes)
        .execute()
        .await
        .map_err(|_| Error::Uncertain)?;
    let CallOutcome::Reply(reply) = completion.into_parts().0 else {
        return Err(Error::Uncertain);
    };
    wire::decode(
        &reply,
        wire::Encoding::Frame {
            method: wire::method_id(method),
            direction: wire::Direction::Response,
        },
        RPC_LIMITS,
    )
    .map_err(|_| Error::Uncertain)
}

/// Bound the request before decoding. Catalog operations enforce caller ownership.
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

fn decode<T: DeserializeOwned, const ID: u64>(bytes: &[u8]) -> Result<T, wire::Error> {
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
            directory_bytes: 4096,
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
