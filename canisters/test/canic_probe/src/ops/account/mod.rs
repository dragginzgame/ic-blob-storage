//! Managed owner borrowing and bounded passive account reports.
use ic_blob_storage::ops::service::{
    account::{AccountInspectionAccess, AccountInspectionLimits},
    operator::OperatorStores,
};

pub(crate) struct AccountHost;
impl AccountInspectionAccess for AccountHost {
    type Memory = ic_blob_storage_canic::memory::ManagedMemory;
    fn with_account_stores<R>(
        &self,
        operation: impl FnOnce(OperatorStores<'_, Self::Memory>) -> R,
    ) -> R {
        super::read(|owner| operation(owner.stores().into()))
    }
}
pub(crate) fn limits() -> AccountInspectionLimits {
    AccountInspectionLimits {
        max_bytes: 4096.try_into().expect("fixed reply bound"),
        decoding_quota: 500_000.try_into().expect("fixed work bound"),
        skipping_quota: 1000.try_into().expect("fixed skipping bound"),
        max_type_entries: 64.try_into().expect("fixed type bound"),
    }
}
