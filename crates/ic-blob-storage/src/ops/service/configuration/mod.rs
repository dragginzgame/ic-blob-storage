//! Bounded candidate decoding and conversion; no state allocation or platform effects.
#[cfg(test)]
pub(super) mod tests;

use super::stores::{ServiceStoreConfiguration, ServiceStoreError};
use crate::{
    dto::configuration::{ServiceConfigurationInput, ServiceReadInput},
    model::{
        billing::{
            FundingLimits, FundingLimitsError,
            allocation::{FundingAllocation, FundingAllocationError},
            configuration::{BillingConfiguration, BillingConfigurationError},
        },
        catalog::{CatalogLimits, admission::UploadLimits},
        service::configuration::{
            ServiceBindings, ServiceConfiguration, ServiceConfigurationError, ServiceCount,
            ServiceLimits, ServiceManifestLimits,
        },
        service::read::session::{ReadSessionError, ReadSessionLimits},
    },
};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use std::num::{NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize};
use thiserror::Error;

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
    Allocation(#[from] FundingAllocationError),
    /// A stable owner's resource/codec envelope is unsupported.
    #[error(transparent)]
    Stores(#[from] ServiceStoreError),
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
) -> Result<ServiceStoreConfiguration, ConfigurationInputError> {
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
) -> Result<ServiceStoreConfiguration, ConfigurationInputError> {
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
    let funding = FundingAllocation::new(
        input.funding.allocated,
        scalar(input.funding.reserve, ConfigurationScalar::FundingReserve)?,
        attempts,
    )?;
    let reads = read_limits(input.reads).map_err(ServiceStoreError::from)?;
    Ok(ServiceStoreConfiguration::new(service, funding, reads)?)
}

fn read_limits(input: ServiceReadInput) -> Result<ReadSessionLimits, ReadSessionError> {
    ReadSessionLimits::new(
        NonZeroU32::new(input.sessions).ok_or(ReadSessionError::Limits)?,
        NonZeroU32::new(input.tenant_sessions).ok_or(ReadSessionError::Limits)?,
        NonZeroU32::new(input.reply_bytes).ok_or(ReadSessionError::Limits)?,
        NonZeroU64::new(input.bytes).ok_or(ReadSessionError::Limits)?,
        NonZeroU64::new(input.tenant_bytes).ok_or(ReadSessionError::Limits)?,
    )
}

fn count(value: u32, field: ServiceCount) -> Result<NonZeroUsize, ConfigurationInputError> {
    let value =
        usize::try_from(value).map_err(|_| ServiceConfigurationError::CountOutOfRange { field })?;
    NonZeroUsize::new(value).ok_or(ConfigurationInputError::ZeroCount(field))
}
fn scalar(value: u128, field: ConfigurationScalar) -> Result<NonZeroU128, ConfigurationInputError> {
    NonZeroU128::new(value).ok_or(ConfigurationInputError::ZeroScalar(field))
}
