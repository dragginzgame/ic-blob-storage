//! Exact reference-operation identity, without receipts or mutation.
use crate::binding::ReferenceKey;
use std::num::NonZeroU128;
/// Request identity within one bound object incarnation.
///
/// Freshness and non-reuse across restart/restore require an external allocator.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReferenceRequestId(NonZeroU128);

impl ReferenceRequestId {
    /// Wrap a caller-supplied nonzero request ID without authorizing its use.
    #[must_use]
    pub const fn new(value: NonZeroU128) -> Self {
        Self(value)
    }

    /// Exact retained identity for encoding; this allocates no fresh operation.
    #[must_use]
    pub const fn get(self) -> NonZeroU128 {
        self.0
    }
}

/// Exact operation and reference arguments bound to a request ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceOperation {
    /// Retain one reference to the bound object.
    Retain(ReferenceKey),
    /// Release one reference to the bound object.
    Release(ReferenceKey),
}

impl ReferenceOperation {
    /// Full object/reference identity; this grants no mutation authority.
    #[must_use]
    pub const fn key(self) -> ReferenceKey {
        match self {
            Self::Retain(key) | Self::Release(key) => key,
        }
    }
}

/// Passive local request data; no wire schema or default request is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceRequest {
    /// ID reused only for an exact retry of this operation.
    pub id: ReferenceRequestId,
    /// Full operation payload; compared directly rather than via a hash.
    pub operation: ReferenceOperation,
}
