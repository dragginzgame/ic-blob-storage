//! Bounded audit response decoding, without interpreting CSV rows as evidence.
//!
//! Schema: `payment_account_audit_log_get_v1` in `docs/evidence/caffeine-cashier.did`,
//! SHA-256 `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! The row schema, operation correlation, ordering and retention are unqualified.
//! This module performs no requests, pagination, reconciliation or persistence.

use std::num::{NonZeroU64, NonZeroUsize};

use candid::{CandidType, Principal, de::DecoderConfig, decode_one_with_config};
use serde::Deserialize;
use thiserror::Error;

/// Explicit limits for one supplied audit response, never a production default.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditLogReplyLimits {
    /// Maximum supplied bytes, checked before decoding; transport has its own bound.
    pub max_bytes: NonZeroUsize,
    /// Maximum UTF-8 bytes retained as opaque CSV after decoding.
    pub max_csv_bytes: NonZeroUsize,
    /// Maximum provider-reported count, not a verified CSV row count.
    pub max_reported_entries: NonZeroU64,
    /// Maximum Candid decoding work.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum Candid type table entries.
    pub max_type_entries: NonZeroUsize,
}

/// An opaque continuation report, not account authority or a verified checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuditLogContinuationView {
    account: Option<Principal>,
    sequence: u64,
}

impl AuditLogContinuationView {
    /// Preserve the reported optional account; absence is not filled from context.
    #[must_use]
    pub const fn account(self) -> Option<Principal> {
        self.account
    }

    /// Preserve the sequence without inferring order, progress or completeness.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

/// Bounded provider report for inspection, not an authenticated payment receipt.
///
/// The response has no independently bound account or request identity. Callers
/// must bind the original service, namespace, Cashier, account, filter and request
/// to its transport. Cursor fields alone establish none of those bindings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditLogPageView {
    csv_content: String,
    reported_entries: u64,
    has_more: bool,
    continuation: Option<AuditLogContinuationView>,
}

impl AuditLogPageView {
    /// Exact UTF-8 payload, without parsing, normalization or CSV/formula escaping.
    ///
    /// Untrusted provider text needs appropriate escaping before display or export.
    #[must_use]
    pub fn csv_content(&self) -> &str {
        &self.csv_content
    }

    /// Provider's count; no CSV columns or row count have been validated.
    #[must_use]
    pub const fn reported_entries(&self) -> u64 {
        self.reported_entries
    }

    /// Provider's pagination flag; false does not prove complete retained history.
    #[must_use]
    pub const fn has_more(&self) -> bool {
        self.has_more
    }

    /// Optional cursor exactly as reported, without initiating a follow-up request.
    #[must_use]
    pub const fn continuation(&self) -> Option<AuditLogContinuationView> {
        self.continuation
    }
}

/// Decoded audit result; neither a page nor a provider error settles a payment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditLogReply {
    /// Structurally usable bounded report, with unqualified row semantics.
    ReportedPage(AuditLogPageView),
    /// Advertised provider failure without its untrusted diagnostic text.
    ProviderFailure(AuditLogProviderError),
}

/// Advertised failure categories; none is an empty audit history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditLogProviderError {
    /// Provider reports denied authority for this principal.
    NotAuthorized(Principal),
    /// Provider reports an invalid request.
    InvalidRequest,
    /// Provider reports an internal failure.
    InternalError,
}

/// Decode the advertised audit envelope while preserving opaque CSV and cursors.
///
/// A page is diagnostic input only. No row proves credit, no missing row proves
/// nonpayment, and a terminal page does not prove a complete retention window.
/// Unknown cursor ordering/optional-account semantics are not invented here.
/// # Errors
/// Rejects oversized, malformed or over-budget replies, oversized CSV, excessive
/// reported counts and a `has_more` report without a continuation token.
pub fn decode_audit_log_reply(
    bytes: &[u8],
    limits: AuditLogReplyLimits,
) -> Result<AuditLogReply, AuditLogReplyError> {
    if bytes.len() > limits.max_bytes.get() {
        return Err(AuditLogReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let reply: wire::AuditLogDownloadResult =
        decode_one_with_config(bytes, &config).map_err(|_| AuditLogReplyError::InvalidReply)?;
    match reply {
        Err(error) => Ok(AuditLogReply::ProviderFailure(match error {
            wire::AuditLogError::NotAuthorized(principal) => {
                AuditLogProviderError::NotAuthorized(principal)
            }
            wire::AuditLogError::InvalidRequest(_) => AuditLogProviderError::InvalidRequest,
            wire::AuditLogError::InternalError(_) => AuditLogProviderError::InternalError,
        })),
        Ok(reply) => {
            if reply.csv_content.len() > limits.max_csv_bytes.get() {
                return Err(AuditLogReplyError::CsvTooLarge);
            }
            if reply.num_returned_entries > limits.max_reported_entries.get() {
                return Err(AuditLogReplyError::TooManyReportedEntries);
            }
            if reply.has_more && reply.continuation_token.is_none() {
                return Err(AuditLogReplyError::MissingContinuation);
            }
            Ok(AuditLogReply::ReportedPage(AuditLogPageView {
                csv_content: reply.csv_content,
                reported_entries: reply.num_returned_entries,
                has_more: reply.has_more,
                continuation: reply
                    .continuation_token
                    .map(|cursor| AuditLogContinuationView {
                        account: cursor.account,
                        sequence: cursor.sequence,
                    }),
            }))
        }
    }
}

/// Unusable audit response; never replace it with an empty page or settled state.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AuditLogReplyError {
    /// Supplied bytes exceed the pre-decode limit.
    #[error("audit reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or over-budget Candid.
    #[error("invalid or over-budget audit reply")]
    InvalidReply,
    /// Decoded CSV exceeds its separate retained-byte budget.
    #[error("audit CSV exceeds byte limit")]
    CsvTooLarge,
    /// Provider-reported entry count exceeds the caller's bound.
    #[error("audit report exceeds entry limit")]
    TooManyReportedEntries,
    /// A page advertises more results but supplies no next token.
    #[error("audit continuation is missing")]
    MissingContinuation,
}

// Single private owner of the advertised response schema; no request API yet.
mod wire {
    use super::{CandidType, Deserialize, Principal};

    pub(super) type AuditLogDownloadResult = Result<AuditLogDownloadResponse, AuditLogError>;

    #[derive(CandidType, Deserialize)]
    pub(super) struct AuditLogDownloadResponse {
        pub continuation_token: Option<AuditLogContinuationToken>,
        pub csv_content: String,
        pub num_returned_entries: u64,
        pub has_more: bool,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) struct AuditLogContinuationToken {
        pub account: Option<Principal>,
        pub sequence: u64,
    }

    #[derive(CandidType, Deserialize)]
    pub(super) enum AuditLogError {
        NotAuthorized(Principal),
        InvalidRequest(String),
        InternalError(String),
    }
}

#[cfg(test)]
mod tests;
