//! Bounded binary-root parsing for ordered batch inputs.
//!
//! Parsing does not establish liveness, tenant ownership or gateway authority.
//! Endpoints must authenticate before using any eventual liveness workflow.

use std::num::NonZeroUsize;

use thiserror::Error;

use super::{HashParseError, ProviderRootHash};

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
/// handler can map invalid entries to false, as Canic does, while evaluating
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
mod tests {
    use super::*;

    fn limits(entries: usize, bytes: usize) -> RootBatchLimits {
        RootBatchLimits {
            max_entries: NonZeroUsize::new(entries).expect("positive entry bound"),
            max_bytes: NonZeroUsize::new(bytes).expect("positive byte bound"),
        }
    }

    #[test]
    fn mixed_roots_preserve_order_duplicates_and_each_malformed_position() {
        let first = vec![1; 32];
        let second = vec![2; 32];
        let input = vec![
            second.clone(),
            vec![],
            first.clone(),
            vec![3; 31],
            second.clone(),
            vec![4; 33],
        ];
        let batch = ProviderRootBatch::from_bytes(&input, limits(6, 160))
            .expect("exact count and byte bounds");
        assert_eq!(
            batch.entries(),
            &[
                ProviderRootHash::try_from(second.as_slice()),
                Err(HashParseError::InvalidByteLength { actual: 0 }),
                ProviderRootHash::try_from(first.as_slice()),
                Err(HashParseError::InvalidByteLength { actual: 31 }),
                ProviderRootHash::try_from(second.as_slice()),
                Err(HashParseError::InvalidByteLength { actual: 33 }),
            ]
        );
        assert!(
            ProviderRootBatch::from_bytes(&[], limits(1, 1))
                .expect("empty batch")
                .entries()
                .is_empty()
        );
    }

    #[test]
    fn raw_count_rejects_duplicate_and_empty_entry_floods_before_byte_inspection() {
        for input in [vec![vec![1; 32]; 3], vec![vec![]; 3]] {
            assert_eq!(
                ProviderRootBatch::from_bytes(&input, limits(2, 1)),
                Err(RootBatchError::TooManyEntries {
                    actual: 3,
                    maximum: 2
                })
            );
        }
    }

    #[test]
    fn byte_budget_counts_malformed_roots_and_rejection_does_not_consume_input() {
        let input = vec![vec![1; 32], vec![2; 33]];
        for _ in 0..2 {
            assert_eq!(
                ProviderRootBatch::from_bytes(&input, limits(2, 64)),
                Err(RootBatchError::TooManyBytes {
                    index: 1,
                    maximum: 64
                })
            );
        }
        let parsed = ProviderRootBatch::from_bytes(&input, limits(2, 65))
            .expect("same input fits adjusted byte budget");
        assert_eq!(
            parsed.entries()[1],
            Err(HashParseError::InvalidByteLength { actual: 33 })
        );
        assert_eq!(
            ProviderRootBatch::from_bytes(&input, limits(2, 31)),
            Err(RootBatchError::TooManyBytes {
                index: 0,
                maximum: 31
            })
        );
    }
}
