//! Passive capacity and operator reconciliation conversion under one owner borrow.
use super::{STATE, conversion::failure, read};
use blob_test_protocol::storage::{
    Failure,
    read::{RootBatchInput, RootObservation},
};
use ic_blob_storage::{
    model::{
        identity::{
            HashParseError,
            batch::{ProviderRootBatch, RootBatchLimits},
        },
        service::upload::UploadContext,
    },
    ops::service::uploads::read::UploadRootObservation,
};
use std::num::{NonZeroU128, NonZeroUsize};

pub(crate) fn roots(
    execution: UploadContext,
    input: &RootBatchInput,
) -> Result<Vec<RootObservation>, Failure> {
    if execution.service != input.service {
        return Err(Failure::Binding);
    }
    let namespace = NonZeroU128::new(input.namespace).ok_or(Failure::Invalid)?;
    // Fixed fixture processing bounds supplement the endpoint's bounded decoder.
    let batch = ProviderRootBatch::from_bytes(
        &input.roots,
        RootBatchLimits {
            max_entries: NonZeroUsize::new(8).unwrap(),
            max_bytes: NonZeroUsize::new(256).unwrap(),
        },
    )
    .map_err(|_| Failure::Capacity)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .observe_roots(execution, namespace, &batch)
        })
        .map(|entries| {
            entries
                .into_iter()
                .map(|entry| match entry {
                    UploadRootObservation::Known(view) => {
                        RootObservation::Known(read::observation(view))
                    }
                    UploadRootObservation::Unknown => RootObservation::Unknown,
                    UploadRootObservation::Malformed(HashParseError::InvalidByteLength {
                        actual,
                    }) => RootObservation::Malformed {
                        bytes: actual as u64,
                    },
                    // This binary parser produces only length errors.
                    UploadRootObservation::Malformed(_) => unreachable!("binary root parser"),
                })
                .collect()
        })
        .map_err(failure)
}
