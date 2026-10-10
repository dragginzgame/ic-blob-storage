//! Host grants, passive DTO conversion and fixture-only stable-write faults.
mod configuration;
mod conversion;
pub(crate) mod exposure;
pub(crate) mod funding;
pub(crate) mod gateways;
pub(crate) mod lifecycle;
pub(crate) mod planning;
pub(crate) mod read;
pub(crate) mod references;
pub(crate) mod resources;
use blob_test_protocol::{
    admission::{Permission, Request},
    storage::{Failure, Observation, Status, WriteFault},
};
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_blob_storage::ic_memory::GenericAllocationPolicy;
use ic_blob_storage::ic_memory::MemoryAllocationPool;
use ic_blob_storage::ic_memory::MemoryAuthority;
use ic_blob_storage::ic_memory::MemoryManagerConfig;
use ic_blob_storage::ic_memory::MemoryRequest;
use ic_blob_storage::ic_memory::MemoryRuntime;
use ic_blob_storage::ic_memory::RuntimeMemory;
use ic_blob_storage::ic_memory::SchemaMetadata;
use ic_blob_storage::ic_memory::SealedDeclarationSnapshot;
use ic_blob_storage::ic_memory::ic_stable_structures::DefaultMemoryImpl;
use ic_blob_storage::ic_memory::ic_stable_structures::Memory;
use ic_blob_storage::model::service::upload::UploadManifestState;
use ic_blob_storage::ops::service::stores::ServiceMemories;
use ic_blob_storage::ops::service::stores::ServiceStores;
use ic_blob_storage::ops::service::uploads::StableUploads;
use ic_blob_storage::ops::service::uploads::UploadMemories;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse;
use ic_blob_storage_contracts::dto::tenant::TenantFailure;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use std::cell::{Cell, RefCell};

#[derive(Clone, Default)]
pub(crate) struct ProbeBackingMemory(DefaultMemoryImpl);
impl Memory for ProbeBackingMemory {
    fn size(&self) -> u64 {
        self.0.size()
    }
    fn grow(&self, pages: u64) -> i64 {
        if REFUSE_GROWTH.get() {
            -1
        } else {
            self.0.grow(pages)
        }
    }
    fn read(&self, offset: u64, dst: &mut [u8]) {
        self.0.read(offset, dst);
    }
    fn write(&self, offset: u64, src: &[u8]) {
        self.0.write(offset, src);
    }
}

