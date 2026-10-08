//! One durable upload/lifecycle owner. Provider evidence is a trusted-host input.
pub mod admission;
pub(crate) mod callbacks;
pub mod capacity;
pub mod certificate;
pub mod completion;
pub mod discovery;
pub mod exposure;
pub mod history;
mod lifecycle;
pub mod manifests;
mod planning;
pub mod read;
mod recovery;
pub(crate) use recovery::envelope as validate_envelope;
pub mod status;
pub mod tenants;
use super::{
    roots::{RootStoreError, StableRootClaims},
    tenant::{StableTenantEnrollments, TenantStoreError},
};
use crate::model::catalog::admission::UploadAdmission;
use crate::model::catalog::admission::UploadError;
use crate::model::catalog::admission::UploadPhase;
use crate::model::catalog::admission::UploadUsage;
use crate::model::catalog::admission::capacity as admission_capacity;
use crate::model::lifecycle::LifecycleChange;
use crate::model::lifecycle::roots::RootClaimError;
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadManifestState;
use crate::model::service::upload::UploadPermissionView;
use crate::model::service::upload::manifest;
use crate::model::service::upload::manifest::UploadManifest;
use crate::model::service::upload::record::UploadConfigurationRecord;
use crate::model::service::upload::record::UploadManifestRecord;
use crate::model::service::upload::record::UploadPermissionRecord;
use crate::model::service::upload::record::UploadStoreRecord;
use crate::model::service::upload::record::UploadUsageRecord;
use crate::model::service::upload::record::lifecycle::ConfirmedLifecycleRecord;
use crate::model::service::upload::record::lifecycle::ReferenceReceiptRecord;
use crate::model::service::upload::record::lifecycle::ReferenceRecord;
use crate::model::service::upload::validation;
use candid::Principal;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::tenant::TenantEnrollmentView;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
pub use lifecycle::ConfirmedUploadView;
use std::num::NonZeroU64;
use thiserror::Error;

type Key = (Principal, u128);
fn key(request: UploadRequest) -> Key {
    (
        request.object.first.object().tenant(),
        request.id.get().get(),
    )
}
fn metadata() -> Key {
    (Principal::management_canister(), 0)
}

/// Ten distinct, exclusively owned memories granted by the host's `ic-memory` runtime.
/// No IDs or allocations are silently registered by the library.
pub struct UploadMemories<M: Memory> {
    /// Enrollment history.
    pub tenants: M,
    /// Forward root claims and their scope metadata.
    pub roots: M,
    /// Reverse object-to-root index.
    pub objects: M,
    /// Exact permissions and admission configuration.
    pub permissions: M,
    /// Maintained global and tenant charged totals.
    pub usage: M,
    /// Immutable bounded manifest declarations, written once per permission.
    pub manifests: M,
    /// Constant-size confirmed lifecycle and retained history counts.
    pub confirmed: M,
    /// Individual reference identities, including released tombstones.
    pub references: M,
    /// Individual exact reference operation receipts.
    pub receipts: M,
    /// Root-to-original-request index, committed with admission.
    pub root_requests: M,
}

