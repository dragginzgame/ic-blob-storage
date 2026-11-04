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
    let batch =
        ProviderRootBatch::from_bytes(&input, limits(6, 160)).expect("exact count and byte bounds");
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
    assert_eq!(
        ProviderRootBatch::from_bytes(&[], limits(1, 1))
            .expect("empty batch")
            .entries(),
        []
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
