//! Validate installation inputs together before a host can consider installation.
use std::num::{NonZeroU64, NonZeroU128, NonZeroUsize};

use candid::Principal;
use thiserror::Error;

use crate::model::{
    billing::configuration::BillingConfiguration,
    catalog::{CatalogLimits, admission::UploadLimits},
    identity::{
        caffeine::{CAFFEINE_CHUNK_BYTES, manifest::CaffeineManifestLimits},
        verification::ContentVerifier,
    },
};

/// Explicit candidate identities. None is inferred from controllers or provider replies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceBindings {
    /// Service instance intended to own certificates and provider callbacks.
    pub service: Principal,
    /// Operator for installation management; this grants no tenant access.
    pub operator: Principal,
    /// Explicit expected payer; equality with service selects the self account.
    pub payment_account: Principal,
    /// Local identity for independently provisioned provider configuration.
    /// This is not a gateway project/bucket or proof of namespace allocation.
    pub namespace: NonZeroU128,
}

/// Candidate resource envelope; all values are explicit, with no production defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceLimits {
    /// Lifetime enrolled tenants, including suspended entries; slots are never recycled.
    pub max_tenants: NonZeroUsize,
    /// Largest individual object, distinct from a browser's aggregate upload batch.
    pub max_object_bytes: NonZeroU64,
    /// Maximum metadata entries retained per prepared lifetime object slot.
    pub max_headers: NonZeroUsize,
    /// Retained UTF-8 metadata bytes plus three framing bytes per entry, per
    /// prepared lifetime object slot. Allocation overhead is additional.
    pub max_header_bytes: NonZeroUsize,
    /// Separate retained manifest capacity, reserved before accepting an upload.
    pub manifests: ServiceManifestLimits,
    /// Lifetime metadata and outstanding byte-obligation limits.
    pub catalog: CatalogLimits,
    /// Concurrent reservation limits, sharing the catalog's lifetime object slots.
    pub uploads: UploadLimits,
}

/// Lifetime leaf capacity, independent of released byte quota or concurrent uploads.
///
/// Each admitted object reserves `ceil(bytes / 1 MiB)` chunks permanently. A
/// prepared operation owns one array of 32-byte leaf hashes. Exposed and
/// cancelled operations keep their reservation. These counts bound
/// leaf payloads, not allocator overhead, other metadata or total canister memory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceManifestLimits {
    /// Global retained chunk slots across all tenants and operation phases.
    pub max_chunks: NonZeroUsize,
    /// Retained chunk slots for one tenant, including cancelled history.
    pub max_tenant_chunks: NonZeroUsize,
}

/// Internally consistent candidate, not permission to install, fund or serve.
///
/// This value has no serialization, default, state registration or lifecycle hooks.
/// The host still needs tenant enrollment, provider project/bucket authority,
/// qualified transport/serving limits, recovery identity and explicit installation
/// authorization. A different payer requires separately verified relationship
/// authority; merely naming it grants none.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceConfiguration {
    bindings: ServiceBindings,
    limits: ServiceLimits,
    billing: BillingConfiguration,
}

impl ServiceConfiguration {
    /// Validate every binding and cross-limit relation without allocating state.
    ///
    /// Metadata counts must fit Wasm32 even on a wider host. Receipt capacity must
    /// fit the first reference's release plus retain/release for every additional
    /// advertised reference. Failed or extra requests may still consume history;
    /// runtime admission continues to reserve release slots independently.
    /// # Errors
    /// Rejects special principals, nonportable counts or contradictory limits.
    pub fn new(
        bindings: ServiceBindings,
        limits: ServiceLimits,
        billing: BillingConfiguration,
    ) -> Result<Self, ServiceConfigurationError> {
        for (principal, field) in [
            (bindings.service, ServicePrincipal::Service),
            (bindings.operator, ServicePrincipal::Operator),
            (bindings.payment_account, ServicePrincipal::PaymentAccount),
        ] {
            if principal == Principal::anonymous() || principal == Principal::management_canister()
            {
                return Err(ServiceConfigurationError::InvalidPrincipal { field });
            }
        }
        validate_limits(limits)?;
        if billing.gateway_limits().max_unique > billing.gateway_limits().max_entries {
            return Err(ServiceLimitError::GatewayUniqueExceedsEntries.into());
        }
        Ok(Self {
            bindings,
            limits,
            billing,
        })
    }

    /// Exact configured identities, without authenticating their real-world roles.
    #[must_use]
    pub const fn bindings(self) -> ServiceBindings {
        self.bindings
    }

    /// Validated limits consumed by shared catalog/admission handlers.
    #[must_use]
    pub const fn limits(self) -> ServiceLimits {
        self.limits
    }

