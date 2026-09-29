//! Transient project-authorized uploader admissions over the shared upload catalog.
//!
//! No certificate, provider call, persistence or restoration
//! occurs here. Hosts must durably commit exposure before returning a certificate.
use std::{collections::BTreeMap, num::NonZeroU64};

use candid::Principal;
use thiserror::Error;

pub(crate) mod capacity;
pub mod completion;
pub mod content;
pub mod download;
pub(crate) mod record;
pub(crate) mod validation;
pub use capacity::UploadManifestLimit;
pub mod manifest;
pub mod planning;
pub use manifest::UploadManifestState;

use super::{
    configuration::ServiceConfiguration,
    tenant::{TenantEnrollmentView, TenantEnrollments, TenantError, TenantUpdate},
};
use crate::model::{
    catalog::{
        CatalogError,
        admission::{
            UploadAdmission, UploadCatalog, UploadError, UploadPhase, UploadRequest,
            UploadRequestId,
        },
    },
    identity::{ProviderRootHash, caffeine::manifest::CaffeineManifestError},
    lifecycle::{
        LifecycleChange,
        binding::ObjectBinding,
        requests::{
            ReferenceOperation, ReferenceReceiptView, ReferenceRequest, ReferenceRequestOutcome,
        },
    },
};

/// Authenticated execution context, supplied by the host rather than request data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadContext {
    /// Actual running service.
    pub service: Principal,
    /// Actual caller: operator for enrollment, project for admission/references,
    /// uploader for exposure. No role is inferred from controller status.
    pub actor: Principal,
}

/// Exact project-approved operation. No wildcard root, account or uploader exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadPermission {
    /// Full tenant/service/namespace/object/reference identity, root and declared size.
    /// A bounded manifest must be bound before local exposure.
    pub request: UploadRequest,
    /// Browser/session principal approved by the tenant project.
    pub uploader: Principal,
    /// Exclusive local certificate-issuance deadline in host nanoseconds.
    /// Expiry cannot revoke an escaped certificate or release uncertain capacity.
    pub expires_at_ns: u64,
}

/// Passive exact-operation observation; no result grants fresh certificate authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadPermissionView {
    /// Manifest consistency only, separately from provider completion.
    pub manifest: UploadManifestState,
    /// Original immutable project instruction.
    pub permission: UploadPermission,
    /// Local admission time, not provider completion time.
    pub admitted_at_ns: u64,
    /// Enrollment activation at admission; reactivation never renews this value.
    pub tenant_generation: NonZeroU64,
    /// Project revocation; escaped effects remain accounted separately.
    pub revoked: bool,
    /// Current catalog state, retaining uncertainty and confirmed reference history.
    pub phase: UploadPhase,
}

#[derive(Debug)]
struct Permission {
    manifest: Option<manifest::PreparedManifest>,
    original: UploadPermission,
    admitted_at_ns: u64,
    tenant_generation: NonZeroU64,
    revoked: bool,
}

/// Single owner of bounded permissions, reservations and confirmed references.
///
/// Each permission consumes its catalog's lifetime operation slot, including
/// cancellation, and reserves manifest leaves from its exact declared size.
/// No mutable catalog escape, clone, reset or restore is provided.
/// Only the configured operator can enroll a project. Enrollment and suspension
/// share this owner; the host must still enforce recovery fences before effects.
#[derive(Debug)]
pub struct UploadAdmissions {
    config: ServiceConfiguration,
    catalog: UploadCatalog,
    permissions: BTreeMap<(Principal, UploadRequestId), Permission>,
    // One immutable mapping per accepted permission, retained through settlement.
    permission_roots: BTreeMap<ProviderRootHash, (Principal, UploadRequestId)>,
    tenants: TenantEnrollments,
    manifest_accounting: capacity::ManifestAccounting,
}

