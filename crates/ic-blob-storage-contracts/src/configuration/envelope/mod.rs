//! Shared bounded resource admission, independent of journals and storage codecs.
use super::service::ServiceConfiguration;
use crate::dto::configuration::ServiceReadInput;
use thiserror::Error;
/// Maximum original-manifest record envelope in the frozen service contract.
pub const MAX_MANIFEST_RECORD_BYTES: usize = 65_536;
/// Maximum gateway members in the frozen service record envelope.
pub const MAX_GATEWAY_MEMBERS: usize = 1024;
/// Invalid immutable funding allocation; no accounting is performed here.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum AllocationLimitError {
    /// Positive reserve must fit within the initial allocation.
    #[error("funding reserve exceeds allocation")]
    Reserve,
    /// Cumulative ceiling must include the initial allocation.
    #[error("funding ceiling excludes initial allocation")]
    Ceiling,
}
/// Validate immutable allocation limits; provider credit and liquidity are separate.
/// # Errors
/// Rejects a zero/oversized reserve or a ceiling below initial allocation.
pub const fn validate_allocation(
    initial: u128,
    reserve: u128,
    ceiling: u128,
) -> Result<(), AllocationLimitError> {
    if reserve == 0 || reserve > initial {
        return Err(AllocationLimitError::Reserve);
    }
    if ceiling < initial {
        return Err(AllocationLimitError::Ceiling);
    }
    Ok(())
}
/// Unsupported fixed storage/resource envelope, separate from opening any owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ConfigurationEnvelopeError {
    /// Original manifests cannot fit the maintained record byte ceiling.
    #[error("unsupported manifest envelope")]
    Manifest,
    /// Gateway membership cannot fit the maintained record ceiling.
    #[error("unsupported gateway envelope")]
    Gateway,
    /// Lifetime funding entries must fit the portable index range.
    #[error("unsupported funding envelope")]
    Funding,
    /// Read occupancy or reply buffers exceed the configured bounds.
    #[error("unsupported read envelope")]
    Reads,
}
/// Validate the original-manifest record byte envelope without constructing a record.
/// # Errors
/// Rejects leaf/string/framing budgets exceeding the maintained codec ceiling.
pub fn validate_manifest_envelope(
    config: &ServiceConfiguration,
) -> Result<(), ConfigurationEnvelopeError> {
    let limits = config.manifest_limits();
    let worst = 1024u128
        + 33 * limits.max_chunks.get() as u128
        + limits.max_header_bytes.get() as u128
        + 8 * limits.max_headers.get() as u128;
    if worst > MAX_MANIFEST_RECORD_BYTES as u128 {
        return Err(ConfigurationEnvelopeError::Manifest);
    }
    Ok(())
}
/// Validate explicit read occupancy without reserving a session or buffer.
/// # Errors
/// Rejects zero or contradictory limits and more than 1024 concurrent sessions.
pub const fn validate_read_limits(
    input: ServiceReadInput,
) -> Result<(), ConfigurationEnvelopeError> {
    if input.sessions == 0
        || input.sessions > 1024
        || input.tenant_sessions == 0
        || input.tenant_sessions > input.sessions
        || input.reply_bytes == 0
        || input.tenant_bytes < input.reply_bytes as u64
        || input.tenant_bytes > input.bytes
    {
        return Err(ConfigurationEnvelopeError::Reads);
    }
    Ok(())
}
/// Validate the portable lifetime funding index envelope; positivity is checked by inputs.
/// # Errors
/// Rejects more entries than the maintained Wasm32 index can represent.
pub const fn validate_funding_envelope(attempts: u128) -> Result<(), ConfigurationEnvelopeError> {
    if attempts > u32::MAX as u128 {
        Err(ConfigurationEnvelopeError::Funding)
    } else {
        Ok(())
    }
}
/// Validate the gateway membership codec envelope without accessing membership state.
/// # Errors
/// Rejects distinct-member capacity beyond the maintained record ceiling.
pub fn validate_gateway_envelope(
    config: &ServiceConfiguration,
) -> Result<(), ConfigurationEnvelopeError> {
    if config.billing().gateway_limits().max_unique.get() > MAX_GATEWAY_MEMBERS {
        Err(ConfigurationEnvelopeError::Gateway)
    } else {
        Ok(())
    }
}
/// Validate every maintained store's immutable envelope before installation effects.
/// # Errors
/// Rejects unsupported manifest, gateway, funding or read limits.
pub fn validate_store_envelope(
    config: &ServiceConfiguration,
    attempts: usize,
    reads: ServiceReadInput,
) -> Result<(), ConfigurationEnvelopeError> {
    validate_manifest_envelope(config)?;
    validate_funding_envelope(attempts as u128)?;
    validate_gateway_envelope(config)?;
    validate_read_limits(reads)
}
