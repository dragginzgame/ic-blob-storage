//! Bounded exact-request receipts for local reference mutations.
//!
//! This transient value owns its lifecycle so receipts and mutations cannot be
//! changed independently through its API. It is not a persisted journal, an
//! upload/funding retry protocol or proof of recovery after a restart. The
//! workflow must authenticate the actor before admission or receipt replay.

#[cfg(any(test, feature = "test-support"))]
use {
    super::BlobLifecycle, ic_blob_storage_contracts::binding::ObjectBinding,
    ic_blob_storage_contracts::reference::binding::ReferenceOperation,
    ic_blob_storage_contracts::reference::binding::ReferenceRequest,
    ic_blob_storage_contracts::reference::binding::ReferenceRequestId, ic_platform::Principal,
    std::collections::BTreeMap, std::num::NonZeroUsize,
};

use thiserror::Error;

use super::LifecycleChange;
use super::LifecycleError;
use ic_blob_storage_contracts::binding::ObjectBindingMismatch;

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

/// Read-only original result for an exactly matched local reference request.
///
/// This is retained bookkeeping, not the reference's current liveness or proof
/// of a durable transaction, provider effect or safe recovery after restore.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceReceiptView {
    /// The recorded transition result, including an admitted lifecycle rejection.
    pub result: Result<LifecycleChange, LifecycleError>,
}

/// Current local history headroom, not a reservation or durable admission promise.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReferenceCapacityView {
    /// Unused lifetime reference identities, including no refunds for released IDs.
    pub reference_slots: usize,
    /// Receipt slots not reserved for releasing currently active references.
    pub unreserved_receipts: usize,
    /// Receipt slots held for cleanup, one per active reference.
    pub release_reserved_receipts: usize,
    /// Additional distinct references that could be retained now, accounting for
    /// both the retain receipt and its eventual release. Zero after deletion queues.
    pub fresh_retains: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg(any(test, feature = "test-support"))]
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
#[cfg(any(test, feature = "test-support"))]
pub struct ReferenceRequests {
    lifecycle: BlobLifecycle,
    receipts: BTreeMap<ReferenceRequestId, ReferenceReceipt>,
    receipt_limit: NonZeroUsize,
}

#[cfg(any(test, feature = "test-support"))]
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
    /// A private single-reference transition is validated before reserving its
    /// receipt and applying it synchronously. Lookup/insertion touches tree paths;
    /// no full reference-history copy or scan is needed.
    /// This establishes local return-path atomicity, not durable IC transactions.
    /// # Errors
    /// Wrong object scope, conflicting ID reuse or insufficient unreserved receipt
    /// capacity reject without changing the lifecycle or receipt history.
    pub fn apply(
        &mut self,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<ReferenceRequestOutcome, ReferenceRequestError> {
        if let Some(receipt) = self.receipt(actor, request)? {
            return Ok(ReferenceRequestOutcome::Replayed {
                result: receipt.result,
            });
        }
        let available = self.receipt_limit.get() - self.receipts.len();
        if available == 0 {
            return Err(ReferenceRequestError::ReceiptLimitReached);
        }

        let planned = match request.operation {
            ReferenceOperation::Retain(key) => self.lifecycle.plan_retain(key),
            ReferenceOperation::Release(key) => self.lifecycle.plan_release(key),
        };
        let active_after = match &planned {
            Ok(Some(mutation)) => mutation.active_references,
            Ok(None) | Err(_) => self.lifecycle.active_references(),
        };
        // Subtract the new receipt before comparing the reserved release slots.
        admit_receipt(available, active_after)?;
        let result = match &planned {
            Ok(Some(_)) => Ok(LifecycleChange::Changed),
            Ok(None) => Ok(LifecycleChange::Unchanged),
            Err(error) => Err(*error),
        };
        self.receipts.insert(
            request.id,
            ReferenceReceipt {
                actor,
                operation: request.operation,
                result,
            },
        );
        if let Ok(mutation) = planned {
            self.lifecycle.commit_reference(mutation);
        }
        Ok(ReferenceRequestOutcome::Recorded { result })
    }

    /// Read the original result without applying a request or allocating a receipt.
    ///
    /// Authenticate the caller before lookup, just as for mutation or replay.
    /// Matching includes the complete object binding, actor and operation payload.
    /// Reads work at capacity and after settlement. `None` means no receipt in
    /// this supplied journal, not permission to retry an uncertain provider effect
    /// or proof that another/stale/restored instance never accepted the request.
    /// # Errors
    /// Rejects a wrong object binding even for an absent ID, or changed actor or
    /// operation for a recorded ID. No result is disclosed on conflict.
    pub fn receipt(
        &self,
        actor: Principal,
        request: ReferenceRequest,
    ) -> Result<Option<ReferenceReceiptView>, ReferenceRequestError> {
        self.lifecycle
            .binding()
            .check(request.operation.key().object())?;
        let Some(receipt) = self.receipts.get(&request.id) else {
            return Ok(None);
        };
        if receipt.actor != actor || receipt.operation != request.operation {
            return Err(ReferenceRequestError::RequestConflict);
        }
        Ok(Some(ReferenceReceiptView {
            result: receipt.result,
        }))
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

    /// Observe remaining capacity without consuming receipts or granting authority.
    ///
    /// Exact retries need no new slots. Fresh distinct retains each need an unused
    /// reference identity and two receipt slots: one now, one reserved for release.
    /// Other admitted requests, including lifecycle failures, can consume headroom
    /// before the next retain. Hosts must authorize disclosure and recheck on mutation.
    #[must_use]
    pub fn capacity(&self) -> ReferenceCapacityView {
        reference_headroom(
            self.lifecycle.phase(),
            self.lifecycle.remaining_reference_slots(),
            self.lifecycle.active_references(),
            self.receipt_limit.get() - self.receipts.len(),
        )
    }
}

/// Shared accounting over validated counts; active references retain cleanup slots.
pub(crate) fn reference_headroom(
    phase: super::LifecyclePhase,
    reference_slots: usize,
    release_reserved_receipts: usize,
    remaining_receipts: usize,
) -> ReferenceCapacityView {
    let unreserved_receipts = remaining_receipts - release_reserved_receipts;
    let fresh_retains = if phase == super::LifecyclePhase::Live {
        reference_slots.min(unreserved_receipts / 2)
    } else {
        0
    };
    ReferenceCapacityView {
        reference_slots,
        unreserved_receipts,
        release_reserved_receipts,
        fresh_retains,
    }
}

pub(crate) fn admit_receipt(
    available: usize,
    active_after: usize,
) -> Result<(), ReferenceRequestError> {
    if available == 0 || active_after > available - 1 {
        return Err(ReferenceRequestError::ReceiptLimitReached);
    }
    Ok(())
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
mod tests;
