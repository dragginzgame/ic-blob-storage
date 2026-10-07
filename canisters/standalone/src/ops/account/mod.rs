//! Read-only owner access and fixed account observation resources.
use ic_blob_storage::ops::service::{
    account::{AccountInspectionAccess, AccountInspectionLimits},
    operator::OperatorStores,
};
pub(crate) struct AccountHost;
impl AccountInspectionAccess for AccountHost {
    type Memory = super::memory::Memory;
    fn with_account_stores<R>(&self, f: impl FnOnce(OperatorStores<'_, Self::Memory>) -> R) -> R {
        super::read(|stores| f(OperatorStores::from(stores)))
    }
}
pub(crate) fn limits() -> AccountInspectionLimits {
    AccountInspectionLimits {
        max_bytes: 4096.try_into().unwrap(),
        decoding_quota: 500_000.try_into().unwrap(),
        skipping_quota: 1000.try_into().unwrap(),
        max_type_entries: 64.try_into().unwrap(),
    }
}