impl UploadAdmissions {
    /// Construct a fresh transient owner, never reconstruct an existing installation.
    /// # Panics
    /// Panics if internal configuration and catalog identity validation disagree.
    #[must_use]
    pub fn new(config: ServiceConfiguration) -> Self {
        Self {
            catalog: UploadCatalog::new(
                config.bindings().service,
                config.limits().catalog,
                config.limits().uploads,
            )
            .expect("configuration validates service"),
            config,
            permissions: BTreeMap::new(),
            permission_roots: BTreeMap::new(),
            tenants: TenantEnrollments::new(config.limits().max_tenants),
            manifest_accounting: capacity::ManifestAccounting::default(),
        }
    }

    /// Enroll, suspend or reactivate a tenant using the exact observed state.
    ///
    /// A lost reply requires inspection: replaying a stale precondition conflicts
    /// instead of reapplying an old instruction. Suspension keeps all obligations
    /// and history. Reactivation does not renew old uploader permissions.
    /// # Errors
    /// Rejects wrong service/operator, invalid tenant, stale precondition, full
    /// lifetime enrollment capacity or exhausted activation generations.
    pub fn update_tenant(
        &mut self,
        context: UploadContext,
        update: TenantUpdate,
    ) -> Result<TenantEnrollmentView, UploadAdmissionError> {
        if context.service != self.config.bindings().service {
            return Err(UploadAdmissionError::WrongService);
        }
        if context.actor != self.config.bindings().operator {
            return Err(UploadAdmissionError::NotOperator);
        }
        Ok(self.tenants.update(update)?)
    }

    /// Inspect enrollment as the configured operator or the exact tenant.
    /// # Errors
    /// Rejects wrong service or an unrelated observer, including uploaders.
    pub fn tenant(
        &self,
        context: UploadContext,
        tenant: Principal,
    ) -> Result<Option<TenantEnrollmentView>, UploadAdmissionError> {
        if context.service != self.config.bindings().service {
            return Err(UploadAdmissionError::WrongService);
        }
        if context.actor != tenant && context.actor != self.config.bindings().operator {
            return Err(UploadAdmissionError::NotObserver);
        }
        Ok(self.tenants.get(tenant))
    }

    /// Reserve before the approved uploader can request certificate exposure.
    ///
    /// Exact retries return retained catalog state even after expiry or revocation;
    /// they never renew a deadline or re-enable exposure, including after tenant
    /// suspension or reactivation. Fresh admissions require active enrollment.
    /// Fresh admission reserves global and tenant manifest capacity before
    /// catalog mutation; manifest preparation cannot compete for those slots later.
    /// Cancellation and settlement preserve this lifetime history budget.
    /// The host supplies time.
    /// # Errors
    /// Rejects wrong service/project/namespace, changed arguments, invalid uploader,
    /// expired fresh permission, oversized objects, exhausted manifest history
    /// or underlying catalog admission. Rejection consumes no capacity or identity.
    pub fn admit(
        &mut self,
        context: UploadContext,
        input: UploadPermission,
        now_ns: u64,
    ) -> Result<UploadAdmission, UploadAdmissionError> {
        self.check_project(context, input.request)?;
        if let Some(existing) = self.permissions.get(&key(input.request)) {
            if existing.original != input {
                return Err(UploadAdmissionError::PermissionConflict);
            }
            return Ok(UploadAdmission::Existing(
                self.catalog.phase(context.actor, input.request)?,
            ));
        }
        let tenant_generation = self.tenants.active_generation(context.actor)?;
        validation::fresh(&self.config, input, now_ns)?;
        self.check_manifest_capacity(context.actor, input.request.object.bytes)?;
        let outcome = self.catalog.reserve(context.actor, input.request)?;
        self.permissions.insert(
            key(input.request),
            Permission {
                manifest: None,
                original: input,
                admitted_at_ns: now_ns,
                tenant_generation,
                revoked: false,
            },
        );
        self.manifest_accounting
            .admit(context.actor, input.request.object.bytes);
        self.permission_roots
            .insert(input.request.object.root, key(input.request));
        Ok(outcome)
    }

