//! Bounded tenant-owned views of confirmed objects with unresolved obligations.

use std::{
    num::NonZeroUsize,
    ops::Bound::{Excluded, Unbounded},
};

use candid::Principal;
use thiserror::Error;

use super::BlobCatalog;
use crate::model::{
    identity::ProviderRootHash,
    lifecycle::{LifecyclePhase, binding::ObjectBinding},
};

/// Independent budgets for inspected tenant history and returned obligations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantObjectPageLimits {
    /// Maximum inspected objects belonging to this tenant, including settled ones.
    pub max_scan: NonZeroUsize,
    /// Maximum returned unsettled objects.
    pub max_results: NonZeroUsize,
}

/// Forward position in one tenant's confirmed history, not authorization or a snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TenantObjectCursor {
    service: Principal,
    tenant: Principal,
    after: ProviderRootHash,
}

/// Current local confirmed-object obligations; bytes are not currency or provider proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsettledObjectView {
    /// Provider root whose immutable claim belongs to this object.
    pub root: ProviderRootHash,
    /// Complete tenant/service/namespace/incarnation binding.
    pub object: ObjectBinding,
    /// Live, deletion pending or physically deleted but not financially settled.
    pub phase: LifecyclePhase,
    /// Bytes charged to the tenant's logical quota.
    pub logical_bytes: u64,
    /// Bytes still charged to global physical capacity.
    pub physical_bytes: u64,
    /// Bytes with outstanding billing obligations; zero does not prove settlement.
    pub liability_bytes: u64,
}

/// A bounded current view across the tenant's namespaces in binary-root order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsettledObjectPage {
    /// Unsettled confirmed objects only, including zero-byte obligations.
    pub entries: Vec<UnsettledObjectView>,
    /// Resume after the last inspected tenant root, even on an empty filtered page.
    pub next: Option<TenantObjectCursor>,
    /// Tenant history rows inspected, including settled objects; excludes other tenants.
    pub scanned: usize,
}

/// Continuation scope differs from the current service or selected tenant.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("tenant object cursor belongs to another scope")]
pub struct TenantObjectCursorError;

impl BlobCatalog {
    /// Enumerate confirmed objects that still have physical or financial obligations.
    ///
    /// Authorize the tenant before disclosure; limits come from trusted configuration.
    /// Only the tenant's retained root index is scanned, so foreign objects cannot
    /// consume the scan budget or appear in cursor/count metadata. Settled entries
    /// keep their history slots but are excluded from results. Work is bounded by
    /// the scan budget with logarithmic entry lookups, and result space by both budgets.
    ///
    /// Pages observe current state. New roots inserted behind a cursor need a fresh
    /// sweep. No cursor is restore evidence and no empty page proves an installation
    /// settled. Pending upload reservations are separate: use `UploadCatalog`'s active
    /// upload pages and aggregate usage alongside this confirmed-object view.
    /// # Errors
    /// Rejects cursor service/tenant mismatch before scanning or allocating results.
    /// # Panics
    /// Panics if the private tenant index references a missing confirmed entry,
    /// indicating an internal invariant violation, not rejected caller input.
    pub fn tenant_unsettled_objects(
        &self,
        tenant: Principal,
        cursor: Option<TenantObjectCursor>,
        limits: TenantObjectPageLimits,
    ) -> Result<UnsettledObjectPage, TenantObjectCursorError> {
        if let Some(cursor) = cursor
            && (cursor.service != self.service || cursor.tenant != tenant)
        {
            return Err(TenantObjectCursorError);
        }
        let start = cursor.map_or(Unbounded, |cursor| Excluded(cursor.after));
        let mut candidates = self
            .tenant_roots
            .get(&tenant)
            .into_iter()
            .flat_map(|roots| roots.range((start, Unbounded)))
            .peekable();
        let mut page = UnsettledObjectPage {
            entries: Vec::new(),
            next: None,
            scanned: 0,
        };
        let mut last = None;
        while page.scanned < limits.max_scan.get() && page.entries.len() < limits.max_results.get()
        {
            let Some(root) = candidates.next() else {
                break;
            };
            page.scanned += 1;
            last = Some(*root);
            let lifecycle = self
                .entries
                .get(root)
                .expect("indexed confirmed object")
                .requests
                .lifecycle();
            if lifecycle.has_unsettled_obligations() {
                page.entries.push(UnsettledObjectView {
                    root: *root,
                    object: lifecycle.binding(),
                    phase: lifecycle.phase(),
                    logical_bytes: lifecycle.logical_bytes(),
                    physical_bytes: lifecycle.physical_bytes(),
                    liability_bytes: lifecycle.liability_bytes(),
                });
            }
        }
        if candidates.peek().is_some() {
            page.next = last.map(|after| TenantObjectCursor {
                service: self.service,
                tenant,
                after,
            });
        }
        Ok(page)
    }
}
