//! Upload boundary values, pure invariants and frozen durable schemas.
use ic_blob_storage_contracts::upload::binding::UploadPermission;

use std::num::NonZeroU64;

use candid::Principal;
use thiserror::Error;

pub(crate) mod capacity;
pub mod content;
pub mod download;
pub(crate) mod record;
pub(crate) mod validation;
pub use capacity::UploadManifestLimit;
pub mod manifest;
pub mod planning;
pub use manifest::UploadManifestState;

use crate::model::catalog::CatalogError;
use crate::model::catalog::admission::UploadError;
use crate::model::catalog::admission::UploadPhase;
use crate::model::lifecycle::LifecycleChange;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestError;
use ic_blob_storage_contracts::tenant::TenantError;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;

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
    Metadata(#[from] ic_blob_storage_contracts::upload::metadata::UploadMetadataError),
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
