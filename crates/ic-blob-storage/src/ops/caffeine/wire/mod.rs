//! Shared private Cashier DTOs, owned by the provider boundary.

use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::Int;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(super) struct AccountCycleBalances {
    pub total: Int,
    pub cycles_prepaid: Int,
    pub cycles_promo: Int,
    pub debt_target: DebtTarget,
    pub cycles_ledger: Int,
}

#[derive(CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(super) enum DebtTarget {
    Prepaid,
    Ledger,
}
