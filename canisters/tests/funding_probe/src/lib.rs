//! Bounded local Cashier receiver substitute for production funding tests.
//! Never deploy as a service or Cashier.

mod model;
mod ops;
mod workflow;

use blob_test_protocol::funding::{FundingFailure, FundingReceiptRecord, FundingUpgradeArgs};
use ic_platform::principal::ExternalPrincipal;

#[ic_platform::cdk::init(decode_with = "blob_test_protocol::fixture_wire::hooks::init")]
fn init(peer: ExternalPrincipal, driver: ExternalPrincipal) {
    ops::initialize(peer.into(), driver.into());
}

#[ic_platform::cdk::post_upgrade(
    decode_with = "blob_test_protocol::fixture_wire::hooks::upgrade_one"
)]
fn post_upgrade(args: FundingUpgradeArgs) {
    // Restore before exposing endpoints; no deferred task can reset unresolved work.
    workflow::restore();
    if args.trap_after_restore {
        ic_platform::cdk::api::trap("deliberate fixture restore failure");
    }
}

#[ic_platform::cdk::update(
    decode_with = "blob_test_protocol::fixture_wire::hooks::one::<_, { blob_test_protocol::fixture_wire::identity(\"configure_cashier\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"configure_cashier\") }>"
)]
fn configure_cashier(
    input: blob_test_protocol::storage::funding::transport::Substitute,
) -> Result<(), FundingFailure> {
    workflow::configure_cashier(ic_platform::cdk::api::msg_caller(), input)
}

#[ic_platform::cdk::update(manual_reply = true, decode_with = "ops::cashier::raw_arguments")]
async fn account_top_up_v1(arguments: Vec<u8>) {
    workflow::receive_top_up(ic_platform::cdk::api::msg_caller(), arguments).await;
}

#[ic_platform::cdk::query(
    decode_with = "blob_test_protocol::fixture_wire::hooks::unit::<{ blob_test_protocol::fixture_wire::identity(\"receipts\") }>",
    encode_with = "blob_test_protocol::fixture_wire::hooks::reply::<_, { blob_test_protocol::fixture_wire::identity(\"receipts\") }>"
)]
fn receipts() -> Option<Vec<FundingReceiptRecord>> {
    ops::receipts(ic_platform::cdk::api::msg_caller())
}
