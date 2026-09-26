//! Bounded pending-deletion scans with scope-bound forward cursors.

use std::{
    num::{NonZeroU128, NonZeroUsize},
    ops::Bound::{Excluded, Unbounded},
};

use candid::Principal;
use thiserror::Error;

use crate::model::{
    identity::ProviderRootHash,
    lifecycle::{LifecyclePhase, binding::ObjectBinding},
};

use super::BlobCatalog;

/// Independent bounds for inspected objects and retained results in one page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingPageLimits {
    /// Maximum catalog entries inspected, including other namespaces/nonpending entries.
    pub max_scan: NonZeroUsize,
    /// Maximum returned pending-deletion entries.
    pub max_results: NonZeroUsize,
}

/// Forward position tied to its service and namespace, not a grant of authority.
///
/// Pages are current views, not snapshots. Rescan from the beginning on the next
/// sweep: objects behind the cursor can become pending between pages. A cursor
/// cannot be reused across reconstruction/restore as evidence of completed work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingDeletionCursor {
    service: Principal,
    namespace: NonZeroU128,
    after: ProviderRootHash,
}

/// A pending object identity; listing does not confirm deletion or billing stop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingDeletionItem {
    /// Provider root whose immutable association belongs to this object.
    pub root: ProviderRootHash,
    /// Complete object incarnation for later evidence correlation.
    pub object: ObjectBinding,
}

/// Bounded, root-ordered results, with continuation even for an empty filtered page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingDeletionPage {
    /// Only pending objects in the selected namespace, in ascending binary-root order.
    pub entries: Vec<PendingDeletionItem>,
    /// Continue scanning after the last inspected entry; None means this sweep ended.
    pub next: Option<PendingDeletionCursor>,
    /// Entries inspected, including those excluded by phase or namespace.
    pub scanned: usize,
}

/// A continuation belongs to another service or namespace.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("pending deletion cursor belongs to another scope")]
pub struct PendingCursorError;

impl BlobCatalog {
    /// Scan a bounded page of locally pending deletions, without mutation.
    ///
    /// The caller must authorize the service/namespace before exposing results.
    /// This is local enumeration, not a provider deletion request or evidence.
    /// # Errors
    /// Rejects a cursor for a different service or provider namespace.
    pub fn pending_deletions(
        &self,
        namespace: NonZeroU128,
        cursor: Option<PendingDeletionCursor>,
        limits: PendingPageLimits,
    ) -> Result<PendingDeletionPage, PendingCursorError> {
        if let Some(cursor) = cursor
            && (cursor.service != self.service || cursor.namespace != namespace)
        {
            return Err(PendingCursorError);
        }
        let start = cursor.map_or(Unbounded, |cursor| Excluded(cursor.after));
        let mut candidates = self.entries.range((start, Unbounded)).peekable();
        let mut result = PendingDeletionPage {
            entries: Vec::new(),
            next: None,
            scanned: 0,
        };
        let mut last = None;
        while result.scanned < limits.max_scan.get()
            && result.entries.len() < limits.max_results.get()
        {
            let Some((root, entry)) = candidates.next() else {
                break;
            };
            result.scanned += 1;
            last = Some(*root);
            let lifecycle = entry.requests.lifecycle();
            if lifecycle.binding().identity().namespace == namespace
                && lifecycle.phase() == LifecyclePhase::DeletionPending
            {
                result.entries.push(PendingDeletionItem {
                    root: *root,
                    object: lifecycle.binding(),
                });
            }
        }
        if candidates.peek().is_some() {
            result.next = last.map(|after| PendingDeletionCursor {
                service: self.service,
                namespace,
                after,
            });
        }
        Ok(result)
    }
}
