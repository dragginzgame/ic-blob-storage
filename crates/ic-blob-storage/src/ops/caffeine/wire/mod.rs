//! Shared private Cashier DTOs, owned by the provider boundary.

use candid::CandidType;
use candid::Int;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub(super) struct AccountCycleBalances {
    pub total: Int,
    pub cycles_prepaid: Int,
    pub cycles_promo: Int,
    pub debt_target: DebtTarget,
    pub cycles_ledger: Int,
}

#[derive(CandidType, Deserialize)]
pub(super) enum DebtTarget {
    Prepaid,
    Ledger,
}