    /// Explicit Cashier and billing limits; not spendability or account credit.
    #[must_use]
    pub const fn billing(self) -> BillingConfiguration {
        self.billing
    }

    /// Manifest budgets derived from the same object limit used for admission.
    ///
    /// A largest admitted object must fit the retained leaf count. The upload owner
    /// uses these budgets before allocating bounded metadata/hash state;
    /// provider enforcement and certificate exposure remain separate requirements.
    /// # Panics
    /// Panics only if construction-time validation of the derived count disagrees.
    #[must_use]
    pub fn manifest_limits(self) -> CaffeineManifestLimits {
        CaffeineManifestLimits {
            max_content_bytes: self.limits.max_object_bytes,
            max_chunks: NonZeroUsize::new(
                usize::try_from(chunk_count(self.limits)).expect("validated chunk count"),
            )
            .expect("positive object length"),
            max_headers: self.limits.max_headers,
            max_header_bytes: self.limits.max_header_bytes,
        }
    }
}

fn chunk_count(limits: ServiceLimits) -> u64 {
    limits
        .max_object_bytes
        .get()
        .div_ceil(CAFFEINE_CHUNK_BYTES as u64)
}

fn validate_limits(limits: ServiceLimits) -> Result<(), ServiceConfigurationError> {
    if limits.max_object_bytes.get() > ContentVerifier::MAX_BYTES {
        return Err(ServiceLimitError::ObjectExceedsHashLength.into());
    }
    if u32::try_from(chunk_count(limits)).is_err() {
        return Err(ServiceConfigurationError::CountOutOfRange {
            field: ServiceCount::ManifestChunks,
        });
    }
    let catalog = limits.catalog;
    let uploads = limits.uploads;
    for (value, field) in [
        (limits.max_tenants, ServiceCount::Tenants),
        (limits.max_headers, ServiceCount::Headers),
        (limits.max_header_bytes, ServiceCount::HeaderBytes),
        (
            limits.manifests.max_chunks,
            ServiceCount::ManifestChunksRetained,
        ),
        (
            limits.manifests.max_tenant_chunks,
            ServiceCount::TenantManifestChunks,
        ),
        (catalog.max_objects, ServiceCount::Objects),
        (catalog.max_tenant_objects, ServiceCount::TenantObjects),
        (catalog.max_references_per_object, ServiceCount::References),
        (catalog.max_receipts_per_object, ServiceCount::Receipts),
        (uploads.max_active, ServiceCount::Uploads),
        (uploads.max_tenant_active, ServiceCount::TenantUploads),
    ] {
        if u32::try_from(value.get()).is_err() {
            return Err(ServiceConfigurationError::CountOutOfRange { field });
        }
    }
    // A 32-byte leaf array must fit below Wasm32's isize::MAX.
    // Other state and allocator overhead still need separate deployment budgets.
    if limits.manifests.max_chunks.get() as u64 > i32::MAX as u64 / 32 {
        return Err(ServiceLimitError::ManifestExceedsAddressSpace.into());
    }
    let relations = [
        (
            (limits.max_header_bytes.get() as u64)
                < ("Content-Length".len() + 3) as u64
                    + u64::from(limits.max_object_bytes.get().ilog10() + 1),
            ServiceLimitError::InsufficientLengthMetadata,
        ),
        (
            limits.manifests.max_tenant_chunks > limits.manifests.max_chunks,
            ServiceLimitError::TenantManifestExceedsGlobal,
        ),
        (
            chunk_count(limits) > limits.manifests.max_tenant_chunks.get() as u64,
            ServiceLimitError::ObjectExceedsManifest,
        ),
        (
            catalog.max_tenant_objects > catalog.max_objects,
            ServiceLimitError::TenantObjectsExceedGlobal,
        ),
        (
            uploads.max_active > catalog.max_objects,
            ServiceLimitError::UploadsExceedObjects,
        ),
        (
            uploads.max_tenant_active > uploads.max_active,
            ServiceLimitError::TenantUploadsExceedGlobal,
        ),
        (
            uploads.max_tenant_active > catalog.max_tenant_objects,
            ServiceLimitError::TenantUploadsExceedObjects,
        ),
        (
            u128::from(limits.max_object_bytes.get()) > catalog.max_physical_bytes.get(),
            ServiceLimitError::ObjectExceedsPhysical,
        ),
        (
            u128::from(limits.max_object_bytes.get()) > catalog.max_liability_bytes.get(),
            ServiceLimitError::ObjectExceedsLiability,
        ),
        (
            u128::from(limits.max_object_bytes.get()) > catalog.max_tenant_logical_bytes.get(),
            ServiceLimitError::ObjectExceedsTenantLogical,
        ),
    ];
    for (invalid, error) in relations {
        if invalid {
            return Err(error.into());
        }
    }
    // Counts already fit u32; calculate in u64 so host and Wasm agree.
    let references = u64::try_from(catalog.max_references_per_object.get()).expect("u32 count");
    let receipts = u64::try_from(catalog.max_receipts_per_object.get()).expect("u32 count");
    if receipts < references * 2 - 1 {
        return Err(ServiceLimitError::InsufficientReferenceReceipts.into());
    }
    Ok(())
}

