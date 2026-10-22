//! External Caffeine GC callback bounds. These are provider Candid contracts,
//! not Blob-owned application frames or proof of deletion/billing settlement.
use crate::identity::batch::RootBatchLimits;
use std::num::NonZeroUsize;

/// Provider mixin's maximum deletion-list length; also bounds callback inputs.
pub const MAX_ROOTS: usize = 10_000;
/// Finite Candid message envelope for 10,000 binary SHA-256 roots and framing.
pub const MAX_REQUEST_BYTES: usize = 400_000;
/// Fixed raw-key count and byte budgets, including duplicate/malformed entries.
pub const ROOT_LIMITS: RootBatchLimits = RootBatchLimits {
    max_entries: NonZeroUsize::new(MAX_ROOTS).expect("positive root bound"),
    max_bytes: NonZeroUsize::new(MAX_ROOTS * 32).expect("positive byte bound"),
};

/// Invalid external callback input; no root lookup or mutation has occurred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum GcInputError {
    /// Raw Candid message exceeds its envelope.
    #[error("GC ingress exceeds byte bound")]
    TooLarge,
    /// Wrong argument shape, malformed Candid or decoder budget exhaustion.
    #[error("invalid GC arguments")]
    Invalid,
    /// Decoded roots exceed the independent processing budget.
    #[error(transparent)]
    Batch(#[from] crate::identity::batch::RootBatchError),
    /// A root is not exactly 32 bytes.
    #[error("malformed GC root")]
    MalformedRoot,
}
/// Decode the provider's single bounded binary-root vector.
/// # Errors
/// Rejects oversized, malformed, wrong-shape or over-budget Candid and root batches.
pub fn decode_roots(bytes: &[u8]) -> Result<Vec<Vec<u8>>, GcInputError> {
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err(GcInputError::TooLarge);
    }
    let config = crate::decoding::limits(5_000_000, 0, 16, Some(1024));
    let roots: Vec<Vec<u8>> = ic_platform::candid::decoder::decode_one_with_config(bytes, &config)
        .map_err(|_| GcInputError::Invalid)?;
    let batch = crate::identity::batch::ProviderRootBatch::from_bytes(&roots, ROOT_LIMITS)?;
    if batch.entries().iter().any(Result::is_err) {
        return Err(GcInputError::MalformedRoot);
    }
    Ok(roots)
}
/// Decode the no-argument deletion-list request with a separate small envelope.
/// # Errors
/// Rejects nonempty arguments, malformed Candid or exhausted processing bounds.
pub fn decode_empty(bytes: &[u8]) -> Result<(), GcInputError> {
    if bytes.len() > 1024 {
        return Err(GcInputError::TooLarge);
    }
    let config = crate::decoding::limits(10_000, 0, 16, Some(1024));
    ic_platform::candid::decoder::decode_args_with_config::<()>(bytes, &config)
        .map_err(|_| GcInputError::Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_platform::candid::contract::{encode_args, encode_one};
    #[test]
    fn callback_input_bounds_preserve_binary_roots_order_and_duplicates() {
        let roots = vec![vec![1u8; 32], vec![2; 32], vec![1; 32]];
        assert_eq!(decode_roots(&encode_one(&roots).unwrap()), Ok(roots));
        assert_eq!(
            decode_roots(&encode_one(Vec::<Vec<u8>>::new()).unwrap()),
            Ok(vec![])
        );
        assert_eq!(decode_empty(&encode_args(()).unwrap()), Ok(()));
        for malformed in [vec![], vec![1u8; 31], vec![1; 33]] {
            assert_eq!(
                decode_roots(&encode_one(vec![malformed]).unwrap()),
                Err(GcInputError::MalformedRoot)
            );
        }
        let maximum = vec![vec![1u8; 32]; MAX_ROOTS];
        assert_eq!(decode_roots(&encode_one(&maximum).unwrap()), Ok(maximum));
        assert!(matches!(
            decode_roots(&encode_one(vec![vec![1u8; 32]; MAX_ROOTS + 1]).unwrap()),
            Err(GcInputError::Batch(_))
        ));
        assert_eq!(
            decode_roots(&vec![0; MAX_REQUEST_BYTES + 1]),
            Err(GcInputError::TooLarge)
        );
        assert_eq!(decode_roots(b"bad"), Err(GcInputError::Invalid));
        assert!(decode_empty(&encode_one(123u64).unwrap()).is_err());
        assert!(decode_roots(&encode_args((vec![vec![1u8; 32]], 5u64)).unwrap()).is_err());
    }
}