pub(crate) struct ProbeMemory {
    memory: RuntimeMemory<ProbeBackingMemory>,
    fault: Option<WriteFault>,
    index: usize,
}
impl Memory for ProbeMemory {
    fn size(&self) -> u64 {
        self.memory.size()
    }
    fn grow(&self, pages: u64) -> i64 {
        Memory::grow(&self.memory, pages)
    }
    fn read(&self, offset: u64, dst: &mut [u8]) {
        let started = resources::start_read();
        self.memory.read(offset, dst);
        resources::finish_read(self.index, dst.len(), started);
    }
    fn write(&self, offset: u64, src: &[u8]) {
        if self.fault.is_some() && self.fault == TRAP_WRITE.get() {
            ic_cdk::trap("fixture stable write failure");
        }
        self.memory.write(offset, src);
    }
}
struct State {
    runtime: MemoryRuntime<ProbeBackingMemory>,
    operator: Principal,
    uploads: StableUploads<ProbeMemory>,
    funding: ic_blob_storage::ops::service::funding::StableFundingJournal<ProbeMemory>,
    gateways: ic_blob_storage::ops::service::gateways::StableGatewayRegistry<ProbeMemory>,
    gateway_attempts: Vec<ic_blob_storage::ops::service::gateways::reply::GatewaySyncRequest>,
    read_sessions: ic_blob_storage::ops::service::reads::StableReadSessions<ProbeMemory>,
    resources: blob_test_protocol::storage::resources::RestorationResources,
}
thread_local! {
    static STATE:RefCell<Option<State>>=const { RefCell::new(None) };
    static TRAP_WRITE:Cell<Option<WriteFault>>=const { Cell::new(None) };
    static REFUSE_GROWTH:Cell<bool>=const { Cell::new(false) };
}
pub(crate) fn with_operator_stores<R>(
    inspect: impl FnOnce(ic_blob_storage::ops::service::operator::OperatorStores<'_, ProbeMemory>) -> R,
) -> R {
    STATE.with_borrow(|state| {
        let state = state.as_ref().expect("initialized fixture");
        inspect(ic_blob_storage::ops::service::operator::OperatorStores {
            uploads: &state.uploads,
            funding: &state.funding,
            gateways: &state.gateways,
            reads: &state.read_sessions,
        })
    })
}
pub(crate) fn initialize(
    input: blob_test_protocol::storage::resources::StorageProbeInstallation,
    restored: bool,
) {
    let started = ic_cdk::api::call_context_instruction_counter();
    let operator = input.operator;
    // Reject invalid candidates before even bootstrapping the host memory runtime.
    let config = configuration::configuration(input);
    STATE.with_borrow(|state| assert!(state.is_none(), "initialization is not reset"));
    let (runtime, memory) = granted_memories();
    let neighbor = runtime.open_memory("neighbor.data.v1").unwrap();
    if !restored {
        assert_eq!(neighbor.grow(1), Ok(0));
        neighbor.write(0, b"neighbor");
    }
    let mut bytes = [0; 8];
    neighbor.read(0, &mut bytes);
    assert_eq!(&bytes, b"neighbor");
    let stores_started = ic_cdk::api::call_context_instruction_counter();
    resources::begin_read_profile(restored);
    let ServiceStores {
        uploads,
        funding,
        gateways,
        reads: read_sessions,
    } = if restored {
        ServiceStores::open(memory, config)
    } else {
        ServiceStores::install(memory, config)
    }
    .unwrap();
    let stores_instructions = ic_cdk::api::call_context_instruction_counter() - stores_started;
    let reads = resources::finish_read_profile();
    STATE.with_borrow_mut(|state| {
        *state = Some(State {
            runtime,
            operator,
            uploads,
            funding,
            gateways,
            gateway_attempts: Vec::new(),
            read_sessions,
            resources: blob_test_protocol::storage::resources::RestorationResources {
                restored,
                initialization_instructions: 0,
                stores_instructions,
                heap_bytes: 0,
                stable_bytes: ic_cdk::api::stable_size() * 65_536,
                reads,
            },
        });
    });
    STATE.with_borrow_mut(|state| {
        let resources = &mut state.as_mut().unwrap().resources;
        resources.heap_bytes = resources::heap_bytes();
        resources.initialization_instructions =
            ic_cdk::api::call_context_instruction_counter() - started;
    });
}
const MEMORY_KEYS: [&str; 16] = [
    "fixture.tenants.v1",
    "fixture.roots.v1",
    "fixture.root_objects.v1",
    "fixture.permissions.v1",
    "fixture.usage.v1",
    "fixture.manifests.v1",
    "fixture.confirmed.v1",
    "fixture.references.v1",
    "fixture.receipts.v1",
    "fixture.root_requests.v1",
    "fixture.funding_accounting.v1",
    "fixture.funding_intents.v1",
    "fixture.gateways.v1",
    "fixture.read_journal.v1",
    "fixture.read_sessions.v1",
    "fixture.read_tenants.v1",
];
fn granted_memories() -> (
    MemoryRuntime<ProbeBackingMemory>,
    ServiceMemories<ProbeMemory>,
) {
    let mut requests = MEMORY_KEYS
        .map(|key| MemoryRequest::new("fixture", key, SchemaMetadata::default()).unwrap())
        .to_vec();
    requests.push(
        MemoryRequest::new("neighbor", "neighbor.data.v1", SchemaMetadata::default()).unwrap(),
    );
    let pool = MemoryAllocationPool::new(
        vec![
            MemoryAuthority::new("fixture", "fixture.").unwrap(),
            MemoryAuthority::new("neighbor", "neighbor.").unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let declarations = SealedDeclarationSnapshot::new(&requests).unwrap();
    let mut runtime = MemoryRuntime::new_with_config(
        ProbeBackingMemory::default(),
        MemoryManagerConfig::new(16).unwrap(),
    )
    .unwrap();
    runtime
        .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
        .unwrap();
    let [
        tenants,
        roots,
        objects,
        permissions,
        usage,
        manifests,
        confirmed,
        references,
        receipts,
        root_requests,
        funding_accounting,
        funding_intents,
        gateways,
        read_journal,
        read_sessions,
        read_tenants,
    ] = std::array::from_fn(|index| probe_memory(&mut runtime, MEMORY_KEYS[index], index));
    let uploads = UploadMemories {
        tenants,
        roots,
        objects,
        permissions,
        usage,
        manifests,
        confirmed,
        references,
        receipts,
        root_requests,
    };
    let memory = ServiceMemories {
        uploads,
        funding: ic_blob_storage::ops::service::funding::FundingMemories {
            accounting: funding_accounting,
            intents: funding_intents,
        },
        gateways,
        reads: ic_blob_storage::ops::service::reads::ReadSessionMemories {
            journal: read_journal,
            sessions: read_sessions,
            tenants: read_tenants,
        },
    };
    (runtime, memory)
}
fn probe_memory(
    runtime: &mut MemoryRuntime<ProbeBackingMemory>,
    key: &str,
    index: usize,
) -> ProbeMemory {
    ProbeMemory {
        memory: runtime.open_memory(key).unwrap(),
        index,
        fault: match key {
            "fixture.tenants.v1" => Some(WriteFault::Tenants),
            "fixture.root_objects.v1" => Some(WriteFault::Objects),
            "fixture.permissions.v1" => Some(WriteFault::Permissions),
            "fixture.usage.v1" => Some(WriteFault::Usage),
            "fixture.manifests.v1" => Some(WriteFault::Manifests),
            "fixture.confirmed.v1" => Some(WriteFault::Confirmed),
            "fixture.references.v1" => Some(WriteFault::References),
            "fixture.receipts.v1" => Some(WriteFault::Receipts),
            "fixture.root_requests.v1" => Some(WriteFault::RootRequests),
            "fixture.funding_accounting.v1" => Some(WriteFault::FundingAccounting),
            "fixture.funding_intents.v1" => Some(WriteFault::FundingIntents),
            "fixture.gateways.v1" => Some(WriteFault::Gateways),
            "fixture.read_journal.v1" => Some(WriteFault::ReadJournal),
            "fixture.read_sessions.v1" => Some(WriteFault::ReadSessions),
            "fixture.read_tenants.v1" => Some(WriteFault::ReadTenants),
            _ => None,
        },
    }
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "CDK decoder owns the argument buffer"
)]
pub(crate) fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    if bytes.len() > blob_test_protocol::storage::resources::STORAGE_PROBE_INPUT_BYTES {
        ic_cdk::trap("fixture input byte bound");
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(32_768)
        .set_skipping_quota(1000)
        .set_max_header_len(4096)
        .set_max_type_len(64)
        .set_full_error_message(false);
    decode_one_with_config(&bytes, &config)
        .unwrap_or_else(|_| ic_cdk::trap("invalid fixture input"))
}
pub(crate) fn admit(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
    fault: Option<WriteFault>,
) -> Result<
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation,
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
> {
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        ic_blob_storage::workflow::uploads::admission::admit(
            &mut state.as_mut().unwrap().uploads,
            context,
            input,
            ic_cdk::api::time(),
        )
    });
    TRAP_WRITE.set(None);
    result
}
pub(crate) fn admit_with_growth(
    context: UploadContext,
    input: blob_test_protocol::storage::GrowthAdmission,
) -> Result<
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation,
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
> {
    // Exercise IC rollback after real application writes, then an explicit
    // physical backing reservation through the maintained runtime growth API.
    let result = admit(context, input.permission, None)?;
    STATE.with_borrow(|state| {
        let memory = state
            .as_ref()
            .unwrap()
            .runtime
            .open_memory("fixture.permissions.v1")
            .unwrap();
        let before = memory.size();
        REFUSE_GROWTH.set(input.refuse);
        let grown = memory.grow(16);
        if input.refuse {
            if !matches!(
                grown,
                Err(ic_blob_storage::ic_memory::RuntimeGrowError::BackingRefused { .. })
            ) {
                REFUSE_GROWTH.set(false);
                return Err(
                    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure::Internal,
                );
            }
            if memory.size() != before {
                REFUSE_GROWTH.set(false);
                return Err(
                    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure::Internal,
                );
            }
            // Only the expected typed backing refusal and unchanged extent take the trap
            // path. Unexpected outcomes return normally so the test cannot mistake
            // an assertion panic for qualified growth-refusal rollback.
            ic_cdk::trap("fixture backing growth refused");
        }
        REFUSE_GROWTH.set(false);
        assert_eq!(grown, Ok(before));
        Ok(())
    })?;
    Ok(result)
}
pub(crate) fn prepare(
    context: UploadContext,
    input: &ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest,
    fault: Option<WriteFault>,
) -> Result<
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation,
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure,
> {
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        ic_blob_storage::workflow::uploads::manifests::prepare(
            &mut state.as_mut().unwrap().uploads,
            context,
            input,
            ic_cdk::api::time(),
        )
    });
    TRAP_WRITE.set(None);
    result
}
pub(crate) fn revoke(
    context: UploadContext,
    input: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
    fault: Option<WriteFault>,
) -> Result<
    ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse,
    ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
