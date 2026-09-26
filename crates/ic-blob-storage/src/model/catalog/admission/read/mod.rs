//! Bounded current views of upload reservations and immutable root ownership.

use super::{UploadCatalog, UploadPhase, UploadRequest, UploadRequestId};
use crate::model::{
    identity::{HashParseError, ProviderRootHash, batch::ProviderRootBatch},
    lifecycle::{LifecyclePhase, binding::ObjectBinding},
};
use candid::Principal;
use std::{
    collections::BTreeMap,
    num::{NonZeroU128, NonZeroUsize},
    ops::Bound::{Excluded, Included},
};
use thiserror::Error;

/// Independent bounds for scanned operation history and returned active uploads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadPageLimits {
    /// Maximum inspected operations, including terminal ones.
    pub max_scan: NonZeroUsize,
    /// Maximum returned active operations.
    pub max_results: NonZeroUsize,
}

/// Forward position in one tenant's operation history, not an authorization token.
/// No snapshot, persistence, restore validity or completed reconciliation is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveUploadCursor {
    service: Principal,
    tenant: Principal,
    after: UploadRequestId,
}

/// Exact immutable request and its current active phase, not permission to retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveUploadView {
    /// Original request, including content and complete object/reference binding.
    pub request: UploadRequest,
    /// Reserved or `ExposurePossible` at the time this page was read.
    pub phase: UploadPhase,
}

/// Tenant-scoped active uploads in ascending request-ID order across namespaces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveUploadPage {
    /// Active requests only, with no cross-tenant operation data.
    pub entries: Vec<ActiveUploadView>,
    /// Resume after the last inspected ID, including on empty filtered pages.
    pub next: Option<ActiveUploadCursor>,
    /// Tenant operations inspected, including terminal history.
    pub scanned: usize,
}

/// Cursor belongs to a different service or tenant.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("upload cursor belongs to another scope")]
pub struct UploadCursorError;

/// Local root state spanning reservations and confirmed lifecycle history.
/// None of these observations grants deletion permission or proves provider state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadRootState {
    /// Unexposed reservation; must not be mistaken for an unknown/dead root.
    Reserved,
    /// Authority may have escaped; preserve the unresolved object's reservation.
    ExposurePossible,
    /// Cancelled before exposure; root history remains claimed, not reusable.
    Cancelled,
    /// Current confirmed-object phase, including retained settlement history.
    Confirmed(LifecyclePhase),
}

/// Trusted local root correlation for policy, without content or operation details.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadRootView {
    /// Original immutable object binding; caller authorization remains separate.
    pub object: ObjectBinding,
    /// Current local reservation/lifecycle observation.
    pub state: UploadRootState,
}

/// Ordered observations for a bounded root batch, without caller authorization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadRootBatchView {
    /// One result per input position, preserving duplicates and malformed entries.
    /// `Ok(None)` means no local claim, never permission to delete.
    pub entries: Vec<Result<Option<UploadRootView>, HashParseError>>,
    /// Operation-history rows inspected in the single shared scan. Does not count
    /// tree lookups; zero when every valid root is unknown or already confirmed.
    pub scanned_operations: usize,
}

impl UploadCatalog {
    /// Scan the selected tenant's active uploads without touching other tenant rows.
    ///
    /// Authorize the tenant before disclosure. Limits come from trusted configuration.
    /// Pages observe current state; they do not reserve work, expose upload authority,
    /// consume receipts or resume effects. New IDs inserted behind a cursor require a
    /// new sweep. A scan ending with no results is not evidence of provider settlement.
    /// # Errors
    /// Rejects cursor service/tenant mismatch before scanning or allocating results.
    pub fn active_uploads(
        &self,
        tenant: Principal,
        cursor: Option<ActiveUploadCursor>,
        limits: UploadPageLimits,
    ) -> Result<ActiveUploadPage, UploadCursorError> {
        if let Some(cursor) = cursor
            && (cursor.service != self.catalog.service() || cursor.tenant != tenant)
        {
            return Err(UploadCursorError);
        }
        let start = cursor.map_or(
            Included((tenant, UploadRequestId::new(NonZeroU128::MIN))),
            |cursor| Excluded((tenant, cursor.after)),
        );
        let end = Included((tenant, UploadRequestId::new(NonZeroU128::MAX)));
        let mut candidates = self.operations.range((start, end)).peekable();
        let mut page = ActiveUploadPage {
            entries: Vec::new(),
            next: None,
            scanned: 0,
        };
        let mut last = None;
        while page.scanned < limits.max_scan.get() && page.entries.len() < limits.max_results.get()
        {
            let Some(((_, id), operation)) = candidates.next() else {
                break;
            };
            page.scanned += 1;
            last = Some(*id);
            if operation.phase.active() {
                page.entries.push(ActiveUploadView {
                    request: operation.request,
                    phase: operation.phase,
                });
            }
        }
        if candidates.peek().is_some() {
            page.next = last.map(|after| ActiveUploadCursor {
                service: self.catalog.service(),
                tenant,
                after,
            });
        }
        Ok(page)
    }

