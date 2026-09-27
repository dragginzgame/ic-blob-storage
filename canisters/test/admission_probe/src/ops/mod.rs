//! Heap access and passive boundary conversion; the core owns every transition.

pub(crate) mod content;
mod conversion;
pub(crate) mod decode;
pub(crate) mod release;
pub(crate) mod resources;
use blob_test_protocol::{
    admission::{
        Enrollment, ExecutionProfile, Failure, Installation, ManifestState, Observation, Outcome,
        Permission, Request, Workload,
    },
    journey::{JourneyManifest, JourneyUsage},
};
use candid::Principal;
use conversion::{failure, phase, request};
use ic_blob_storage::model::{
    billing::{FundingLimits, configuration::BillingConfiguration},
    catalog::{
        CatalogLimits,
        admission::{UploadAdmission, UploadLimits},
    },
    identity::{
        ProviderRootHash,
        caffeine::{CaffeineHeader, manifest::CaffeineChunkHash},
    },
    lifecycle::LifecycleChange,
    service::{
        configuration::{
            ServiceBindings, ServiceConfiguration, ServiceLimits, ServiceManifestLimits,
        },
        tenant::{TenantEnrollmentView, TenantUpdate},
        upload::{
            UploadAdmissions, UploadContext, UploadManifestState, UploadPermission,
            manifest::UploadManifest,
        },
    },
};
use std::{
    cell::RefCell,
    num::{NonZeroU64, NonZeroU128, NonZeroUsize},
};

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

struct State {
    owner: UploadAdmissions,
    config: ServiceConfiguration,
    last_profile: Option<ExecutionProfile>,
}

fn count(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).expect("positive fixture count")
}
fn number(n: u128) -> NonZeroU128 {
    NonZeroU128::new(n).expect("positive fixture bound")
}

pub(crate) fn initialize(service: Principal, installation: Installation) {
    let operator = installation.operator;
    let (objects, tenant_objects, chunks, tenant_chunks, tenant_bytes, references, receipts) =
        match installation.workload {
            Workload::Small => (4, 2, 15, 10, 10, 2, 3),
            Workload::RetainedHistory => (256, 128, 256, 128, 10, 2, 3),
            Workload::ReleaseHistory => (704, 704, 768, 768, 288, 4, 7),
        };
    let config = ServiceConfiguration::new(
        ServiceBindings {
            service,
            operator,
            payment_account: service,
            namespace: number(1),
        },
        ServiceLimits {
            max_tenants: count(2),
            max_object_bytes: NonZeroU64::new(10 * 1024 * 1024).unwrap(),
            max_headers: count(8),
            max_header_bytes: count(1024),
            manifests: ServiceManifestLimits {
                max_chunks: count(chunks),
                max_tenant_chunks: count(tenant_chunks),
            },
            catalog: CatalogLimits {
                max_objects: count(objects),
                max_tenant_objects: count(tenant_objects),
                max_physical_bytes: number(2 * tenant_bytes * 1024 * 1024),
                max_liability_bytes: number(2 * tenant_bytes * 1024 * 1024),
                max_tenant_logical_bytes: number(tenant_bytes * 1024 * 1024),
                max_references_per_object: count(references),
                max_receipts_per_object: count(receipts),
            },
            uploads: UploadLimits {
                max_active: count(4),
                max_tenant_active: count(2),
            },
        },
        // Explicit substitute identity only; this probe cannot call a Cashier.
        BillingConfiguration::new(operator, FundingLimits::new(1, 10, 100).unwrap(), 8, 4).unwrap(),
    )
    .expect("valid local envelope");
    STATE.with_borrow_mut(|state| {
        assert!(state.is_none(), "initialization is not reset");
        *state = Some(State {
            owner: UploadAdmissions::new(config),
            config,
            last_profile: None,
        });
    });
}

fn mutate<T>(f: impl FnOnce(&mut UploadAdmissions) -> T) -> T {
    // Ordinary typed failures preserve the owner; never reconstruct it on error.
    STATE.with_borrow_mut(|state| f(&mut state.as_mut().expect("initialized probe").owner))
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
        uploader: input.uploader,
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
    mutate(|owner| owner.expose_root(context, root, now))
        .map(|_| Outcome::Exposed)
        .map_err(failure)
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
        let owner = &state.as_ref().expect("initialized probe").owner;
        let view = owner.lookup(context, request).map_err(failure)?;
        let usage = (context.actor == input.tenant).then(|| {
            let usage = owner.catalog().tenant_usage(input.tenant);
            JourneyUsage {
                logical: usage.logical_bytes,
                physical: usage.physical_bytes,
                liability: usage.liability_bytes,
            }
        });
        Ok(Observation {
            permission: Permission {
                request: input,
                uploader: view.permission.uploader,
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
