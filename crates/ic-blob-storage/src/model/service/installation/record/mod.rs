//! Host-owned current configuration schema; boundary DTOs are not persisted.
use candid::{CandidType, Deserialize, Principal};

/// One bounded immutable installation record, never a freshness authority.
#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) struct ConfigurationRecord {
    pub(crate) format: String,
    pub(crate) release: String,
    pub(crate) platform_installation_version: u64,
    pub(crate) service: Principal,
    pub(crate) operator: Principal,
    pub(crate) payment_account: Principal,
    pub(crate) namespace: u128,
    pub(crate) project: String,
    pub(crate) completion_verifier: Principal,
    pub(crate) max_tenants: u32,
    pub(crate) max_object_bytes: u64,
    pub(crate) max_headers: u32,
    pub(crate) max_header_bytes: u32,
    pub(crate) max_chunks: u32,
    pub(crate) max_tenant_chunks: u32,
    pub(crate) max_objects: u32,
    pub(crate) max_tenant_objects: u32,
    pub(crate) max_physical_bytes: u128,
    pub(crate) max_liability_bytes: u128,
    pub(crate) max_tenant_logical_bytes: u128,
    pub(crate) max_references_per_object: u32,
    pub(crate) max_receipts_per_object: u32,
    pub(crate) max_active: u32,
    pub(crate) max_tenant_active: u32,
    pub(crate) billing_cashier: Principal,
    pub(crate) billing_reserve: u128,
    pub(crate) billing_minimum_balance: u128,
    pub(crate) billing_target_balance: u128,
    pub(crate) billing_max_gateway_entries: u32,
    pub(crate) billing_max_gateway_unique: u32,
    pub(crate) funding_allocated: u128,
    pub(crate) funding_renewal_ceiling: u128,
    pub(crate) funding_reserve: u128,
    pub(crate) funding_max_attempts: u32,
    pub(crate) read_sessions: u32,
    pub(crate) read_tenant_sessions: u32,
    pub(crate) read_reply_bytes: u32,
    pub(crate) read_bytes: u64,
    pub(crate) read_tenant_bytes: u64,
}
impl ConfigurationRecord {
    /// Frozen current layout, independently checked from the installation release.
    /// The host allocation key names a memory slot, not this record's layout.
    pub(crate) const FORMAT: &str = "ic-blob-storage/installation:project-upload-grants-platform-anchor-funding-credit-index-renewal";

    pub(crate) fn check_binding(
        &self,
        service: Principal,
        release: &str,
    ) -> Result<(), super::InstallationBindingError> {
        if self.format != Self::FORMAT {
            return Err(super::InstallationBindingError::Format);
        }
        if self.service != service {
            return Err(super::InstallationBindingError::Service);
        }
        if self.release != release {
            return Err(super::InstallationBindingError::Release);
        }
        Ok(())
    }
}
crate::model::record_codec::storable!(
    ConfigurationRecord, bytes: 16_384, work: 100_000, skipping: 1024,
    types: 64, header: 16_384,
    layout: ic_stable_structures::storable::Bound::Bounded {
        max_size: 16_384, is_fixed_size: false,
    },
);