/// Identity role rejected before a candidate can become validated configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServicePrincipal {
    /// Certificate/callback owner.
    Service,
    /// Explicit installation operator.
    Operator,
    /// Expected Cashier payer.
    PaymentAccount,
}

/// Collection count that cannot be represented on all supported targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceCount {
    /// Global lifetime manifest leaves.
    ManifestChunksRetained,
    /// Per-tenant lifetime manifest leaves.
    TenantManifestChunks,
    /// Derived maximum manifest leaves at the configured object size.
    ManifestChunks,
    /// Raw metadata entries per manifest.
    Headers,
    /// Raw framed metadata bytes per manifest.
    HeaderBytes,
    /// Lifetime enrolled tenants.
    Tenants,
    /// Global lifetime objects.
    Objects,
    /// Tenant lifetime objects.
    TenantObjects,
    /// Lifetime references per object.
    References,
    /// Lifetime receipts per object.
    Receipts,
    /// Global active uploads.
    Uploads,
    /// Tenant active uploads.
    TenantUploads,
}

/// Contradictory configured resource bounds; not a live capacity-exhaustion result.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ServiceLimitError {
    /// The largest object's required Content-Length plus framing cannot fit.
    #[error("metadata budget cannot fit required Content-Length")]
    InsufficientLengthMetadata,
    /// Retained leaf payloads alone would exceed the portable address space.
    #[error("manifest leaf arrays exceed Wasm32 address space")]
    ManifestExceedsAddressSpace,
    /// Tenant leaf capacity must fit global capacity.
    #[error("tenant manifest limit exceeds global limit")]
    TenantManifestExceedsGlobal,
    /// One largest object cannot fit its tenant's manifest budget.
    #[error("object manifest exceeds manifest capacity")]
    ObjectExceedsManifest,
    /// The declared object size cannot be hashed with the maintained algorithm.
    #[error("object limit exceeds SHA-256 length bound")]
    ObjectExceedsHashLength,
    /// Tenant object capacity must fit global capacity.
    #[error("tenant object limit exceeds global limit")]
    TenantObjectsExceedGlobal,
    /// Every upload reserves a lifetime object slot.
    #[error("upload limit exceeds object limit")]
    UploadsExceedObjects,
    /// Tenant upload capacity must fit global concurrency.
    #[error("tenant upload limit exceeds global limit")]
    TenantUploadsExceedGlobal,
    /// Tenant uploads also reserve tenant lifetime slots.
    #[error("tenant upload limit exceeds tenant object limit")]
    TenantUploadsExceedObjects,
    /// A largest permitted object cannot fit the physical budget.
    #[error("object limit exceeds physical byte limit")]
    ObjectExceedsPhysical,
    /// A largest permitted object cannot fit outstanding billing bytes.
    #[error("object limit exceeds liability byte limit")]
    ObjectExceedsLiability,
    /// A largest permitted object cannot fit a tenant's logical budget.
    #[error("object limit exceeds tenant logical byte limit")]
    ObjectExceedsTenantLogical,
    /// Minimum retain/release history for advertised references cannot fit.
    #[error("insufficient reference receipt capacity")]
    InsufficientReferenceReceipts,
    /// Distinct gateways cannot exceed raw list input capacity.
    #[error("unique gateway limit exceeds input entry limit")]
    GatewayUniqueExceedsEntries,
}

/// Rejected installation configuration; no live state has been accessed or changed.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ServiceConfigurationError {
    /// Anonymous or management principal cannot occupy this role.
    #[error("invalid service configuration principal: {field:?}")]
    InvalidPrincipal {
        /// Rejected identity role.
        field: ServicePrincipal,
    },
    /// Count does not fit Wasm32's collection index range.
    #[error("service count exceeds supported index range: {field:?}")]
    CountOutOfRange {
        /// Rejected collection count.
        field: ServiceCount,
    },
    /// Limits disagree before any resource is allocated.
    #[error(transparent)]
    Limits(#[from] ServiceLimitError),
}

#[cfg(test)]
mod tests;
