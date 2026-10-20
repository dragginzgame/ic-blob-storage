//! Private maintained account-link Candid shape.
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::Int;
use ic_platform::candid::contract::Principal;
#[derive(CandidType)]
#[candid_path("ic_platform::candid::contract")]
pub(super) struct PaymentAccountCanisterAddRequest {
    pub spending_limit_per_day: Int,
    pub paid_canister: Principal,
    pub expiration_timestamp: Option<u64>,
    pub payment_account: Option<Principal>,
}