/// Incremental durable upload, reference and obligation bookkeeping.
///
/// Hosts must call mutations synchronously in an IC update and propagate storage
/// traps: root, permission, manifest and accounting writes share that transaction.
/// There is no native-memory rollback guarantee, provider call, certificate output,
/// or independent provider authentication. This owner must not coexist with another owner
/// of the same namespace. Reopening validates all records then stays fenced.
pub struct StableUploads<M: Memory> {
    config: ServiceConfiguration,
    tenants: StableTenantEnrollments<M>,
    roots: StableRootClaims<M>,
    permissions: BTreeMap<Key, UploadStoreRecord, M>,
    usage: BTreeMap<Principal, UploadUsageRecord, M>,
    manifests: BTreeMap<Key, UploadManifestRecord, M>,
    confirmed: BTreeMap<Key, ConfirmedLifecycleRecord, M>,
    references: BTreeMap<(Key, u128), ReferenceRecord, M>,
    receipts: BTreeMap<(Key, u128), ReferenceReceiptRecord, M>,
    root_requests: BTreeMap<[u8; 32], u128, M>,
    fenced: bool,
}
impl<M: Memory> StableUploads<M> {
    pub(crate) fn set_recovery_fence(&mut self, fenced: bool) {
        self.fenced = fenced;
        self.tenants.set_recovery_fence(fenced);
        self.roots.set_recovery_fence(fenced);
    }
    /// Install in ten fresh memories after checking the complete upload envelope.
    /// # Errors
    /// Rejects allocated memory or an envelope exceeding the bounded manifest codec.
    /// # Panics
    /// Aliased memories or stable writes trap. Hosts must propagate this for IC rollback.
    pub fn install(
        memory: UploadMemories<M>,
        config: ServiceConfiguration,
    ) -> Result<Self, UploadStoreError> {
        recovery::envelope(&config)?;
        if [
            &memory.tenants,
            &memory.roots,
            &memory.objects,
            &memory.permissions,
            &memory.usage,
            &memory.manifests,
            &memory.confirmed,
            &memory.references,
            &memory.receipts,
            &memory.root_requests,
        ]
        .iter()
        .any(|m| m.size() != 0)
        {
            return Err(UploadStoreError::AlreadyAllocated);
        }
        let tenants = StableTenantEnrollments::install(memory.tenants, config)
            .expect("checked fresh tenant memory");
        let roots = StableRootClaims::install(memory.roots, memory.objects, config)
            .expect("fresh distinct root memories");
        assert_eq!(memory.permissions.size(), 0, "distinct permission memory");
        let mut permissions = BTreeMap::new(memory.permissions);
        assert_eq!(memory.usage.size(), 0, "distinct usage memory");
        let mut usage = BTreeMap::new(memory.usage);
        assert_eq!(memory.manifests.size(), 0, "distinct manifest memory");
        let manifests = BTreeMap::new(memory.manifests);
        assert_eq!(memory.confirmed.size(), 0, "distinct confirmed memory");
        let confirmed = BTreeMap::new(memory.confirmed);
        assert_eq!(memory.references.size(), 0, "distinct reference memory");
        let references = BTreeMap::new(memory.references);
        assert_eq!(memory.receipts.size(), 0, "distinct receipt memory");
        let receipts = BTreeMap::new(memory.receipts);
        assert_eq!(
            memory.root_requests.size(),
            0,
            "distinct root request memory"
        );
        let root_requests = BTreeMap::new(memory.root_requests);
        permissions.insert(
            metadata(),
            UploadStoreRecord::Configuration(UploadConfigurationRecord::new(&config)),
        );
        usage.insert(Principal::management_canister(), UploadUsageRecord::empty());
        Ok(Self {
            config,
            tenants,
            roots,
            permissions,
            usage,
            manifests,
            confirmed,
            references,
            receipts,
            root_requests,
            fenced: false,
        })
    }

    /// Validate retained configuration, identity indexes, manifests and totals.
    ///
    /// No missing row is reconstructed. All mutations remain fenced. Host release
    /// identity checks are required; cross-release recovery is unsupported.
    /// # Errors
    /// Rejects missing memories, changed configuration or inconsistent retained state.
    /// # Panics
    /// Corrupt binary memory/records trap without seeding a new owner.
    pub fn open(
        memory: UploadMemories<M>,
        config: ServiceConfiguration,
    ) -> Result<Self, UploadStoreError> {
        recovery::envelope(&config)?;
        if [
            &memory.tenants,
            &memory.roots,
            &memory.objects,
            &memory.permissions,
            &memory.usage,
            &memory.manifests,
            &memory.confirmed,
            &memory.references,
            &memory.receipts,
            &memory.root_requests,
        ]
        .iter()
        .any(|m| m.size() == 0)
        {
            return Err(UploadStoreError::Missing);
        }
        let tenants = StableTenantEnrollments::open(memory.tenants, config)?;
        let roots = StableRootClaims::open(memory.roots, memory.objects, config)?;
        let owner = Self {
            config,
            tenants,
            roots,
            permissions: BTreeMap::load(memory.permissions),
            usage: BTreeMap::load(memory.usage),
            manifests: BTreeMap::load(memory.manifests),
            confirmed: BTreeMap::load(memory.confirmed),
            references: BTreeMap::load(memory.references),
            receipts: BTreeMap::load(memory.receipts),
            root_requests: BTreeMap::load(memory.root_requests),
            fenced: true,
        };
        owner.validate()?;
        Ok(owner)
    }

