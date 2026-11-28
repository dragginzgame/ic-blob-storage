//! Local provider candidate: callbacks and a certified-ingress reply, with no provider calls.
//! Never deploy this qualification artifact on a live funded owner.

mod external;

mod host {
    use crate::external;
    use ic_blob_storage::{
        caffeine::certificate,
        local::{
            Error,
            persistence::{
                self, Service,
                callbacks::{self, Callbacks},
            },
        },
        metrics::{CommandKind, MetricsStorage},
        protocol::{Command, Inspection, Installation, Observation, Outcome, RPC_LIMITS},
    };
    use ic_logging::{EntryInput, Level, Limits, Log};
    use ic_platform_runtime::{raw as ic0, system as api};
    use ic_platform_types::Principal;
    use ic_storage::{
        AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
        MemoryRequest, MemoryRuntime, SealedDeclarationSnapshot, StableKey, StorageConfig,
    };
    use ic_wire as wire;
    use serde::{Serialize, de::DeserializeOwned};
    use std::{cell::RefCell, rc::Rc};

    // init
    // Install the probe with its Blob configuration and callback gateway.
    // limits : 65,536 request bytes, no reply
    #[ic_platform_runtime::init(
        runtime = ic_platform_runtime,
        guard = unguarded,
        decode = decode::<(Installation, Principal), INIT>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn init((input, gateway): (Installation, Principal)) {
        let runtime = memory(true);
        let service = Service::create_with_metrics(
            runtime
                .open_memory(&StableKey::parse(persistence::STORAGE_KEY).unwrap())
                .unwrap(),
            api::canister_self(),
            input,
            METRICS.with(Rc::clone),
        )
        .unwrap()
        .with_logging(record);
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

    // pre_upgrade
    // Require drained admission before upgrading the probe.
    // limits : no request bytes or reply
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

    // post_upgrade
    // Reopen the existing control records and callback binding.
    // limits : 65,536 request bytes, no reply
    #[ic_platform_runtime::post_upgrade(
        runtime = ic_platform_runtime,
        guard = unguarded,
        decode = decode::<(), UPGRADE>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn post_upgrade(_: ()) {
        let runtime = memory(false);
        let service = Service::open_with_metrics(
            runtime
                .open_memory(&StableKey::parse(persistence::STORAGE_KEY).unwrap())
                .unwrap(),
            api::canister_self(),
            METRICS.with(Rc::clone),
        )
        .unwrap()
        .with_logging(record);
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

    // blob_update
    // Apply grants and references for the application authority, registration for the bound uploader, or maintenance for the manager.
    // guard  : admit checks the Wire byte budget
    // limits : RPC_LIMITS for request and reply
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

    // blob_query
    // Inspect records under the caller’s application, uploader or maintenance permissions.
    // guard  : admit checks the Wire byte budget
    // limits : RPC_LIMITS for request and reply
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

    // registration_metrics
    // Compare both heap readers for the authorized application in this installed fixture.
    // guard  : the service admits the application authority before either read
    // limits : RPC_LIMITS for request and reply
    #[ic_platform_runtime::query(
        runtime = ic_platform_runtime,
        name = "probe_registration_metrics",
        guard = unguarded,
        decode = decode::<(), METRICS_QUERY>,
        encode = encode::<_, METRICS_QUERY>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn registration_metrics(_: ()) -> Result<([u128; 5], u64), Error> {
        STATE.with_borrow(|state| {
            let service = state.as_ref().unwrap().service();
            service.inspect(api::caller(), Inspection::Usage)?;

            let report = service.metrics().unwrap();
            assert_eq!(report, METRICS.with(|storage| storage.snapshot()));
            let counts = report.commands(CommandKind::Register);

            Ok((
                [
                    counts.succeeded,
                    counts.denied,
                    counts.capacity,
                    counts.expired,
                    counts.other_refusals,
                ]
                .map(|counter| counter.total()),
                report.registered_bytes.aggregate().samples(),
            ))
        })
    }

    // log_messages
    // Read the last eight probe messages in append order.
    // guard  : the service admits the manager before reading the log
    // limits : RPC_LIMITS for request and reply
    #[ic_platform_runtime::query(
        runtime = ic_platform_runtime,
        name = "probe_log_messages",
        guard = unguarded,
        decode = decode::<(), LOG_QUERY>,
        encode = encode::<_, LOG_QUERY>,
        max_bytes = 65536,
        on_error = fail
    )]
    fn log_messages(_: ()) -> Result<Vec<String>, Error> {
        STATE.with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .service()
                .inspect(api::caller(), Inspection::Maintenance)
        })?;

        Ok(LOG.with_borrow(|log| {
            log.checkpoint()
                .entries()
                .map(|row| row.message.clone())
                .collect()
        }))
    }

    // issue
    // Register one canonical SHA-256 root for its bound uploader and return the certificate response record.
    // guard  : admit_callback checks the Candid byte budget
    // limits : 4,096 request bytes, 1,024 reply bytes
    #[ic_platform_runtime::update(
        runtime = ic_platform_runtime,
        name = "_immutableObjectStorageCreateCertificate",
        guard = admit_callback,
        decode = certificate::decode_request,
        encode = certificate::encode_reply,
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

    // liveness
    // Report retention decisions for up to 64 roots to any caller.
    // guard  : admit_callback checks the Candid byte budget
    // limits : 4,096 bytes each for request and reply
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

    // candidates
    // Return release candidates to the configured gateway.
    // guard  : admit_callback checks the Candid byte budget
    // limits : 4,096 bytes each for request and reply
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

    // acknowledge
    // Record the configured gateway’s deletion acknowledgment for up to 64 roots.
    // guard  : admit_callback checks the Candid byte budget
    // limits : 4,096 bytes each for request and reply
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

    const INIT: u64 = wire::method_id("callback_init");
    const UPGRADE: u64 = wire::method_id("callback_upgrade");
    const UPDATE: u64 = wire::method_id("blob_update");
    const QUERY: u64 = wire::method_id("blob_query");

    const METRICS_QUERY: u64 = wire::method_id("probe_registration_metrics");

    const LOG_QUERY: u64 = wire::method_id("probe_log_messages");

    thread_local! {
        static LOG: RefCell<Log> = const { RefCell::new(Log::new(Limits {
            max_entries: std::num::NonZeroUsize::new(8).unwrap(),
            max_label_bytes: 16,
            max_message_bytes: 64,
            max_text_bytes: 512,
        })) };
        static METRICS: Rc<MetricsStorage> = Rc::default();
        static STATE: RefCell<Option<Callbacks<DefaultMemoryImpl>>> = const { RefCell::new(None) };
    }

    fn record(level: Level, source: &'static str, message: &str) {
        LOG.with_borrow_mut(|log| {
            let _ = log.append(EntryInput {
                created_at_secs: api::time() / 1_000_000_000,
                level,
                source,
                topic: None,
                message,
            });
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
                &AllocationGrants::new(vec![MemoryAuthority::new("blob", "blob.").unwrap()])
                    .unwrap(),
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

        ic_blob_storage::protocol::frame::decode(
            bytes,
            wire::Encoding::Frame {
                method: ID,
                direction: wire::Direction::Request,
            },
            RPC_LIMITS,
        )
    }

    fn encode<T: Serialize, const ID: u64>(value: T) -> Result<Vec<u8>, wire::Error> {
        ic_blob_storage::protocol::frame::encode(
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
}
