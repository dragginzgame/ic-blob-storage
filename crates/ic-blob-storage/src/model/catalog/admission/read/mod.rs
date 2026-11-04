//! Bounded scanning policy for the durable upload history.
use std::num::NonZeroUsize;

/// Independent bounds for scanned operation history and returned active uploads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadPageLimits {
    /// Maximum inspected operations, including terminal ones.
    pub max_scan: NonZeroUsize,
    /// Maximum returned active operations.
    pub max_results: NonZeroUsize,
}
