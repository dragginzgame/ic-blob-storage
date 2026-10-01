//! Shared private Cashier DTOs, owned by the provider boundary.

use candid::{CandidType, Int, Principal};
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

#[derive(CandidType)]
pub(super) struct PaymentAccountCanisterAddRequest {
    pub spending_limit_per_day: Int,
    pub paid_canister: Principal,
    pub expiration_timestamp: Option<u64>,
    pub payment_account: Option<Principal>,
}
