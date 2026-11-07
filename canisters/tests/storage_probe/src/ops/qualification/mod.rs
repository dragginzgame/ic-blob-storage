//! Heap access and passive boundary conversion; the core owns every transition.

mod conversion;
pub(crate) mod decode;
pub(crate) mod release;
pub(crate) mod resources;
use super::{ProbeMemory, STATE};
use blob_test_protocol::{
    admission::{Enrollment, Failure, ManifestState, Observation, Outcome, Permission, Request},
    journey::{JourneyManifest, JourneyUsage},
};
use conversion::{failure, phase, request};
use ic_blob_storage::model::catalog::admission::UploadAdmission;
use ic_blob_storage::model::lifecycle::LifecycleChange;
use ic_blob_storage::model::service::upload::UploadManifestState;
use ic_blob_storage::model::service::upload::manifest::UploadManifest;
use ic_blob_storage::ops::service::uploads::StableUploads;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_platform::Principal;
use std::num::NonZeroU64;

fn mutate<T>(f: impl FnOnce(&mut StableUploads<ProbeMemory>) -> T) -> T {
    // Ordinary typed failures preserve the owner; never reconstruct it on error.
    STATE.with_borrow_mut(|state| f(&mut state.as_mut().expect("initialized probe").uploads))
}

pub(crate) fn enroll(
    context: UploadContext,
    tenant: Principal,
    expected: Option<Enrollment>,
    active: bool,
) -> Result<Outcome, Failure> {
    let expected = expected
        .map(|entry| {
            Ok(TenantEnrollmentView {
                generation: NonZeroU64::new(entry.generation).ok_or(Failure::InvalidInput)?,
                active: entry.active,
            })
        })
        .transpose()?;
    let view = mutate(|owner| {
        owner.update_tenant(
            context,
            TenantUpdate {
                tenant,
                expected,
                active,
            },
        )
    })
    .map_err(failure)?;
    Ok(Outcome::Enrolled(Enrollment {
        generation: view.generation.get(),
        active: view.active,
    }))
}

pub(crate) fn admit(
    context: UploadContext,
    input: Permission,
    now: u64,
) -> Result<Outcome, Failure> {
    let permission = UploadPermission {
        request: request(input.request)?,
        uploader: input.uploader.into(),
        expires_at_ns: input.expires_at_ns,
    };
    Ok(
        match mutate(|owner| owner.admit(context, permission, now)).map_err(failure)? {
            UploadAdmission::Reserved => Outcome::Admitted,
            UploadAdmission::Existing(existing) => Outcome::Existing(phase(existing)),
        },
    )
}

pub(crate) fn prepare(
    context: UploadContext,
    input: Request,
    manifest: &JourneyManifest,
    now: u64,
) -> Result<Outcome, Failure> {
    let request = request(input)?;
    // Reject shape/byte excess before allocating conversion vectors. The shared
    // owner still checks authority and validates the exact manifest independently.
    let limits = STATE.with_borrow(|state| {
        state
            .as_ref()
            .expect("initialized probe")
            .config
            .manifest_limits()
    });
    if manifest.chunks.len() > limits.max_chunks.get()
        || manifest.headers.len() > limits.max_headers.get()
    {
        return Err(Failure::Manifest);
    }
    let mut remaining = limits.max_header_bytes.get();
    for (name, value) in &manifest.headers {
        for bytes in [name.len(), value.len(), 3] {
            remaining = remaining.checked_sub(bytes).ok_or(Failure::Manifest)?;
        }
    }
    let chunks = manifest
        .chunks
        .iter()
        .map(|hash| CaffeineChunkHash::try_from(hash.as_slice()).expect("fixed hash width"))
        .collect::<Vec<_>>();
    let headers = manifest
        .headers
        .iter()
        .map(|(name, value)| CaffeineHeader { name, value })
        .collect::<Vec<_>>();
    mutate(|owner| {
        owner.prepare_manifest(
            context,
            request,
            UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
            now,
        )
    })
    .map(changed)
    .map_err(failure)
}

pub(crate) fn expose(context: UploadContext, root: [u8; 32], now: u64) -> Result<Outcome, Failure> {
    let root = ProviderRootHash::try_from(root.as_slice()).expect("fixed hash width");
    mutate(|owner| {
        ic_blob_storage::test_support::certificate::issue(owner, context, &root.to_string(), now)
    })
    .map(|_| Outcome::Exposed)
    .map_err(Failure::Exposure)
}

pub(crate) fn revoke(context: UploadContext, input: Request) -> Result<Outcome, Failure> {
    let request = request(input)?;
    mutate(|owner| owner.revoke(context, request))
        .map(changed)
        .map_err(failure)
}

fn changed(change: LifecycleChange) -> Outcome {
    Outcome::Changed(change == LifecycleChange::Changed)
}

pub(crate) fn inspect(context: UploadContext, input: Request) -> Result<Observation, Failure> {
    let request = request(input)?;
    STATE.with_borrow(|state| {
        let owner = &state.as_ref().expect("initialized probe").uploads;
        let view = owner.lookup(context, request).map_err(failure)?;
        let usage = if context.actor == input.tenant.into() {
            let usage = owner.tenant_usage(input.tenant.into()).map_err(failure)?;
            Some({
                JourneyUsage {
                    logical: usage.logical_bytes,
                    physical: usage.physical_bytes,
                    liability: usage.liability_bytes,
                }
            })
        } else {
            None
        };
        Ok(Observation {
            permission: Permission {
                request: input,
                uploader: view.permission.uploader.into(),
                expires_at_ns: view.permission.expires_at_ns,
            },
            generation: view.tenant_generation.get(),
            revoked: view.revoked,
            phase: phase(view.phase),
            manifest: match view.manifest {
                UploadManifestState::Unprepared => ManifestState::Unprepared,
                UploadManifestState::Bound => ManifestState::Bound,
            },
            usage,
        })
    })
}
