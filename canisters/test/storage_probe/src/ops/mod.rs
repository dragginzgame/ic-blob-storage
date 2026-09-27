//! Host grants, passive DTO conversion and fixture-only stable-write faults.
mod configuration;
mod conversion;
pub(crate) mod lifecycle;
pub(crate) mod planning;
pub(crate) mod read;
use blob_test_protocol::{
    admission::{
        Enrollment, Permission, Request,
        input::{EnrollmentInput, PreparationInput},
    },
    storage::{Failure, Observation, Status, WriteFault},
};
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_blob_storage::{
    ic_memory::{
        GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig,
        MemoryManagerIdRange, MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, RuntimeMemory,
        SchemaMetadata, SealedDeclarationSnapshot, StaticMemoryRangeDeclaration,
        ic_stable_structures::{DefaultMemoryImpl, Memory},
    },
    model::{
        catalog::admission::UploadAdmission,
        identity::caffeine::{CaffeineHeader, manifest::CaffeineChunkHash},
        lifecycle::LifecycleChange,
        service::{
            tenant::{TenantEnrollmentView, TenantUpdate},
            upload::{
                UploadContext, UploadManifestState, UploadPermission, manifest::UploadManifest,
            },
        },
    },
    ops::service::uploads::{StableUploads, UploadMemories},
};
use std::{
    cell::{Cell, RefCell},
    num::NonZeroU64,
};

