//! Pure installation input validation; no records, memory, registration or authority.
use crate::configuration::billing::BillingConfiguration;
use crate::configuration::billing::BillingConfigurationError;
use crate::configuration::envelope::AllocationLimitError;
use crate::configuration::envelope::ConfigurationEnvelopeError;
use crate::configuration::envelope::validate_allocation;
use crate::configuration::envelope::validate_store_envelope;
use crate::configuration::funding::FundingLimits;
use crate::configuration::funding::FundingLimitsError;
use crate::configuration::limits::CatalogLimits;
use crate::configuration::limits::UploadLimits;
use crate::configuration::service::ServiceBindings;
use crate::configuration::service::ServiceConfiguration;
use crate::configuration::service::ServiceConfigurationError;
use crate::configuration::service::ServiceCount;
use crate::configuration::service::ServiceLimits;
use crate::configuration::service::ServiceManifestLimits;
use crate::download::scope::CaffeineDownloadScope;
use crate::download::scope::DownloadScopeError;
use crate::dto::configuration::ServiceConfigurationInput;
use crate::dto::configuration::ServiceFundingInput;
use crate::dto::configuration::ServiceReadInput;
use crate::upload::completion::CompletionAuthority;
use crate::upload::completion::InvalidCompletionAuthority;
use candid::Principal;
use candid::de::DecoderConfig;
use candid::decode_one_with_config;
use std::num::NonZeroU64;
use std::num::NonZeroU128;
use std::num::NonZeroUsize;
use thiserror::Error;

pub mod billing;
pub mod envelope;
pub mod funding;
pub mod limits;
pub mod service;
/// Validated immutable candidate fields, not installed state or memory grants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedConfiguration {
    /// Shared identities and resource ceilings.
    service: ServiceConfiguration,
    /// Explicit attachment limits; no accounting journal or liquidity is inferred.
    funding: ServiceFundingInput,
    /// Explicit occupancy limits; no session is allocated.
    reads: ServiceReadInput,
}
impl ValidatedConfiguration {
    /// Checked immutable identity/resource profile, with no storage authority.
    #[must_use]
    pub const fn service(&self) -> ServiceConfiguration {
        self.service
    }
    /// Checked allocation input; this is not live liquidity or spent accounting.
    #[must_use]
    pub const fn funding(&self) -> ServiceFundingInput {
        self.funding
    }
    /// Checked session ceilings; no occupancy or session is granted.
    #[must_use]
    pub const fn reads(&self) -> ServiceReadInput {
        self.reads
    }
}
/// Positive scalar fields outside the model's collection-count categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationScalar {
    /// Local provider namespace identity.
    Namespace,
    /// Largest admitted object size.
    ObjectBytes,
    /// Stored-byte budget.
    PhysicalBytes,
    /// Continuing billing-byte budget.
    LiabilityBytes,
    /// Tenant logical-byte budget.
    TenantLogicalBytes,
    /// Reserve within the local attachment allocation.
    FundingReserve,
}

