//! Bounded binary-root parsing for ordered batch inputs.
//!
//! Parsing does not establish liveness, tenant ownership or gateway authority.
//! Endpoints must authenticate before using any eventual liveness workflow.

use std::num::NonZeroUsize;

use thiserror::Error;

use crate::identity::HashParseError;
use crate::identity::ProviderRootHash;

/// Explicit bounds on raw batch entries and their combined byte length.
///
/// Duplicates and malformed entries consume both budgets. These limits govern
/// already decoded input; decoder allocation/message limits remain separate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootBatchLimits {
    /// Maximum entries to inspect and results to retain.
    pub max_entries: NonZeroUsize,
    /// Maximum combined length of all supplied binary roots.
    pub max_bytes: NonZeroUsize,
}

/// Ordered parse results for an entire bounded batch, including invalid entries.
///
/// Empty batches are accepted. Duplicates remain separate entries; malformed
/// roots retain their typed error at the original position. An eventual liveness
/// handler can map invalid entries to false while evaluating
/// valid roots through separately authorized state access. This value does not
/// perform that lookup or claim any root is live or dead.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRootBatch {
    entries: Vec<Result<ProviderRootHash, HashParseError>>,
}

impl ProviderRootBatch {
    /// Parse binary roots after checking both processing bounds.
    ///
    /// The entry count is checked before scanning inputs; the byte budget is
    /// checked before allocating results. Each entry, including an invalid one,
    /// counts at its actual byte length. No partial batch is returned on a limit
    /// failure, and no input bytes are copied into retained errors.
    ///
    /// # Errors
    /// Rejects the whole batch if either configured bound is exceeded. Within
    /// those bounds, malformed roots are per-entry results, not batch failures.
    pub fn from_bytes(inputs: &[Vec<u8>], limits: RootBatchLimits) -> Result<Self, RootBatchError> {
        if inputs.len() > limits.max_entries.get() {
            return Err(RootBatchError::TooManyEntries {
                actual: inputs.len(),
                maximum: limits.max_entries.get(),
            });
        }
        let mut remaining = limits.max_bytes.get();
        for (index, bytes) in inputs.iter().enumerate() {
            remaining = remaining
                .checked_sub(bytes.len())
                .ok_or(RootBatchError::TooManyBytes {
                    index,
                    maximum: limits.max_bytes.get(),
                })?;
        }
        Ok(Self {
            entries: inputs
                .iter()
                .map(|bytes| ProviderRootHash::try_from(bytes.as_slice()))
                .collect(),
        })
    }

    /// One parse result for each original entry, in the same order.
    #[must_use]
    pub fn entries(&self) -> &[Result<ProviderRootHash, HashParseError>] {
        &self.entries
    }
}

/// A raw input batch exceeds its processing budget.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum RootBatchError {
    /// Too many entries, regardless of whether their bytes are valid or distinct.
    #[error("root batch has {actual} entries, maximum is {maximum}")]
    TooManyEntries {
        /// Raw entry count.
        actual: usize,
        /// Configured entry bound.
        maximum: usize,
    },
    /// Combined input length exceeds the byte budget.
    #[error("root batch exceeds {maximum} bytes at entry {index}")]
    TooManyBytes {
        /// First entry whose full length does not fit the remaining budget.
        index: usize,
        /// Configured byte bound, including malformed input bytes.
        maximum: usize,
    },
}

#[cfg(test)]
mod tests;
