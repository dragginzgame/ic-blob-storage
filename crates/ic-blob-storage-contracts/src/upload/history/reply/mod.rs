//! Bounded current history observations; no effect, completed sweep or retry authority.
use crate::dto::upload::history::UploadContentState;
use crate::dto::upload::history::UploadHistoryCursor;
use crate::dto::upload::history::UploadHistoryFailure;
use crate::dto::upload::history::UploadHistoryFilter;
use crate::dto::upload::history::UploadHistoryPage;
use crate::dto::upload::history::UploadHistoryRequest;
use crate::dto::upload::history::UploadHistoryScope;
use crate::upload::history::LifecyclePhase;
use crate::upload::history::UploadRootState;
use candid::Principal;
use candid::de::DecoderConfig;
use candid::decode_one_with_config;
use std::collections::BTreeSet;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
use thiserror::Error;

/// Invalid or refused observation, never an empty page or authority to retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum UploadHistoryReplyError {
    /// Encoded reply, entry count or scanned work exceeds its independent bound.
    #[error("upload history reply exceeds limit")]
    Limit,
    /// Malformed identity, encoding, ordering, filter result or continuation.
    #[error("invalid upload history request or reply")]
    Invalid,
    /// Reply, entry or cursor belongs to a different request or scope.
    #[error("upload history binding mismatch")]
    Binding,
    /// Authenticated service refusal, distinct from an empty page.
    #[error("upload history refused: {0:?}")]
    Remote(UploadHistoryFailure),
}
/// Caller-owned acceptance bounds; these do not set the host's scan/page limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadHistoryReplyLimits {
    /// Applied before decoding; transport must independently bound buffering.
    pub bytes: NonZeroUsize,
    /// Maximum matching entries in this observation.
    pub entries: NonZeroUsize,
    /// Maximum inspected rows, including filtered-out history.
    pub scanned: NonZeroU64,
}

fn principal(value: Principal) -> Result<(), UploadHistoryReplyError> {
    if [Principal::anonymous(), Principal::management_canister()].contains(&value) {
        return Err(UploadHistoryReplyError::Invalid);
    }
    Ok(())
}
fn cursor(
    input: UploadHistoryRequest,
    c: UploadHistoryCursor,
) -> Result<(), UploadHistoryReplyError> {
    if c.service != input.service
        || c.namespace != input.namespace
        || c.scope != input.scope
        || c.filter != input.filter
    {
        return Err(UploadHistoryReplyError::Binding);
    }
    principal(c.after_tenant)?;
    if c.after_request == 0 {
        return Err(UploadHistoryReplyError::Invalid);
    }
    if let UploadHistoryScope::Tenant(tenant) = input.scope
        && tenant != c.after_tenant
    {
        return Err(UploadHistoryReplyError::Binding);
    }
    Ok(())
}
fn check(input: UploadHistoryRequest) -> Result<(), UploadHistoryReplyError> {
    principal(input.service)?;
    if input.namespace == 0 {
        return Err(UploadHistoryReplyError::Invalid);
    }
    if let UploadHistoryScope::Tenant(tenant) = input.scope {
        principal(tenant)?;
    }
    if let Some(c) = input.cursor {
        cursor(input, c)?;
    }
    Ok(())
}
/// Encode one structurally validated scan without dispatching or granting authority.
/// # Errors
/// Rejects malformed identities and a foreign or malformed saved cursor.
pub fn request(input: UploadHistoryRequest) -> Result<Vec<u8>, UploadHistoryReplyError> {
    check(input)?;
    candid::encode_one(input).map_err(|_| UploadHistoryReplyError::Invalid)
}
fn position(c: UploadHistoryCursor) -> (Principal, u128) {
    (c.after_tenant, c.after_request)
}
fn state(input: UploadContentState) -> UploadRootState {
    match input {
        UploadContentState::Reserved => UploadRootState::Reserved,
        UploadContentState::ExposurePossible => UploadRootState::ExposurePossible,
        UploadContentState::Cancelled => UploadRootState::Cancelled,
        UploadContentState::Live => UploadRootState::Confirmed(LifecyclePhase::Live),
        UploadContentState::DeletionPending => {
            UploadRootState::Confirmed(LifecyclePhase::DeletionPending)
        }
        UploadContentState::ProviderDeleted => {
            UploadRootState::Confirmed(LifecyclePhase::ProviderDeleted)
        }
        UploadContentState::Settled => UploadRootState::Confirmed(LifecyclePhase::Settled),
    }
}

/// Decode one independently authenticated current history page under the exact request.
/// Validates complete entry identities, tenant/request ordering, filtering, scanned
/// work and forward progress. Empty filtered pages may legitimately continue past
/// rows not returned. Fences are retained; no snapshot or completed sweep is implied.
/// # Errors
/// Rejects over-budget, malformed, foreign, inconsistent or refused replies.
pub fn page(
    input: UploadHistoryRequest,
    bytes: &[u8],
    limits: UploadHistoryReplyLimits,
) -> Result<UploadHistoryPage, UploadHistoryReplyError> {
    check(input)?;
    if bytes.len() > limits.bytes.get() {
        return Err(UploadHistoryReplyError::Limit);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(limits.bytes.get())
        .set_full_error_message(false);
    let page: Result<UploadHistoryPage, UploadHistoryFailure> =
        decode_one_with_config(bytes, &config).map_err(|_| UploadHistoryReplyError::Invalid)?;
    let page = page.map_err(UploadHistoryReplyError::Remote)?;
    if page.request != input {
        return Err(UploadHistoryReplyError::Binding);
    }
    if page.entries.len() > limits.entries.get() || page.scanned > limits.scanned.get() {
        return Err(UploadHistoryReplyError::Limit);
    }
    let count = u64::try_from(page.entries.len()).map_err(|_| UploadHistoryReplyError::Limit)?;
    if page.scanned < count || (input.filter == UploadHistoryFilter::All && page.scanned != count) {
        return Err(UploadHistoryReplyError::Invalid);
    }
    let mut previous = input.cursor.map(position);
    let mut roots = BTreeSet::new();
    let mut objects = BTreeSet::new();
    for entry in &page.entries {
        let u = entry.request;
        if u.service != input.service
            || u.namespace != input.namespace
            || matches!(input.scope, UploadHistoryScope::Tenant(tenant) if tenant != u.tenant)
        {
            return Err(UploadHistoryReplyError::Binding);
        }
        let request = crate::reference::parse_upload_binding(input.service, u)
            .map_err(|_| UploadHistoryReplyError::Invalid)?;
        if !roots.insert(request.object.root)
            || !objects.insert((u.tenant, u.object, u.incarnation))
        {
            return Err(UploadHistoryReplyError::Invalid);
        }
        let current = (u.tenant, u.upload);
        if previous.is_some_and(|previous| current <= previous)
            || !super::filter(input.filter).includes(state(entry.state))
        {
            return Err(UploadHistoryReplyError::Invalid);
        }
        previous = Some(current);
    }
    if let Some(next) = page.next {
        cursor(input, next)?;
        let next = position(next);
        if page.scanned == 0
            || input.cursor.is_some_and(|c| next <= position(c))
            || previous.is_some_and(|last| next < last)
            || (input.filter == UploadHistoryFilter::All && previous != Some(next))
        {
            return Err(UploadHistoryReplyError::Invalid);
        }
    }
    Ok(page)
}

#[cfg(test)]
mod tests;
