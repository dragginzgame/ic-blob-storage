//! Private maintained account-link Candid shape.
use candid::CandidType;
use candid::Int;
use candid::Principal;
#[derive(CandidType)]
pub(super) struct PaymentAccountCanisterAddRequest {
    pub spending_limit_per_day: Int,
    pub paid_canister: Principal,
    pub expiration_timestamp: Option<u64>,
    pub payment_account: Option<Principal>,
}