> {
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        ic_blob_storage::workflow::uploads::admission::revoke(
            &mut state.as_mut().unwrap().uploads,
            context,
            input,
        )
    });
    TRAP_WRITE.set(None);
    result
}
pub(crate) fn lookup(context: UploadContext, input: Request) -> Result<Observation, Failure> {
    let request = conversion::request(input)?;
    let v = STATE
        .with_borrow(|state| state.as_ref().unwrap().uploads.lookup(context, request))
        .map_err(conversion::failure)?;
    Ok(Observation {
        permission: Permission {
            request: input,
            uploader: v.permission.uploader,
            expires_at_ns: v.permission.expires_at_ns,
        },
        generation: v.tenant_generation.get(),
        revoked: v.revoked,
        prepared: v.manifest == UploadManifestState::Bound,
        phase: conversion::phase(v.phase),
    })
}
pub(crate) fn update_tenant(
    context: UploadContext,
    input: TenantUpdateRequest,
    fault: Option<WriteFault>,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        ic_blob_storage::workflow::tenants::update(
            &mut state.as_mut().unwrap().uploads,
            context,
            input,
        )
    });
    TRAP_WRITE.set(None);
    result
}
pub(crate) fn tenant(
    context: UploadContext,
    scope: TenantScope,
) -> Result<TenantEnrollmentResponse, TenantFailure> {
    STATE.with_borrow(|state| {
        ic_blob_storage::workflow::tenants::inspect(
            &state.as_ref().unwrap().uploads,
            context,
            scope,
        )
    })
}
pub(crate) fn status(context: UploadContext) -> Result<Status, Failure> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        let usage = state.uploads.usage().map_err(conversion::failure)?;
        Ok(Status {
            operations: usage.operations as u64,
            active: usage.active_reservations as u64,
            bytes: usage.reserved_bytes,
            usage: blob_test_protocol::journey::JourneyUsage {
                logical: usage.logical_bytes,
                physical: usage.physical_bytes,
                liability: usage.liability_bytes,
            },
            fenced: state.uploads.is_fenced(),
        })
    })
}