/// Candidate rejection before any store is installed or opened.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ConfigurationInputError {
    /// Encoded Candid exceeds the caller's buffering budget.
    #[error("configuration input exceeds byte limit")]
    Limit,
    /// Invalid Candid, unsupported numeric width or exceeded decoding quota.
    #[error("invalid configuration encoding")]
    Encoding,
    /// The candidate names another service, not the host's actual canister.
    #[error("configuration service differs from host")]
    ServiceBinding,
    /// A collection limit is zero.
    #[error("configuration count must be positive: {0:?}")]
    ZeroCount(ServiceCount),
    /// A non-collection scalar is zero.
    #[error("configuration scalar must be positive: {0:?}")]
    ZeroScalar(ConfigurationScalar),
    /// Funding thresholds violate existing model invariants.
    #[error(transparent)]
    Funding(#[from] FundingLimitsError),
    /// Cashier identity or gateway limits are invalid.
    #[error(transparent)]
    Billing(#[from] BillingConfigurationError),
    /// Service identities or resource relationships are invalid.
    #[error(transparent)]
    Configuration(#[from] ServiceConfigurationError),
    /// Lifetime funding history must have positive portable capacity.
    #[error("funding attempt capacity must be positive and fit Wasm32")]
    FundingCapacity,
    /// Local attachment allocation cannot preserve its reserve.
    #[error(transparent)]
    Allocation(#[from] AllocationLimitError),
    /// A stable owner's resource/codec envelope is unsupported.
    #[error(transparent)]
    Stores(#[from] ConfigurationEnvelopeError),
}

/// Decode one bounded current configuration candidate and apply the shared model.
/// The host must also bound ingress buffering and authenticate installation authority.
/// `actual_service` must come from the platform, never from the candidate itself.
/// This is not a reconfiguration endpoint, persisted schema or authority to deploy.
/// # Errors
/// Rejects oversized/malformed Candid, service mismatch or invalid configuration.
pub fn decode_candidate(
    actual_service: Principal,
    bytes: &[u8],
    max_bytes: NonZeroUsize,
) -> Result<ValidatedConfiguration, ConfigurationInputError> {
    if bytes.len() > max_bytes.get() {
        return Err(ConfigurationInputError::Limit);
    }
    let mut decoder = DecoderConfig::new();
    decoder
        .set_decoding_quota(100_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max_bytes.get())
        .set_full_error_message(false);
    let input =
        decode_one_with_config(bytes, &decoder).map_err(|_| ConfigurationInputError::Encoding)?;
    validate_candidate(actual_service, input)
}

/// Convert explicit inputs without defaults, clamping, state access or provider calls.
/// Reuses the model's funding, billing and cross-resource validation.
/// # Errors
/// Rejects changed service identity, zero limits or invalid model relationships.
pub fn validate_candidate(
    actual_service: Principal,
    input: ServiceConfigurationInput,
) -> Result<ValidatedConfiguration, ConfigurationInputError> {
    if actual_service != input.service {
        return Err(ConfigurationInputError::ServiceBinding);
    }
    let r = input.resources;
    let b = input.billing;
    let billing = BillingConfiguration::new(
        b.cashier,
        FundingLimits::new(b.reserve, b.minimum_balance, b.target_balance)?,
        u64::from(b.max_gateway_entries),
        u64::from(b.max_gateway_unique),
    )?;
    let bindings = ServiceBindings {
        service: input.service,
        operator: input.operator,
        payment_account: input.payment_account,
        namespace: scalar(input.namespace, ConfigurationScalar::Namespace)?,
    };
    let limits = ServiceLimits {
        max_tenants: count(r.max_tenants, ServiceCount::Tenants)?,
        max_object_bytes: NonZeroU64::new(r.max_object_bytes).ok_or(
            ConfigurationInputError::ZeroScalar(ConfigurationScalar::ObjectBytes),
        )?,
        max_headers: count(r.max_headers, ServiceCount::Headers)?,
        max_header_bytes: count(r.max_header_bytes, ServiceCount::HeaderBytes)?,
        manifests: ServiceManifestLimits {
            max_chunks: count(r.max_chunks, ServiceCount::ManifestChunksRetained)?,
            max_tenant_chunks: count(r.max_tenant_chunks, ServiceCount::TenantManifestChunks)?,
        },
        catalog: CatalogLimits {
            max_objects: count(r.max_objects, ServiceCount::Objects)?,
            max_tenant_objects: count(r.max_tenant_objects, ServiceCount::TenantObjects)?,
            max_physical_bytes: scalar(r.max_physical_bytes, ConfigurationScalar::PhysicalBytes)?,
            max_liability_bytes: scalar(
                r.max_liability_bytes,
                ConfigurationScalar::LiabilityBytes,
            )?,
            max_tenant_logical_bytes: scalar(
                r.max_tenant_logical_bytes,
                ConfigurationScalar::TenantLogicalBytes,
            )?,
            max_references_per_object: count(
                r.max_references_per_object,
                ServiceCount::References,
            )?,
            max_receipts_per_object: count(r.max_receipts_per_object, ServiceCount::Receipts)?,
        },
        uploads: UploadLimits {
            max_active: count(r.max_active, ServiceCount::Uploads)?,
            max_tenant_active: count(r.max_tenant_active, ServiceCount::TenantUploads)?,
        },
    };
    let service = ServiceConfiguration::new(bindings, limits, billing)?;
    let attempts = usize::try_from(input.funding.max_attempts)
        .ok()
        .and_then(NonZeroUsize::new)
        .ok_or(ConfigurationInputError::FundingCapacity)?;
    scalar(input.funding.reserve, ConfigurationScalar::FundingReserve)?;
    validate_allocation(
        input.funding.allocated,
        input.funding.reserve,
        input.funding.renewal_ceiling,
    )?;
    validate_store_envelope(&service, attempts.get(), input.reads)?;
    Ok(ValidatedConfiguration {
        service,
        funding: input.funding,
        reads: input.reads,
    })
}

fn count(value: u32, field: ServiceCount) -> Result<NonZeroUsize, ConfigurationInputError> {
    let value =
        usize::try_from(value).map_err(|_| ServiceConfigurationError::CountOutOfRange { field })?;
    NonZeroUsize::new(value).ok_or(ConfigurationInputError::ZeroCount(field))
}
fn scalar(value: u128, field: ConfigurationScalar) -> Result<NonZeroU128, ConfigurationInputError> {
    NonZeroU128::new(value).ok_or(ConfigurationInputError::ZeroScalar(field))
}

/// Explicit installation data plus the compiled library release identity.
/// No defaults, deployment, allocation or provider qualification are implied.
#[derive(Clone, Copy, Debug)]
pub struct ServiceInstallationCandidate<'a> {
    /// Shared identity, resource and provider-economic configuration.
    pub configuration: ServiceConfigurationInput,
    /// Explicit provisioned project mapping; not inferred from payer/namespace.
    pub project: &'a str,
    /// Trusted whole-content verifier, independent of operator/controller roles.
    pub completion_verifier: Principal,
    /// Frozen library release, normally the service’s compiled `LIBRARY_VERSION`; never an
    /// ingress override on restore. Host artifact identity is separate.
    pub release: &'a str,
    /// Actual IC version observed in init; zero means an offline/native candidate
    /// has no qualified platform anchor. Never accept this value from ingress.
    pub platform_installation_version: u64,
}

/// Completely checked passive installation input, with no persisted owner or grant.
pub struct ValidatedInstallationInput {
    /// Immutable service/resource fields.
    configuration: ValidatedConfiguration,
    /// Explicit provider mapping, never inferred from HTTP responses.
    download_scope: CaffeineDownloadScope,
    /// Explicit verifier binding; no content evidence is supplied.
    completion: CompletionAuthority,
}
/// Invalid passive installation candidate; storage/opening failures stay in the service.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum InstallationInputError {
    /// Invalid configuration/resource envelope.
    #[error(transparent)]
    Configuration(#[from] ConfigurationInputError),
    /// Malformed explicit provider scope.
    #[error(transparent)]
    Download(#[from] DownloadScopeError),
    /// Malformed explicit verifier identity.
    #[error(transparent)]
    Completion(#[from] InvalidCompletionAuthority),
    /// Release text must be bounded, nonempty and canonical.
    #[error("invalid release identity")]
    ReleaseIdentity,
}
impl ValidatedInstallationInput {
    /// Borrow the checked configuration without allowing post-validation mutation.
    #[must_use]
    pub const fn configuration(&self) -> &ValidatedConfiguration {
        &self.configuration
    }
    /// Exact verifier binding checked against this candidate's service/namespace.
    #[must_use]
    pub const fn completion(&self) -> CompletionAuthority {
        self.completion
    }
    /// Consume the checked candidate to obtain its provider scope without cloning it.
    #[must_use]
    pub fn into_download_scope(self) -> CaffeineDownloadScope {
        self.download_scope
    }

    /// Check the complete candidate without records, memory, registration or provider effects.
    /// The caller authenticates installation and obtains actual platform identity separately.
    /// # Errors
    /// Rejects invalid limits, service/project/role bindings or bounded release identity.
    pub fn new(
        actual_service: Principal,
        candidate: ServiceInstallationCandidate<'_>,
    ) -> Result<Self, InstallationInputError> {
        let configuration = validate_candidate(actual_service, candidate.configuration)?;
        if candidate.release.is_empty()
            || candidate.release.len() > 128
            || candidate.release.trim() != candidate.release
            || candidate.release.chars().any(char::is_control)
        {
            return Err(InstallationInputError::ReleaseIdentity);
        }
        let namespace = configuration.service.bindings().namespace;
        Ok(Self {
            configuration,
            download_scope: CaffeineDownloadScope::new(
                actual_service,
                namespace,
                candidate.project,
            )?,
            completion: CompletionAuthority::new(
                actual_service,
                namespace,
                candidate.completion_verifier,
            )?,
        })
    }
}

#[cfg(test)]
mod tests;
