//! The enrollment record shared by persistence and inspection-budget tests.

#[cfg(target_arch = "wasm32")]
pub(super) mod stable;
#[cfg(test)]
mod tests;

use crate::protocol::View;
use ic_platform_protocol::caffeine::{Nat, TopUpArgs, encode_top_up};
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Record {
    version: u32,
    view: View,
}

fn arguments(owner: Principal, target: u64) -> ic_wire::Bytes<1024> {
    ic_wire::Bytes::new(
        encode_top_up(
            Some(&TopUpArgs {
                target_balance: Some(Nat::from(target)),
                account: Some(owner),
            }),
            1024,
        )
        .expect("fixed Cashier request"),
    )
    .unwrap()
}