    /// Mark possible certificate exposure once, by the exact approved uploader.
    ///
    /// Success is a local transition that MUST be persisted before any certificate
    /// response escapes. It is not upload completion, a paid retry permit or proof
    /// of gateway enforcement. A repeated update is denied after exposure; lost
    /// ingress/certificate results require exact result recovery or reconciliation.
    /// The manifest must match the admitted root and declared size. This does not
    /// prove actual byte length. Before issuing a real certificate, the host must
    /// establish provider pre-charge size/tree enforcement, namespace/replay
    /// guarantees, recovery eligibility and durable commit.
    /// # Errors
    /// Rejects wrong context, unknown/changed operation, another uploader, revoked
    /// or expired permission, backwards time, a non-reserved phase or missing manifest.
    pub fn expose(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
        now_ns: u64,
    ) -> Result<(), UploadAdmissionError> {
        let permission = self.check_uploader(context, request, now_ns)?;
        if permission.manifest.is_none() {
            return Err(UploadAdmissionError::ManifestNotPrepared);
        }
        self.catalog
            .mark_exposure_possible(request.object.first.object().tenant(), request)?;
        Ok(())
    }

    fn check_uploader(
        &self,
        context: UploadContext,
        request: UploadRequest,
        now_ns: u64,
    ) -> Result<&Permission, UploadAdmissionError> {
        self.check_service(context, request)?;
        let permission = self.permission(request)?;
        let tenant = request.object.first.object().tenant();
        let view = UploadPermissionView {
            manifest: if permission.manifest.is_some() {
                UploadManifestState::Bound
            } else {
                UploadManifestState::Unprepared
            },
            permission: permission.original,
            admitted_at_ns: permission.admitted_at_ns,
            tenant_generation: permission.tenant_generation,
            revoked: permission.revoked,
            phase: self.catalog.phase(tenant, request)?,
        };
        validation::uploader(
            context,
            &view,
            self.tenants.active_generation(tenant),
            now_ns,
        )?;
        Ok(permission)
    }

    /// Resolve a root-only certificate request to its original admitted operation.
    ///
    /// The shared catalog retains exclusive root claims, so a root cannot select
    /// a newer operation after cancellation or release. A retained root index
    /// selects the original permission in logarithmic time, then delegates to the
    /// same uploader/deadline/exposure checks as `expose`. The index grants no
    /// authority by itself; production resource budgets remain to be qualified.
    /// # Errors
    /// Rejects wrong service, unknown root and every exact exposure rejection.
    /// # Panics
    /// Panics if the private root index points to a missing permission, indicating
    /// an internal invariant violation rather than rejected caller input.
    pub fn expose_root(
        &mut self,
        context: UploadContext,
        root: ProviderRootHash,
        now_ns: u64,
    ) -> Result<UploadRequest, UploadAdmissionError> {
        if context.service != self.config.bindings().service {
            return Err(UploadAdmissionError::WrongService);
        }
        let permission_key = self
            .permission_roots
            .get(&root)
            .ok_or(UploadAdmissionError::UnknownPermission)?;
        let request = self
            .permissions
            .get(permission_key)
            .expect("indexed permission")
            .original
            .request;
        self.expose(context, request, now_ns)?;
        Ok(request)
    }