    /// Operator enrollment uses the same model transition and store as standalone enrollment.
    /// # Errors
    /// Returns authenticated enrollment/precondition failures or the enforced restore fence.
    pub fn update_tenant(
        &mut self,
        context: UploadContext,
        input: TenantUpdate,
    ) -> Result<TenantEnrollmentView, UploadStoreError> {
        Ok(self.tenants.update(context, input)?)
    }
    /// Inspect enrollment as its tenant or configured operator.
    /// # Errors
    /// Rejects wrong scope, observer or corrupt records.
    pub fn tenant(
        &self,
        context: UploadContext,
        tenant: Principal,
    ) -> Result<Option<TenantEnrollmentView>, UploadStoreError> {
        Ok(self.tenants.inspect(context, tenant)?)
    }

    /// Commit one exact permission, immutable root claim and all reservation totals.
    /// Exact retries preserve deadline, generation and history without new capacity.
    /// # Errors
    /// Rejects wrong tenant/scope, fence, changed identity, inactive enrollment,
    /// invalid permission or exhausted global/tenant capacity before any write.
    /// # Panics
    /// Storage failure traps and must roll back the complete IC update.
    pub fn admit(
        &mut self,
        context: UploadContext,
        input: UploadPermission,
        now: u64,
    ) -> Result<UploadAdmission, UploadStoreError> {
        self.project(context, input.request)?;
        self.mutable()?;
        if let Some(record) = self.permission(input.request)? {
            let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
            if view.permission != input {
                return Err(UploadAdmissionError::PermissionConflict.into());
            }
            return Ok(UploadAdmission::Existing(view.phase));
        }
        let generation = self.generation(context.actor)?;
        validation::fresh(&self.config, input, now)?;
        let mut global = self.total(Principal::management_canister())?;
        let mut tenant = self.total(context.actor)?;
        recovery::manifest_capacity(&self.config, global, tenant, input.request.object.bytes)?;
        if self.roots.contains(input.request.object.root) {
            return Err(UploadError::Root(RootClaimError::RootAlreadyClaimed).into());
        }
        admission_capacity::check(
            global.view().ok_or(UploadStoreError::InvalidRecord)?,
            tenant.view().ok_or(UploadStoreError::InvalidRecord)?,
            self.config.limits().catalog,
            self.config.limits().uploads,
            input.request.object.bytes,
        )?;
        let record = UploadPermissionRecord::new(input, now, generation);
        global.admit(input.request.object.bytes);
        tenant.admit(input.request.object.bytes);
        self.roots
            .claim(
                context,
                input.request.object.root,
                input.request.object.first.object(),
            )
            .map_err(root_admission)?;
        self.permissions
            .insert(key(input.request), UploadStoreRecord::Permission(record));
        self.root_requests.insert(
            *input.request.object.root.as_bytes(),
            input.request.id.get().get(),
        );
        self.usage.insert(context.actor, tenant);
        self.usage.insert(Principal::management_canister(), global);
        Ok(UploadAdmission::Reserved)
    }

    /// Bind a validated immutable declaration without transferring file bytes.
    /// # Errors
    /// Rejects scope/uploader/generation/time/phase errors, changed declarations or fence.
    /// # Panics
    /// Stable write failures must propagate to the IC transaction.
    pub fn prepare_manifest(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
        input: UploadManifest<'_>,
        now: u64,
    ) -> Result<LifecycleChange, UploadStoreError> {
        validation::object(&self.config, context, request.object.first.object())?;
        self.mutable()?;
        let mut record = self.required(request)?;
        self.uploader(context, &record, now)?;
        manifest::validate(&self.config, request, input)?;
        if let Some(existing) = self.manifests.get(&key(request)) {
            if !existing.matches(input) {
                return Err(UploadAdmissionError::PermissionConflict.into());
            }
            return Ok(LifecycleChange::Unchanged);
        }
        record.bind_manifest();
        self.manifests
            .insert(key(request), UploadManifestRecord::new(input));
        self.permissions
            .insert(key(request), UploadStoreRecord::Permission(record));
        Ok(LifecycleChange::Changed)
    }

