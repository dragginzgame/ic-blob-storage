//! Private reservation totals, changed only with successful owner transitions.
use super::{CatalogUsage, UploadUsage};
use candid::Principal;
use std::collections::BTreeMap;

#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct ReservationAccounting {
    global: Totals,
    // One row per tenant with lifetime operations; never removed on cancellation.
    tenants: BTreeMap<Principal, Totals>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Totals {
    operations: usize,
    active: usize,
    bytes: u128,
}

impl ReservationAccounting {
    pub(super) fn admit(&mut self, tenant: Principal, bytes: u64) {
        for total in [&mut self.global, self.tenants.entry(tenant).or_default()] {
            total.operations += 1;
            total.active += 1;
            total.bytes += u128::from(bytes);
        }
    }

    pub(super) fn release(&mut self, tenant: Principal, bytes: u64) {
        for total in [
            &mut self.global,
            self.tenants.get_mut(&tenant).expect("admitted tenant"),
        ] {
            total.active = total.active.checked_sub(1).expect("active reservation");
            total.bytes = total
                .bytes
                .checked_sub(u128::from(bytes))
                .expect("reserved bytes");
        }
    }

    pub(super) fn usage(&self, confirmed: CatalogUsage) -> UploadUsage {
        self.global.with_confirmed(confirmed)
    }

    pub(super) fn tenant_usage(&self, tenant: Principal, confirmed: CatalogUsage) -> UploadUsage {
        self.tenants
            .get(&tenant)
            .copied()
            .unwrap_or_default()
            .with_confirmed(confirmed)
    }
}

impl Totals {
    fn with_confirmed(self, confirmed: CatalogUsage) -> UploadUsage {
        // A lifetime operation is counted once, independently of its confirmed
        // entry. Confirmation removes the reservation after inserting that entry.
        UploadUsage {
            operations: self.operations,
            active_reservations: self.active,
            reserved_bytes: self.bytes,
            logical_bytes: confirmed.logical_bytes + self.bytes,
            physical_bytes: confirmed.physical_bytes + self.bytes,
            liability_bytes: confirmed.liability_bytes + self.bytes,
        }
    }
}
