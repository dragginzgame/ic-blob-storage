//! Service metadata invariants, stricter than the provider's generic hash format.
use crate::model::identity::caffeine::CaffeineHeader;
use std::collections::BTreeSet;
use thiserror::Error;

/// Rejected declaration. This validates metadata, never actual provider bytes.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UploadMetadataError {
    /// Raw entry count exceeds the configured budget.
    #[error("too many service metadata headers")]
    HeaderCount,
    /// Raw UTF-8 bytes plus framing exceed the configured budget.
    #[error("service metadata exceeds byte budget")]
    HeaderBytes,
    /// Names must be nonempty ASCII HTTP tokens, without trimming.
    #[error("invalid service metadata name")]
    HeaderName,
    /// Controls, line separators or surrounding whitespace are not accepted.
    #[error("invalid service metadata value")]
    HeaderValue,
    /// Names must be unique ignoring ASCII case.
    #[error("duplicate service metadata name")]
    DuplicateHeader,
    /// Exactly one canonical Content-Length entry is required.
    #[error("service metadata requires Content-Length")]
    LengthRequired,
    /// Length must use canonical name casing and unsigned decimal without padding.
    #[error("noncanonical service Content-Length")]
    NonCanonicalLength,
    /// Metadata length must equal the immutable admitted declaration.
    #[error("service Content-Length disagrees with reservation")]
    LengthMismatch,
}

pub(super) fn validate(
    headers: &[CaffeineHeader<'_>],
    bytes: u64,
    max_headers: usize,
    max_bytes: usize,
) -> Result<(), UploadMetadataError> {
    if headers.len() > max_headers {
        return Err(UploadMetadataError::HeaderCount);
    }
    let mut remaining = max_bytes;
    for header in headers {
        for length in [header.name.len(), header.value.len(), 3] {
            remaining = remaining
                .checked_sub(length)
                .ok_or(UploadMetadataError::HeaderBytes)?;
        }
    }
    // All scans and allocations below are bounded by the raw entry/byte budgets.
    let mut names = BTreeSet::new();
    let mut length = None;
    for header in headers {
        if header.name.is_empty() || !header.name.bytes().all(token) {
            return Err(UploadMetadataError::HeaderName);
        }
        if !names.insert(header.name.to_ascii_lowercase()) {
            return Err(UploadMetadataError::DuplicateHeader);
        }
        if header
            .value
            .chars()
            .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'))
            || header
                .value
                .trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}')
                != header.value
        {
            return Err(UploadMetadataError::HeaderValue);
        }
        if header.name.eq_ignore_ascii_case("Content-Length") {
            if header.name != "Content-Length"
                || header.value.is_empty()
                || !header.value.bytes().all(|b| b.is_ascii_digit())
                || (header.value.len() > 1 && header.value.starts_with('0'))
            {
                return Err(UploadMetadataError::NonCanonicalLength);
            }
            length = Some(
                header
                    .value
                    .parse::<u64>()
                    .map_err(|_| UploadMetadataError::NonCanonicalLength)?,
            );
        }
    }
    match length {
        None => Err(UploadMetadataError::LengthRequired),
        Some(length) if length != bytes => Err(UploadMetadataError::LengthMismatch),
        Some(_) => Ok(()),
    }
}

fn token(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
}
