//! Private aggregate usage; only the catalog can replace one entry's contribution.
use super::{CatalogUsage, Entry, LifecyclePhase};
use candid::Principal;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct CatalogAccounting {
    global: CatalogUsage,
    // One retained row per tenant with confirmed entries, bounded by object slots.
    tenants: BTreeMap<Principal, CatalogUsage>,
}

impl CatalogAccounting {
    pub(super) fn insert(&mut self, entry: &Entry) {
        let usage = contribution(entry);
        replace(&mut self.global, CatalogUsage::default(), usage);
        replace(
            self.tenants
                .entry(entry.original.first.object().tenant())
                .or_default(),
            CatalogUsage::default(),
            usage,
        );
    }

    pub(super) fn replace(&mut self, tenant: Principal, before: CatalogUsage, after: CatalogUsage) {
        if before == after {
            return;
        }
        replace(&mut self.global, before, after);
        replace(
            self.tenants.get_mut(&tenant).expect("confirmed tenant"),
            before,
            after,
        );
    }

    pub(super) const fn usage(&self) -> CatalogUsage {
        self.global
    }

    pub(super) fn tenant_usage(&self, tenant: Principal) -> CatalogUsage {
        self.tenants.get(&tenant).copied().unwrap_or_default()
    }
}

pub(super) fn contribution(entry: &Entry) -> CatalogUsage {
    let lifecycle = entry.requests.lifecycle();
    CatalogUsage {
        objects: 1,
        live_objects: usize::from(lifecycle.phase() == LifecyclePhase::Live),
        pending_deletions: usize::from(lifecycle.phase() == LifecyclePhase::DeletionPending),
        unsettled_objects: usize::from(lifecycle.has_unsettled_obligations()),
        logical_bytes: u128::from(lifecycle.logical_bytes()),
        physical_bytes: u128::from(lifecycle.physical_bytes()),
        liability_bytes: u128::from(lifecycle.liability_bytes()),
        active_references: lifecycle.active_references() as u128,
        reference_slots: lifecycle.reference_slots() as u128,
        receipt_slots: entry.requests.receipt_count() as u128,
    }
}

fn replace(total: &mut CatalogUsage, before: CatalogUsage, after: CatalogUsage) {
    // Subtract before adding: lifetime usize/u64 products fit u128, and transient
    // double counting must not overflow a valid maximum-width aggregate.
    total.objects = count(total.objects, before.objects, after.objects);
    total.live_objects = count(total.live_objects, before.live_objects, after.live_objects);
    total.pending_deletions = count(
        total.pending_deletions,
        before.pending_deletions,
        after.pending_deletions,
    );
    total.unsettled_objects = count(
        total.unsettled_objects,
        before.unsettled_objects,
        after.unsettled_objects,
    );
    total.logical_bytes = wide(
        total.logical_bytes,
        before.logical_bytes,
        after.logical_bytes,
    );
    total.physical_bytes = wide(
        total.physical_bytes,
        before.physical_bytes,
        after.physical_bytes,
    );
    total.liability_bytes = wide(
        total.liability_bytes,
        before.liability_bytes,
        after.liability_bytes,
    );
    total.active_references = wide(
        total.active_references,
        before.active_references,
        after.active_references,
    );
    total.reference_slots = wide(
        total.reference_slots,
        before.reference_slots,
        after.reference_slots,
    );
    total.receipt_slots = wide(
        total.receipt_slots,
        before.receipt_slots,
        after.receipt_slots,
    );
}

fn count(total: usize, before: usize, after: usize) -> usize {
    total
        .checked_sub(before)
        .and_then(|remaining| remaining.checked_add(after))
        .expect("bounded catalog count")
}

fn wide(total: u128, before: u128, after: u128) -> u128 {
    total
        .checked_sub(before)
        .and_then(|remaining| remaining.checked_add(after))
        .expect("bounded catalog total")
}