    /// Persist possible exposure before any future certificate could escape.
    /// This returns no certificate and does not qualify provider enforcement.
    /// Hosts use `workflow::uploads::exposure` for the additional provider/recovery
    /// gates; this primitive alone checks only local bookkeeping prerequisites.
    /// # Errors
    /// Rejects wrong uploader/activation/time, missing manifest, repeated exposure or fence.
    /// # Panics
    /// Stable write failures must roll back the containing IC update.
    pub fn expose(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
        now: u64,
    ) -> Result<(), UploadStoreError> {
        let mut record = self.exposure_record(context, request, now)?;
        record.expose();
        self.permissions
            .insert(key(request), UploadStoreRecord::Permission(record));
        Ok(())
    }
    // Shared by inspection and commit; an inspection can never bypass the current
    // restore fence, current activation, original uploader/time or prepared phase.
    fn exposure_record(
        &self,
        context: UploadContext,
        request: UploadRequest,
        now: u64,
    ) -> Result<UploadPermissionRecord, UploadStoreError> {
        validation::object(&self.config, context, request.object.first.object())?;
        self.mutable()?;
        let record = self.required(request)?;
        let view = self.uploader(context, &record, now)?;
        if view.manifest != UploadManifestState::Bound {
            return Err(UploadAdmissionError::ManifestNotPrepared.into());
        }
        Ok(record)
    }

