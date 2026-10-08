//! Bounded synthetic population through the maintained durable operations; no provider effects.
pub(crate) mod histories;
use super::{STATE, conversion};
use blob_test_protocol::storage::{
    Failure,
    resources::{PopulationBatch, RestorationMemoryReads, RestorationResources},
};
use ic_blob_storage::model::service::upload::manifest::UploadManifest;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::reference::binding::ReferenceOperation;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::reference::binding::ReferenceRequestId;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use std::cell::{Cell, RefCell};
use std::num::NonZeroU128;

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

pub(crate) fn begin_read_profile(restored: bool) {
    READS.with_borrow_mut(|reads| {
        reads.fill(ReadCounter {
            calls: 0,
            bytes: 0,
            instructions: 0,
        });
    });
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
            // One completed wrapped read, in IC call-context instructions.
            // Bytes have a separate unit and share this read's attribution.
            ic_metrics::record_sample(&mut row.calls, &mut row.instructions, instructions);
            row.bytes = row.bytes.saturating_add(bytes as u64);
        });
    }
}
pub(crate) fn finish_read_profile() -> Vec<RestorationMemoryReads> {
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

pub(crate) fn heap_bytes() -> u64 {
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

pub(crate) fn populate(context: UploadContext, batch: PopulationBatch) -> Result<(), Failure> {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().unwrap();
        if context.actor != state.operator {
            return Err(Failure::Denied);
        }
        if state.uploads.is_fenced() {
            return Err(Failure::Fenced);
        }
        let count = u32::try_from(batch.uploads.len()).map_err(|_| Failure::Invalid)?;
        if count == 0
            || count > 100
            || batch
                .start
                .checked_add(count)
                .is_none_or(|end| end > 10_000)
        {
            return Err(Failure::Invalid);
        }
        if state
            .uploads
            .usage()
            .map_err(conversion::failure)?
            .operations
            != batch.start as usize
        {
            return Err(Failure::Conflict);
        }
        for (offset, input) in batch.uploads.iter().enumerate() {
            // Failed domain operations trap: IC rollback preserves the complete batch.
            populate_upload(state, context, input, batch.start as usize + offset);
        }
        Ok(())
    })
}

fn populate_upload(
    state: &mut super::State,
    context: UploadContext,
    input: &blob_test_protocol::admission::input::PreparationInput,
    index: usize,
) {
    let tenant = input.request.tenant;
    if state.uploads.tenant(context, tenant).unwrap().is_none() {
        state
            .uploads
            .update_tenant(
                context,
                TenantUpdate {
                    tenant,
                    expected: None,
                    active: true,
                },
            )
            .unwrap();
    }
    let tenant_context = UploadContext {
        actor: tenant,
        ..context
    };
    let now = ic_cdk::api::time();
    let request = conversion::request(input.request).unwrap();
    state
        .uploads
        .admit(
            tenant_context,
            UploadPermission {
                request,
                uploader: tenant,
                expires_at_ns: u64::MAX,
            },
            now,
        )
        .unwrap();
    if index % 4 == 3 {
        state.uploads.revoke(tenant_context, request).unwrap();
        return;
    }
    let chunks: Vec<_> = input
        .manifest
        .chunks
        .iter()
        .map(|hash| CaffeineChunkHash::try_from(hash.as_slice()).unwrap())
        .collect();
    let headers: Vec<_> = input
        .manifest
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect();
    state
        .uploads
        .prepare_manifest(
            tenant_context,
            request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            now,
        )
        .unwrap();
    state.uploads.expose(tenant_context, request, now).unwrap();
    if index.is_multiple_of(4) {
        return;
    }
    // Explicit local completion substitute, not verifier or deployed-provider evidence.
    state.uploads.confirm_upload(request).unwrap();
    let operation = if index % 4 == 1 {
        ReferenceOperation::Retain(ReferenceKey::new(
            request.object.first.object(),
            ReferenceId::new(NonZeroU128::new(2).unwrap()),
        ))
    } else {
        ReferenceOperation::Release(request.object.first)
    };
    let outcome = state
        .uploads
        .apply_reference(
            tenant_context,
            request,
            ReferenceRequest {
                id: ReferenceRequestId::new(NonZeroU128::MIN),
                operation,
            },
        )
        .unwrap();
    assert!(matches!(
        outcome,
        ic_blob_storage::model::lifecycle::requests::ReferenceRequestOutcome::Recorded {
            result: Ok(_)
        }
    ));
}

pub(crate) fn credit_resources(
    started: u64,
    result: Result<Option<bool>, Failure>,
) -> blob_test_protocol::storage::resources::FundingCreditResources {
    blob_test_protocol::storage::resources::FundingCreditResources {
        result,
        instructions: ic_cdk::api::call_context_instruction_counter() - started,
        heap_bytes: heap_bytes(),
        stable_bytes: ic_cdk::api::stable_size() * 65_536,
        reads: finish_read_profile(),
    }
}