    /// Resolve reservation or confirmed history without granting caller authority.
    ///
    /// Unknown roots have no local claim. Confirmed roots use logarithmic lookup;
    /// a pending/cancelled claim scans at most the configured lifetime operation
    /// bound. Callers processing batches must independently bound the raw count.
    /// This creates no second mutable root index and exposes no operation payload.
    #[must_use]
    pub fn root_view(&self, root: ProviderRootHash) -> Option<UploadRootView> {
        let object = self.catalog.claims.resolve(root).ok()?;
        if let Some(journal) = self.catalog.get(root) {
            return Some(UploadRootView {
                object,
                state: UploadRootState::Confirmed(journal.lifecycle().phase()),
            });
        }
        let operation = self
            .operations
            .values()
            .find(|operation| operation.request.object.root == root)?;
        let state = unconfirmed_root_state(operation.phase);
        Some(UploadRootView { object, state })
    }

    /// Resolve a bounded batch with at most one pass over operation history.
    ///
    /// Authorize before disclosure. Unique valid roots use existing claim/catalog
    /// lookups; only pending/cancelled claims need the shared scan, which stops
    /// once all such roots are found. Duplicates reuse the same observation.
    /// Unknown, malformed and confirmed-only batches do not scan history.
    /// Temporary maps retain at most the unique valid input roots; no index or
    /// observation cache survives this read. For B inputs, U unique roots and H
    /// history rows, work is O(B log(U+1) + U log(H+1) + H log(U+1)) at worst,
    /// with O(B) temporary/output space. Supplied batch limits must be trusted.
    #[must_use]
    pub fn root_views(&self, batch: &ProviderRootBatch) -> UploadRootBatchView {
        let mut views = BTreeMap::new();
        for root in batch.entries().iter().flatten() {
            views.insert(*root, None);
        }
        let mut pending = BTreeMap::new();
        for (root, view) in &mut views {
            let Ok(object) = self.catalog.claims.resolve(*root) else {
                continue;
            };
            if let Some(journal) = self.catalog.get(*root) {
                *view = Some(UploadRootView {
                    object,
                    state: UploadRootState::Confirmed(journal.lifecycle().phase()),
                });
            } else {
                pending.insert(*root, object);
            }
        }
        let mut scanned_operations = 0;
        if !pending.is_empty() {
            for operation in self.operations.values() {
                scanned_operations += 1;
                let root = operation.request.object.root;
                if let Some(object) = pending.remove(&root) {
                    views.insert(
                        root,
                        Some(UploadRootView {
                            object,
                            state: unconfirmed_root_state(operation.phase),
                        }),
                    );
                    if pending.is_empty() {
                        break;
                    }
                }
            }
        }
        UploadRootBatchView {
            entries: batch
                .entries()
                .iter()
                .map(|entry| match entry {
                    Ok(root) => Ok(views[root]),
                    Err(error) => Err(*error),
                })
                .collect(),
            scanned_operations,
        }
    }
}

fn unconfirmed_root_state(phase: UploadPhase) -> UploadRootState {
    match phase {
        UploadPhase::Reserved => UploadRootState::Reserved,
        UploadPhase::ExposurePossible => UploadRootState::ExposurePossible,
        UploadPhase::Cancelled => UploadRootState::Cancelled,
        // Confirmation inserts the catalog entry before changing this phase.
        UploadPhase::Confirmed => unreachable!("confirmed operation owns catalog entry"),
    }
}