    /// Revoke local issuance; only an unexposed reservation releases byte capacity.
    /// Exposed uncertainty, cancelled IDs, root claims and leaf budgets remain retained.
    /// # Errors
    /// Rejects wrong tenant/scope, missing/changed permission or restored owner.
    /// # Panics
    /// Stable write failures must roll back all permission and counter writes.
    pub fn revoke(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadStoreError> {
        self.project(context, request)?;
        self.mutable()?;
        let mut record = self.required(request)?;
        let before = record.view().ok_or(UploadStoreError::InvalidRecord)?;
        let change = record.revoke();
        if change == LifecycleChange::Unchanged {
            return Ok(change);
        }
        let mut global = self.total(Principal::management_canister())?;
        let mut tenant = self.total(context.actor)?;
        if before.phase == UploadPhase::Reserved {
            global.cancel(request.object.bytes);
            tenant.cancel(request.object.bytes);
        }
        self.permissions
            .insert(key(request), UploadStoreRecord::Permission(record));
        self.usage.insert(context.actor, tenant);
        self.usage.insert(Principal::management_canister(), global);
        Ok(change)
    }

    /// Exact permission inspection by its tenant or original uploader, including after restore.
    /// # Errors
    /// Rejects wrong binding, missing/changed operation or unrelated observers.
    pub fn lookup(
        &self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<UploadPermissionView, UploadStoreError> {
        validation::object(&self.config, context, request.object.first.object())?;
        let view = self
            .required(request)?
            .view()
            .ok_or(UploadStoreError::InvalidRecord)?;
        if context.actor != request.object.first.object().tenant()
            && context.actor != view.permission.uploader
        {
            return Err(UploadAdmissionError::NotObserver.into());
        }
        Ok(view)
    }
    /// Global charged totals for a trusted host, not an authenticated public endpoint.
    /// # Errors
    /// Rejects missing or malformed retained counters.
    pub fn usage(&self) -> Result<UploadUsage, UploadStoreError> {
        self.total(Principal::management_canister())?
            .view()
            .ok_or(UploadStoreError::InvalidRecord)
    }
    /// Tenant charged totals for trusted host accounting; hosts must authorize disclosure.
    /// # Errors
    /// Rejects malformed retained counters.
    pub fn tenant_usage(&self, tenant: Principal) -> Result<UploadUsage, UploadStoreError> {
        self.total(tenant)?
            .view()
            .ok_or(UploadStoreError::InvalidRecord)
    }
    /// Whether mutation is fenced; only qualified installation recovery clears it.
    #[must_use]
    pub const fn is_fenced(&self) -> bool {
        self.fenced
    }

    fn permission(
        &self,
        request: UploadRequest,
    ) -> Result<Option<UploadPermissionRecord>, UploadStoreError> {
        match self.permissions.get(&key(request)) {
            None => Ok(None),
            Some(UploadStoreRecord::Permission(record)) => {
                let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
                if view.permission.request != request {
                    return Err(UploadAdmissionError::PermissionConflict.into());
                }
                Ok(Some(record))
            }
            Some(_) => Err(UploadStoreError::InvalidRecord),
        }
    }
    fn required(&self, request: UploadRequest) -> Result<UploadPermissionRecord, UploadStoreError> {
        self.permission(request)?
            .ok_or(UploadAdmissionError::UnknownPermission.into())
    }
    fn total(&self, tenant: Principal) -> Result<UploadUsageRecord, UploadStoreError> {
        match self.usage.get(&tenant) {
            Some(record) if record.view().is_some() => Ok(record),
            None if tenant != Principal::management_canister() => Ok(UploadUsageRecord::empty()),
            _ => Err(UploadStoreError::InvalidRecord),
        }
    }
    fn generation(&self, tenant: Principal) -> Result<NonZeroU64, UploadStoreError> {
        let entry = self
            .tenants
            .enrollment_for_owner(tenant)?
            .ok_or(UploadAdmissionError::Tenant(TenantError::NotEnrolled))?;
        if !entry.active {
            return Err(UploadAdmissionError::Tenant(TenantError::Suspended).into());
        }
        Ok(entry.generation)
    }
    fn uploader(
        &self,
        context: UploadContext,
        record: &UploadPermissionRecord,
        now: u64,
    ) -> Result<UploadPermissionView, UploadStoreError> {
        let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
        // Preserve role/revocation precedence before consulting enrollment storage.
        if context.actor != view.permission.uploader {
            return Err(UploadAdmissionError::NotUploader.into());
        }
        if view.revoked {
            return Err(UploadAdmissionError::Revoked.into());
        }
        let generation = self.generation(view.permission.request.object.first.object().tenant())?;
        validation::uploader(context, &view, Ok(generation), now)?;
        Ok(view)
    }
    fn project(
        &self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<(), UploadStoreError> {
        validation::object(&self.config, context, request.object.first.object())?;
        if context.actor != request.object.first.object().tenant() {
            return Err(UploadAdmissionError::NotProject.into());
        }
        Ok(())
    }
    fn mutable(&self) -> Result<(), UploadStoreError> {
        if self.fenced {
            Err(UploadStoreError::Fenced)
        } else {
            Ok(())
        }
    }
}
fn root_admission(error: RootStoreError) -> UploadStoreError {
    match error {
        RootStoreError::Claim(error) => UploadError::Root(error).into(),
        other => UploadStoreError::Roots(other),
    }
}

/// Typed rejection before mutation. Storage failures trap for IC rollback.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadStoreError {
    /// At least one memory is allocated; installation cannot reset history.
    #[error("upload memory allocated")]
    AlreadyAllocated,
    /// One of the required memories is absent.
    #[error("upload memory missing")]
    Missing,
    /// Retained configuration differs from the candidate.
    #[error("upload configuration mismatch")]
    Binding,
    /// Records, indexes, manifests or counters are inconsistent.
    #[error("invalid upload state")]
    InvalidRecord,
    /// Candidate permits a manifest larger than this store's 64 KiB record bound.
    #[error("upload manifest envelope exceeds codec bound")]
    UnsupportedEnvelope,
    /// Restoration never releases mutation authority from its own counters.
    #[error("upload owner fenced")]
    Fenced,
    /// Shared service validation or quota rejection.
    #[error(transparent)]
    Admission(#[from] UploadAdmissionError),
    /// Enrollment storage rejection.
    #[error(transparent)]
    Tenants(#[from] TenantStoreError),
    /// Root storage rejection.
    #[error(transparent)]
    Roots(#[from] RootStoreError),
    /// Exact reference request admission rejected without recording a receipt.
    #[error(transparent)]
    Reference(#[from] crate::model::lifecycle::requests::ReferenceRequestError),
    /// Confirmed lifecycle transition rejected.
    #[error(transparent)]
    Lifecycle(#[from] crate::model::lifecycle::LifecycleError),
    /// Pagination cursor differs from the requested service, scope or filter.
    #[error("upload scan cursor scope mismatch")]
    CursorScope,
}
impl From<UploadError> for UploadStoreError {
    fn from(error: UploadError) -> Self {
        Self::Admission(error.into())
    }
}

#[cfg(test)]
mod tests;