struct ProbeMemory {
    memory: RuntimeMemory<DefaultMemoryImpl>,
    fault: Option<WriteFault>,
}
impl Memory for ProbeMemory {
    fn size(&self) -> u64 {
        self.memory.size()
    }
    fn grow(&self, pages: u64) -> i64 {
        self.memory.grow(pages)
    }
    fn read(&self, offset: u64, dst: &mut [u8]) {
        self.memory.read(offset, dst);
    }
    fn write(&self, offset: u64, src: &[u8]) {
        if self.fault.is_some() && self.fault == TRAP_WRITE.get() {
            ic_cdk::trap("fixture stable write failure");
        }
        self.memory.write(offset, src);
    }
}
struct State {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    operator: Principal,
    uploads: StableUploads<ProbeMemory>,
}
thread_local! {
    static STATE:RefCell<Option<State>>=const { RefCell::new(None) };
    static TRAP_WRITE:Cell<Option<WriteFault>>=const { Cell::new(None) };
}
pub(crate) fn initialize(operator: Principal, restored: bool) {
    let keys = [
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
    ];
    let requests =
        keys.map(|key| MemoryRequest::new("fixture", key, SchemaMetadata::default()).unwrap());
    let grant = StaticMemoryRangeDeclaration::new(
        MemoryManagerAuthorityRecord::new(
            MemoryManagerIdRange::new(120, 129).unwrap(),
            "fixture",
            MemoryManagerRangeMode::Allowed,
            None,
        )
        .unwrap(),
    )
    .unwrap();
    let declarations = SealedDeclarationSnapshot::new(&[], &[grant], &requests).unwrap();
    let mut runtime = MemoryRuntime::new_with_config(
        DefaultMemoryImpl::default(),
        MemoryManagerConfig::new(16).unwrap(),
    )
    .unwrap();
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
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
    ] = keys.map(|key| ProbeMemory {
        memory: runtime.open_memory_by_key(key).unwrap(),
        fault: match key {
            "fixture.root_objects.v1" => Some(WriteFault::Objects),
            "fixture.permissions.v1" => Some(WriteFault::Permissions),
            "fixture.usage.v1" => Some(WriteFault::Usage),
            "fixture.manifests.v1" => Some(WriteFault::Manifests),
            "fixture.confirmed.v1" => Some(WriteFault::Confirmed),
            "fixture.references.v1" => Some(WriteFault::References),
            "fixture.receipts.v1" => Some(WriteFault::Receipts),
            "fixture.root_requests.v1" => Some(WriteFault::RootRequests),
            _ => None,
        },
    });
    let memory = UploadMemories {
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
    let config = configuration::configuration(operator);
    let uploads = if restored {
        StableUploads::open(memory, config)
    } else {
        StableUploads::install(memory, config)
    }
    .unwrap();
    STATE.with_borrow_mut(|state| {
        assert!(state.is_none(), "initialization is not reset");
        *state = Some(State {
            _runtime: runtime,
            operator,
            uploads,
        });
    });
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "CDK decoder owns the argument buffer"
)]
pub(crate) fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    if bytes.len() > 4096 {
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
    input: Permission,
    fault: Option<WriteFault>,
) -> Result<bool, Failure> {
    let permission = UploadPermission {
        request: conversion::request(input.request)?,
        uploader: input.uploader,
        expires_at_ns: input.expires_at_ns,
    };
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .unwrap()
            .uploads
            .admit(context, permission, ic_cdk::api::time())
    });
    TRAP_WRITE.set(None);
    result
        .map(|r| r == UploadAdmission::Reserved)
        .map_err(conversion::failure)
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "bounded input owned by dispatch"
)]
pub(crate) fn prepare(
    context: UploadContext,
    input: PreparationInput,
    fault: Option<WriteFault>,
) -> Result<bool, Failure> {
    let request = conversion::request(input.request)?;
    let chunks = input
        .manifest
        .chunks
        .iter()
        .map(|v| CaffeineChunkHash::try_from(v.as_slice()).expect("fixed leaf"))
        .collect::<Vec<_>>();
    let headers = input
        .manifest
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect::<Vec<_>>();
    TRAP_WRITE.set(fault);
    let result = STATE.with_borrow_mut(|state| {
        state.as_mut().unwrap().uploads.prepare_manifest(
            context,
            request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            ic_cdk::api::time(),
        )
    });
    TRAP_WRITE.set(None);
    result
        .map(|v| v == LifecycleChange::Changed)
        .map_err(conversion::failure)
}
pub(crate) fn expose(context: UploadContext, input: Request) -> Result<(), Failure> {
    let request = conversion::request(input)?;
    STATE
        .with_borrow_mut(|state| {
            state
                .as_mut()
                .unwrap()
                .uploads
                .expose(context, request, ic_cdk::api::time())
        })
        .map_err(conversion::failure)
}
pub(crate) fn revoke(
    context: UploadContext,
    input: Request,
    fault: Option<WriteFault>,
) -> Result<bool, Failure> {
    let request = conversion::request(input)?;
    TRAP_WRITE.set(fault);
    let result =
        STATE.with_borrow_mut(|state| state.as_mut().unwrap().uploads.revoke(context, request));
    TRAP_WRITE.set(None);
    result
        .map(|v| v == LifecycleChange::Changed)
        .map_err(conversion::failure)
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
fn enrollment(view: TenantEnrollmentView) -> Enrollment {
    Enrollment {
        generation: view.generation.get(),
        active: view.active,
    }
}
pub(crate) fn enroll(
    context: UploadContext,
    input: EnrollmentInput,
) -> Result<Enrollment, Failure> {
    let expected = input
        .expected
        .map(|v| {
            Ok(TenantEnrollmentView {
                generation: NonZeroU64::new(v.generation).ok_or(Failure::Invalid)?,
                active: v.active,
            })
        })
        .transpose()?;
    STATE
        .with_borrow_mut(|state| {
            state.as_mut().unwrap().uploads.update_tenant(
                context,
                TenantUpdate {
                    tenant: input.tenant,
                    expected,
                    active: input.active,
                },
            )
        })
        .map(enrollment)
        .map_err(conversion::failure)
}
pub(crate) fn tenant(
    context: UploadContext,
    tenant: Principal,
) -> Result<Option<Enrollment>, Failure> {
    STATE
        .with_borrow(|state| state.as_ref().unwrap().uploads.tenant(context, tenant))
        .map(|v| v.map(enrollment))
        .map_err(conversion::failure)
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