    /// Block future local issuance. Only unexposed reservations can be cancelled.
    ///
    /// Revocation after exposure retains all uncertain bytes and operation history;
    /// it cannot recall a provider certificate or deny a later valid completion.
    /// # Errors
    /// Rejects wrong project/service/namespace or an unknown/conflicting operation.
    pub fn revoke(
        &mut self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadAdmissionError> {
        self.check_project(context, request)?;
        let permission = self
            .permissions
            .get_mut(&key(request))
            .ok_or(UploadAdmissionError::UnknownPermission)?;
        if permission.original.request != request {
            return Err(UploadAdmissionError::PermissionConflict);
        }
        if permission.revoked {
            return Ok(LifecycleChange::Unchanged);
        }
        if self.catalog.phase(context.actor, request)? == UploadPhase::Reserved {
            self.catalog.cancel(context.actor, request)?;
        }
        permission.revoked = true;
        Ok(LifecycleChange::Changed)
    }

    /// Inspect the original permission without renewal, dispatch or new allocation.
    ///
    /// Only its project or exact uploader can inspect it through this boundary.
    /// Expired and revoked entries remain visible; exposure uncertainty cannot
    /// be turned into an absent operation by losing a certificate/consumer reply.
    /// # Errors
    /// Rejects wrong scope, unknown/changed operation or an unrelated observer.
    pub fn lookup(
        &self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<UploadPermissionView, UploadAdmissionError> {
        self.check_service(context, request)?;
        let permission = self.permission(request)?;
        let tenant = request.object.first.object().tenant();
        if context.actor != tenant && context.actor != permission.original.uploader {
            return Err(UploadAdmissionError::NotObserver);
        }
        Ok(UploadPermissionView {
            manifest: if permission.manifest.is_some() {
                UploadManifestState::Bound
            } else {
                UploadManifestState::Unprepared
            },
            permission: permission.original,
            admitted_at_ns: permission.admitted_at_ns,
            tenant_generation: permission.tenant_generation,
            revoked: permission.revoked,
            phase: self.catalog.phase(tenant, request)?,
        })
    }

    /// Apply independently authenticated provider completion and exact stored size.
    ///
    /// The browser's success/progress cannot supply that evidence. Confirmation
    /// retains the initial reference even if consumer asset registration fails.
    /// Repeating completion never allocates another reference or resurrects release.
    /// # Errors
    /// Propagates exact catalog identity and phase errors.
    pub fn confirm_upload(
        &mut self,
        request: UploadRequest,
    ) -> Result<LifecycleChange, UploadError> {
        self.catalog.confirm_upload(request)
    }

    /// Apply an exact project reference operation through the same catalog.
    ///
    /// Fresh retains require active enrollment. Suspended projects can still
    /// release references and recover exact receipts, including prior retains;
    /// replay never allocates a reference. Operator status is not tenant authority.
    /// # Errors
    /// Rejects wrong context/binding, inactive fresh retains, or catalog admission.
    pub fn apply_reference(
        &mut self,
        context: UploadContext,
        root: ProviderRootHash,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, UploadAdmissionError> {
        if let Some(receipt) = self.reference_receipt(context, root, request)? {
            return Ok(ReferenceRequestOutcome::Replayed {
                result: receipt.result,
            });
        }
        if matches!(request.operation, ReferenceOperation::Retain(_)) {
            self.tenants.active_generation(context.actor)?;
        }
        Ok(self.catalog.apply_reference(root, context.actor, request)?)
    }

    /// Inspect the original result for an exact tenant reference operation.
    ///
    /// The same authority and payload checks protect reads and mutation retries.
    /// Suspension, full history and settlement do not erase receipts. Reading an
    /// absent receipt neither records a failure nor applies the operation. A past
    /// success is not current reference liveness; absence is only an observation
    /// of this owner, never authority to repeat an uncertain provider effect.
    /// # Errors
    /// Rejects wrong service/tenant/namespace/object binding, unknown roots and
    /// changed arguments for a recorded operation identity.
    pub fn reference_receipt(
        &self,
        context: UploadContext,
        root: ProviderRootHash,
        request: ReferenceRequest,
    ) -> Result<Option<ReferenceReceiptView>, UploadAdmissionError> {
        let object = match request.operation {
            ReferenceOperation::Retain(key) | ReferenceOperation::Release(key) => key.object(),
        };
        self.check_object(context, object)?;
        if context.actor != object.tenant() {
            return Err(UploadAdmissionError::NotProject);
        }
        let journal = self
            .catalog
            .confirmed()
            .get(root)
            .ok_or(CatalogError::UnknownRoot)?;
        Ok(journal
            .receipt(context.actor, request)
            .map_err(CatalogError::from)?)
    }

    /// Apply exact authenticated physical-deletion evidence, separately from billing.
    /// # Errors
    /// Propagates catalog binding and lifecycle errors.
    pub fn confirm_provider_deleted(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        self.catalog.confirm_provider_deleted(root, object)
    }

    /// Apply independently established final billing cessation for this object.
    /// # Errors
    /// Propagates catalog binding and lifecycle errors.
    pub fn confirm_billing_stopped(
        &mut self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, CatalogError> {
        self.catalog.confirm_billing_stopped(root, object)
    }

    /// Read-only catalog for existing tenant/gateway policies and operation lookup.
    /// The host must authenticate access before disclosing these observations.
    #[must_use]
    pub const fn catalog(&self) -> &UploadCatalog {
        &self.catalog
    }

    fn check_service(
        &self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<(), UploadAdmissionError> {
        self.check_object(context, request.object.first.object())
    }

    fn check_object(
        &self,
        context: UploadContext,
        object: ObjectBinding,
    ) -> Result<(), UploadAdmissionError> {
        validation::object(&self.config, context, object)
    }

    fn check_project(
        &self,
        context: UploadContext,
        request: UploadRequest,
    ) -> Result<(), UploadAdmissionError> {
        self.check_service(context, request)?;
        if context.actor != request.object.first.object().tenant() {
            return Err(UploadAdmissionError::NotProject);
        }
        Ok(())
    }

    fn permission(&self, request: UploadRequest) -> Result<&Permission, UploadAdmissionError> {
        let permission = self
            .permissions
            .get(&key(request))
            .ok_or(UploadAdmissionError::UnknownPermission)?;
        if permission.original.request != request {
            return Err(UploadAdmissionError::PermissionConflict);
        }
        Ok(permission)
    }
}

fn key(request: UploadRequest) -> (Principal, UploadRequestId) {
    (request.object.first.object().tenant(), request.id)
}

/// Admission or certificate-exposure rejection; never completion or refund evidence.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadAdmissionError {
    /// Lifetime manifest capacity is exhausted; no catalog reservation was made.
    #[error("upload manifest capacity exhausted: {0:?}")]
    ManifestCapacity(UploadManifestLimit),
    /// No manifest is bound to this admitted operation yet.
    #[error("upload manifest is not prepared")]
    ManifestNotPrepared,
    /// Manifest exceeds configured bounds or disagrees with the admitted root/length.
    #[error(transparent)]
    Manifest(#[from] CaffeineManifestError),
    /// Service header syntax or declared-length binding failed.
    #[error(transparent)]
    Metadata(#[from] manifest::UploadMetadataError),
    /// Empty provider objects are outside the maintained content contract.
    #[error("empty provider objects are unsupported")]
    EmptyObject,
    /// Tenant enrollment is managed only by the explicitly configured operator.
    #[error("caller is not the service operator")]
    NotOperator,
    /// Enrollment or activation-generation rejection.
    #[error(transparent)]
    Tenant(#[from] TenantError),
    /// Reference catalog identity, receipt or capacity rejection.
    #[error(transparent)]
    Reference(#[from] CatalogError),
    /// Actual and configured service differ.
    #[error("wrong upload service")]
    WrongService,
    /// Request is outside the configured namespace.
    #[error("wrong upload namespace")]
    WrongNamespace,
    /// Only the authenticated tenant project can admit or revoke its uploads.
    #[error("caller is not the tenant project")]
    NotProject,
    /// Uploader cannot be anonymous or management.
    #[error("invalid uploader")]
    InvalidUploader,
    /// No retained grant for this tenant operation.
    #[error("unknown upload permission")]
    UnknownPermission,
    /// An existing ID cannot change upload arguments, uploader or deadline.
    #[error("upload permission conflict")]
    PermissionConflict,
    /// Caller differs from the exact admitted uploader.
    #[error("caller is not the admitted uploader")]
    NotUploader,
    /// Caller is not an allowed observer of the requested permission or enrollment.
    #[error("caller cannot inspect upload permission")]
    NotObserver,
    /// Project has withdrawn local issuance permission.
    #[error("upload permission revoked")]
    Revoked,
    /// Exclusive issuance deadline reached.
    #[error("upload permission expired")]
    Expired,
    /// Host clock is earlier than admission.
    #[error("upload clock precedes admission")]
    ClockReversed,
    /// Configured individual-object limit exceeded before reservation.
    #[error("object exceeds configured limit")]
    ObjectTooLarge,
    /// Certificate issuance requires a still-unexposed reservation.
    #[error("upload is not reserved: {0:?}")]
    NotReserved(UploadPhase),
    /// Shared catalog admission, binding or capacity rejection.
    #[error(transparent)]
    Catalog(#[from] UploadError),
}

#[cfg(test)]
mod tests;
