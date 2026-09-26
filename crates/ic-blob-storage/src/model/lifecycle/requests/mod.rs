//! Bounded exact-request receipts for local reference mutations.
//!
//! This transient value owns its lifecycle so receipts and mutations cannot be
//! changed independently through its API. It is not a persisted journal, an
//! upload/funding retry protocol or proof of recovery after a restart. The
//! workflow must authenticate the actor before admission or receipt replay.

use std::{
    collections::BTreeMap,
    num::{NonZeroU128, NonZeroUsize},
};

use candid::Principal;
use thiserror::Error;

use super::{
    BlobLifecycle, LifecycleChange, LifecycleError,
    binding::{ObjectBinding, ObjectBindingMismatch, ReferenceKey},
};

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
    const fn key(self) -> ReferenceKey {
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

/// Original local transition result, distinguished from a receipt replay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferenceRequestOutcome {
    /// A new receipt was recorded, including a typed lifecycle rejection.
    Recorded {
        /// Original result; a rejected transition does not change the lifecycle.
        result: Result<LifecycleChange, LifecycleError>,
    },
    /// An existing receipt was returned without re-evaluating the operation.
    Replayed {
        /// Original result, even when the lifecycle has changed since that call.
        result: Result<LifecycleChange, LifecycleError>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReferenceReceipt {
    actor: Principal,
    operation: ReferenceOperation,
    result: Result<LifecycleChange, LifecycleError>,
}

/// A lifecycle with bounded exact-request receipts and reserved release capacity.
///
/// Every active reference reserves one remaining receipt slot for its eventual
/// release. New requests cannot consume those slots unless they also release
/// the corresponding reference. Receipts, including rejected transition results,
/// are never expired or evicted. All capacity is per object; service/tenant-wide
/// resource admission and durable atomic accounting remain separate requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceRequests {
    lifecycle: BlobLifecycle,
    receipts: BTreeMap<ReferenceRequestId, ReferenceReceipt>,
    receipt_limit: NonZeroUsize,
}

impl ReferenceRequests {
    /// Take ownership of a lifecycle and reserve one receipt per active reference.
    ///
    /// This starts an empty receipt history, not a restore or reset operation.
    /// Callers must not reconstruct an existing journal this way to discard
    /// request history or reclaim capacity.
    /// # Errors
    /// Rejects a bound too small to record each active reference's release.
    pub fn new(
        lifecycle: BlobLifecycle,
        receipt_limit: NonZeroUsize,
    ) -> Result<Self, ReferenceRequestError> {
        if lifecycle.active_references() > receipt_limit.get() {
            return Err(ReferenceRequestError::ReceiptLimitReached);
        }
        Ok(Self {
            lifecycle,
            receipts: BTreeMap::new(),
            receipt_limit,
        })
    }

    /// Apply an authenticated actor's exact request or return its stored result.
    ///
    /// Caller authentication must occur before this call, including for replay.
    /// The actor comes from trusted execution context, not request data. A new
    /// admitted request retains either success or lifecycle failure. Failed
    /// transitions can only be re-evaluated under a fresh request ID.
    ///
    /// The lifecycle is staged in a bounded clone before publication together
    /// with its receipt; work is proportional to retained reference slots.
    /// This establishes local return-path atomicity, not durable IC transactions.
    /// # Errors
    /// Wrong object scope, conflicting ID reuse or insufficient unreserved receipt
    /// capacity reject without changing the lifecycle or receipt history.
    pub fn apply(
        &mut self,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, ReferenceRequestError> {
        self.lifecycle
            .binding()
            .check(request.operation.key().object())?;
        if let Some(receipt) = self.receipts.get(&request.id) {
            if receipt.actor != actor || receipt.operation != request.operation {
                return Err(ReferenceRequestError::RequestConflict);
            }
            return Ok(ReferenceRequestOutcome::Replayed {
                result: receipt.result,
            });
        }
        let available = self.receipt_limit.get() - self.receipts.len();
        if available == 0 {
            return Err(ReferenceRequestError::ReceiptLimitReached);
        }

        let mut candidate = self.lifecycle.clone();
        let result = match request.operation {
            ReferenceOperation::Retain(key) => candidate.retain(key),
            ReferenceOperation::Release(key) => candidate.release(key),
        };
        // Subtract the new receipt before comparing the reserved release slots.
        if candidate.active_references() > available - 1 {
            return Err(ReferenceRequestError::ReceiptLimitReached);
        }
        self.receipts.insert(
            request.id,
            ReferenceReceipt {
                actor,
                operation: request.operation,
                result,
            },
        );
        self.lifecycle = candidate;
        Ok(ReferenceRequestOutcome::Recorded { result })
    }

    /// Apply already validated physical deletion evidence without erasing receipts.
    /// # Errors
    /// Returns the lifecycle's binding or live-reference rejection unchanged.
    pub fn confirm_provider_deleted(
        &mut self,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, LifecycleError> {
        self.lifecycle.confirm_provider_deleted(object)
    }

    /// Apply already validated billing settlement evidence without erasing receipts.
    /// # Errors
    /// Returns the lifecycle's binding or unconfirmed-deletion rejection unchanged.
    pub fn confirm_billing_stopped(
        &mut self,
        object: ObjectBinding,
    ) -> Result<LifecycleChange, LifecycleError> {
        self.lifecycle.confirm_billing_stopped(object)
    }

    /// Read the owned lifecycle without a path to mutate it independently.
    #[must_use]
    pub const fn lifecycle(&self) -> &BlobLifecycle {
        &self.lifecycle
    }

    /// Number of retained results, including lifecycle failures.
    #[must_use]
    pub fn receipt_count(&self) -> usize {
        self.receipts.len()
    }
}

/// Request admission failure; no new receipt or lifecycle mutation occurs.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ReferenceRequestError {
    /// Request belongs to another service, tenant, namespace, object or incarnation.
    #[error(transparent)]
    BindingMismatch(#[from] ObjectBindingMismatch),
    /// An accepted request ID was reused with different actor or operation data.
    #[error("request ID conflicts with its recorded operation")]
    RequestConflict,
    /// Recording the request would exceed capacity or consume release reservations.
    #[error("request receipt capacity is exhausted or reserved for release")]
    ReceiptLimitReached,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::lifecycle::{LifecyclePhase, ReferenceId, binding::ObjectIdentity};

    fn number(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("positive ID")
    }
    fn bound(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("positive bound")
    }
    fn actor() -> Principal {
        Principal::from_slice(&[2, 1])
    }
    fn object() -> ObjectBinding {
        ObjectBinding::new(
            Principal::from_slice(&[1, 1]),
            actor(),
            ObjectIdentity {
                namespace: number(1),
                object: number(1),
                incarnation: number(1),
            },
        )
        .expect("valid binding")
    }
    fn key(value: u128) -> ReferenceKey {
        ReferenceKey::new(object(), ReferenceId::new(number(value)))
    }
    fn request(id: u128, operation: ReferenceOperation) -> ReferenceRequest {
        ReferenceRequest {
            id: ReferenceRequestId::new(number(id)),
            operation,
        }
    }
    fn journal(limit: usize) -> ReferenceRequests {
        ReferenceRequests::new(
            BlobLifecycle::from_confirmed_upload(100, key(1), bound(3)),
            bound(limit),
        )
        .expect("release capacity")
    }

    #[test]
    fn replay_returns_original_success_without_reactivating_released_reference() {
        let mut state = journal(6);
        let retain = request(1, ReferenceOperation::Retain(key(2)));
        assert_eq!(
            state.apply(actor(), retain),
            Ok(ReferenceRequestOutcome::Recorded {
                result: Ok(LifecycleChange::Changed)
            })
        );
        state
            .apply(actor(), request(2, ReferenceOperation::Release(key(2))))
            .expect("release");
        let before = state.clone();
        assert_eq!(
            state.apply(actor(), retain),
            Ok(ReferenceRequestOutcome::Replayed {
                result: Ok(LifecycleChange::Changed)
            })
        );
        assert_eq!(state, before);
        assert_eq!(state.lifecycle().active_references(), 1);
    }

    #[test]
    fn changed_payload_or_actor_cannot_reuse_an_accepted_request_id() {
        let mut state = journal(6);
        let original = request(1, ReferenceOperation::Retain(key(2)));
        state.apply(actor(), original).expect("original request");
        let before = state.clone();
        for operation in [
            ReferenceOperation::Release(key(2)),
            ReferenceOperation::Retain(key(3)),
        ] {
            assert_eq!(
                state.apply(actor(), request(1, operation)),
                Err(ReferenceRequestError::RequestConflict)
            );
            assert_eq!(state, before);
        }
        assert_eq!(
            state.apply(Principal::from_slice(&[3, 1]), original),
            Err(ReferenceRequestError::RequestConflict)
        );
        let other = ObjectBinding::new(
            object().service(),
            object().tenant(),
            ObjectIdentity {
                incarnation: number(2),
                ..object().identity()
            },
        )
        .expect("other incarnation");
        assert_eq!(
            state.apply(
                actor(),
                request(
                    1,
                    ReferenceOperation::Retain(ReferenceKey::new(other, key(2).reference()))
                )
            ),
            Err(ReferenceRequestError::BindingMismatch(
                ObjectBindingMismatch::Incarnation
            ))
        );
        assert_eq!(state, before);
    }

    #[test]
    fn a_recorded_failure_is_not_reinterpreted_after_state_changes() {
        let mut state = journal(6);
        let release = request(1, ReferenceOperation::Release(key(2)));
        assert_eq!(
            state.apply(actor(), release),
            Ok(ReferenceRequestOutcome::Recorded {
                result: Err(LifecycleError::UnknownReference)
            })
        );
        state
            .apply(actor(), request(2, ReferenceOperation::Retain(key(2))))
            .expect("later retain");
        let before = state.clone();
        assert_eq!(
            state.apply(actor(), release),
            Ok(ReferenceRequestOutcome::Replayed {
                result: Err(LifecycleError::UnknownReference)
            })
        );
        assert_eq!(state, before);
        assert_eq!(state.lifecycle().active_references(), 2);
    }

    #[test]
    fn receipt_pressure_preserves_release_capacity_and_replays_at_full_capacity() {
        let mut state = journal(4);
        let retained = request(1, ReferenceOperation::Retain(key(2)));
        state
            .apply(actor(), retained)
            .expect("retain second reference and reserve its release");
        state
            .apply(actor(), request(2, ReferenceOperation::Retain(key(1))))
            .expect("one unreserved slot");
        let before = state.clone();
        for operation in [
            ReferenceOperation::Retain(key(3)),
            ReferenceOperation::Retain(key(1)),
            ReferenceOperation::Release(key(3)),
        ] {
            assert_eq!(
                state.apply(actor(), request(3, operation)),
                Err(ReferenceRequestError::ReceiptLimitReached)
            );
            assert_eq!(state, before);
        }
        for (id, reference) in [(3, 1), (4, 2)] {
            assert_eq!(
                state.apply(
                    actor(),
                    request(id, ReferenceOperation::Release(key(reference)))
                ),
                Ok(ReferenceRequestOutcome::Recorded {
                    result: Ok(LifecycleChange::Changed)
                })
            );
        }
        assert_eq!(state.receipt_count(), 4);
        assert_eq!(state.lifecycle().phase(), LifecyclePhase::DeletionPending);
        state.confirm_provider_deleted(object()).expect("deletion");
        state.confirm_billing_stopped(object()).expect("billing");
        let settled = state.clone();
        assert_eq!(
            state.apply(actor(), retained),
            Ok(ReferenceRequestOutcome::Replayed {
                result: Ok(LifecycleChange::Changed)
            })
        );
        assert_eq!(
            state.apply(actor(), request(5, ReferenceOperation::Release(key(1)))),
            Err(ReferenceRequestError::ReceiptLimitReached)
        );
        assert_eq!(state, settled);
    }

    #[test]
    fn construction_requires_release_slots_for_all_existing_references() {
        let mut value = BlobLifecycle::from_confirmed_upload(100, key(1), bound(3));
        value.retain(key(2)).expect("second reference");
        assert_eq!(
            ReferenceRequests::new(value.clone(), bound(1)),
            Err(ReferenceRequestError::ReceiptLimitReached)
        );
        let mut state = ReferenceRequests::new(value, bound(2)).expect("exact release reservation");
        for reference in [1, 2] {
            assert_eq!(
                state.apply(
                    actor(),
                    request(reference, ReferenceOperation::Release(key(reference)))
                ),
                Ok(ReferenceRequestOutcome::Recorded {
                    result: Ok(LifecycleChange::Changed)
                })
            );
        }
        assert_eq!(state.lifecycle().phase(), LifecyclePhase::DeletionPending);
    }
}
