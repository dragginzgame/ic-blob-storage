//! Experimental standalone host. All domain and admission rules live in the library.
//! Exports only owned Wire methods; no Caffeine certificate, callback or network call.

#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(dead_code, reason = "The installed IC entrypoints call these codecs.")
)]
mod codec;

// Platform Runtime exposes IC system calls and transport only on Wasm.
#[cfg(target_arch = "wasm32")]
mod host {
    use crate::codec::{self, INIT, MANAGEMENT, QUERY, UPDATE, UPGRADE};
    use ic_blob_storage::{
        local::{
            Error,
            persistence::{STORAGE_AUTHORITY, STORAGE_KEY, Service},
        },
        protocol::{
            Command, Inspection, Installation, ManagementView, Observation, Outcome, RPC_LIMITS,
        },
    };
    use ic_platform_runtime::{raw as ic0, system as api};
    use ic_storage::{
        AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
        MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
        StorageConfig,
    };

    use std::cell::RefCell;

    // init
    // Create the fresh standalone service with the supplied authority and limits.
    // scope  : IC lifecycle
    // limits : 65,536 request bytes
    #[ic_platform_runtime::init(
        runtime = ic_platform_runtime,
        guard = codec::unguarded,
        decode = codec::decode::<Installation, INIT>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn init(input: Installation) {
        let service = Service::create(memory(true), api::canister_self(), input)
            .expect("Blob fresh installation");
        SERVICE.with_borrow_mut(|state| *state = Some(service));
    }

    // pre_upgrade
    // Require drained admission before the controller replaces the module.
    // scope  : IC lifecycle
    // limits : no arguments
    #[ic_platform_runtime::pre_upgrade(
        runtime = ic_platform_runtime,
        guard = codec::unguarded,
        decode = codec::empty,
        max_bytes = 0,
        on_error = fail
    )]
    fn pre_upgrade(_: ()) {
        SERVICE.with_borrow(|state| {
            assert!(
                state.as_ref().expect("initialized Blob").is_drained(),
                "drain before upgrade"
            )
        });
    }

    // post_upgrade
    // Reopen the existing service and verify admission remains drained.
    // scope  : IC lifecycle
    // limits : 65,536 request bytes
    #[ic_platform_runtime::post_upgrade(
        runtime = ic_platform_runtime,
        guard = codec::unguarded,
        decode = codec::decode::<(), UPGRADE>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn post_upgrade(_: ()) {
        let service =
            Service::open(memory(false), api::canister_self()).expect("Blob existing state");
        assert!(service.is_drained(), "upgrade must retain closed admission");
        SERVICE.with_borrow_mut(|state| *state = Some(service));
    }

    // blob_update
    // Apply a command under the ingress caller’s service permissions.
    // scope  : application authority for grants and references, bound uploader for registration, manager for maintenance
    // limits : 65,536 request bytes
    #[ic_platform_runtime::update(
        runtime = ic_platform_runtime,
        name = "blob_update",
        guard = admit::<UPDATE>,
        decode = codec::decode::<Command, UPDATE>,
        encode = codec::encode::<Result<Outcome, Error>, UPDATE>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn blob_update(command: Command) -> Result<Outcome, Error> {
        SERVICE.with_borrow_mut(|state| {
            state
                .as_mut()
                .expect("initialized Blob")
                .execute(api::caller(), command, api::time())
        })
    }

    // blob_query
    // Inspect the local records visible to the caller.
    // scope  : application authority for catalog inspection, bound uploader for its upload, manager for maintenance
    // limits : 65,536 request bytes
    #[ic_platform_runtime::query(
        runtime = ic_platform_runtime,
        name = "blob_query",
        guard = admit::<QUERY>,
        decode = codec::decode::<Inspection, QUERY>,
        encode = codec::encode::<Result<Observation, Error>, QUERY>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn blob_query(request: Inspection) -> Result<Observation, Error> {
        SERVICE.with_borrow(|state| {
            state
                .as_ref()
                .expect("initialized Blob")
                .inspect(api::caller(), request)
        })
    }

    // blob_management
    // Return a replicated maintenance observation to an authorized management caller.
    // scope  : configured manager
    // limits : 65,536 request bytes
    #[ic_platform_runtime::update(
        runtime = ic_platform_runtime,
        name = "blob_management",
        guard = admit::<MANAGEMENT>,
        decode = codec::decode::<(), MANAGEMENT>,
        encode = codec::encode::<Result<ManagementView, Error>, MANAGEMENT>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn blob_management(_: ()) -> Result<ManagementView, Error> {
        SERVICE.with_borrow(|state| {
            state
                .as_ref()
                .expect("initialized Blob")
                .inspect_management(api::caller())
        })
    }

    thread_local! {
        static SERVICE: RefCell<Option<Service<DefaultMemoryImpl>>> = const { RefCell::new(None) };
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

    /// Check the byte limit and method header before decoding. The service authorizes the caller.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Platform guard signature; malformed transport still traps"
    )]
    fn admit<const ID: u64>() -> Result<(), String> {
        let length = ic0::msg_arg_data_size();
        if length > RPC_LIMITS.bytes {
            api::trap("Wire request exceeds byte allowance");
        }
        let mut header = [0; 11]; // Wire's fixed request-header argument, validated below.
        if length < header.len() {
            trap(ic_wire::Error::Malformed);
        }
        // Inspect only the fixed header before Runtime allocates/copies the payload.
        ic0::msg_arg_data_copy(&mut header, 0);
        ic_wire::validate_request_header(ID, &header).unwrap_or_else(|error| trap(error));
        Ok(())
    }

    #[cold]
    fn trap(error: ic_wire::Error) -> ! {
        api::trap(&error.to_string())
    }

    fn fail(error: impl std::fmt::Debug) -> ! {
        api::trap(&format!("entrypoint failed: {error:?}"))
    }
}
