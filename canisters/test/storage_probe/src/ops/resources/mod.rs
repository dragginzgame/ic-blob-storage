//! Bounded synthetic population through the maintained durable operations; no provider effects.
use super::{STATE, conversion};
use blob_test_protocol::{
    admission::Request,
    storage::{
        Failure,
        resources::{PopulationBatch, RestorationMemoryReads, RestorationResources},
    },
};
use candid::Principal;
use ic_blob_storage::model::{
    identity::caffeine::{
        CaffeineHashLimits, CaffeineHeader, manifest::builder::CaffeineManifestBuilder,
    },
    lifecycle::{
        ReferenceId,
        binding::ReferenceKey,
        requests::{ReferenceOperation, ReferenceRequest, ReferenceRequestId},
    },
    service::{
        tenant::TenantUpdate,
        upload::{UploadContext, UploadPermission, manifest::UploadManifest},
    },
};
use std::cell::{Cell, RefCell};
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

#[derive(Clone, Copy)]
struct ReadCounter {
    calls: u64,
    bytes: u64,
    instructions: u64,
}
thread_local! {
    static MEASURE_READS: Cell<bool> = const { Cell::new(false) };
    static READS: RefCell<[ReadCounter; 16]> = const { RefCell::new([ReadCounter { calls: 0, bytes: 0, instructions: 0 }; 16]) };
}

pub(super) fn begin_read_profile(restored: bool) {
    MEASURE_READS.set(restored);
}
pub(super) fn start_read() -> Option<u64> {
    MEASURE_READS
        .get()
        .then(ic_cdk::api::call_context_instruction_counter)
}
pub(super) fn finish_read(index: usize, bytes: usize, started: Option<u64>) {
    if let Some(started) = started {
        let instructions = ic_cdk::api::call_context_instruction_counter() - started;
        READS.with_borrow_mut(|reads| {
            let row = &mut reads[index];
            row.calls += 1;
            row.bytes += bytes as u64;
            row.instructions += instructions;
        });
    }
}
pub(super) fn finish_read_profile() -> Vec<RestorationMemoryReads> {
    if !MEASURE_READS.replace(false) {
        return Vec::new();
    }
    READS.with_borrow(|reads| {
        reads
            .iter()
            .zip(super::MEMORY_KEYS)
            .map(|(r, memory)| RestorationMemoryReads {
                memory: memory.to_owned(),
                calls: r.calls,
                bytes: r.bytes,
                instructions: r.instructions,
            })
            .collect()
    })
}

pub(super) fn heap_bytes() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        (core::arch::wasm32::memory_size(0) as u64) * 65_536
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

pub(crate) fn inspect(context: UploadContext) -> Result<RestorationResources, Failure> {
    STATE.with_borrow(|state| {
        let state = state.as_ref().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        Ok(state.resources.clone())
    })
}

pub(crate) fn populate(
    context: UploadContext,
    batch: PopulationBatch,
) -> Result<Vec<Request>, Failure> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        if context.actor != state.operator { return Err(Failure::Denied); }
        if state.uploads.is_fenced() { return Err(Failure::Fenced); }
        if batch.count == 0 || batch.count > 100 || batch.start.checked_add(batch.count).is_none_or(|end| end > 10_000) {
            return Err(Failure::Invalid);
        }
        if state.uploads.usage().map_err(conversion::failure)?.operations != batch.start as usize {
            return Err(Failure::Conflict);
        }
        let tenant = Principal::from_slice(&[43, 1]);
        if batch.start == 0 {
            state.uploads.update_tenant(context, TenantUpdate { tenant, expected: None, active: true }).unwrap();
        }
        let tenant_context = UploadContext { actor: tenant, ..context };
        let now = ic_cdk::api::time();
        let headers = [CaffeineHeader { name: "Content-Length", value: "10" },
            CaffeineHeader { name: "Content-Type", value: "application/octet-stream" }];
        let mut samples = Vec::new();
        for index in batch.start..batch.start + batch.count {
            let mut body = [0; 10];
            body[..4].copy_from_slice(&index.to_le_bytes());
            let mut builder = CaffeineManifestBuilder::new(10, &headers, CaffeineHashLimits {
                max_content_bytes: NonZeroU64::new(10).unwrap(),
                max_append_bytes: NonZeroUsize::new(10).unwrap(),
                max_headers: NonZeroUsize::new(8).unwrap(),
                max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            }, NonZeroUsize::MIN).unwrap();
            builder.append(0, &body).unwrap();
            let built = builder.finish().unwrap();
            let input = Request { service: context.service, tenant, namespace: 1,
                id: u128::from(index) + 1, root: *built.hashes().provider_root.as_bytes(), bytes: 10 };
            let request = conversion::request(input).unwrap();
            state.uploads.admit(tenant_context, UploadPermission { request, uploader: tenant,
                expires_at_ns: now.checked_add(60_000_000_000).unwrap() }, now).unwrap();
            if index % 4 == 3 {
                state.uploads.revoke(tenant_context, request).unwrap();
            } else {
                state.uploads.prepare_manifest(tenant_context, request,
                    UploadManifest { chunks: built.manifest().chunks(), headers: &headers }, now).unwrap();
                state.uploads.expose(tenant_context, request, now).unwrap();
                if index % 4 != 0 {
                    // Explicit local completion substitute, not verifier or deployed-provider evidence.
                    state.uploads.confirm_upload(request).unwrap();
                    let operation = if index % 4 == 1 {
                        ReferenceOperation::Retain(ReferenceKey::new(request.object.first.object(),
                            ReferenceId::new(NonZeroU128::new(2).unwrap())))
                    } else { ReferenceOperation::Release(request.object.first) };
                    let outcome = state.uploads.apply_reference(tenant_context, request,
                        ReferenceRequest { id: ReferenceRequestId::new(NonZeroU128::MIN), operation }).unwrap();
                    assert!(matches!(outcome, ic_blob_storage::model::lifecycle::requests::ReferenceRequestOutcome::Recorded { result: Ok(_) }));
                }
            }
            if samples.len() < 4 { samples.push(input); }
        }
        Ok(samples)
    })
}
