//! Bounded authenticated management status using Platform's selected schema.
use crate::network::Client;
use ic_platform_protocol::{
    candid::Limits,
    management_snapshots::{CanisterArgs, CanisterStatus, decode_canister_status},
};
use ic_platform_types::Principal;
pub(crate) const LIMITS: Limits = Limits {
    bytes: 64 * 1024,
    header: 8 * 1024,
    types: 128,
    work: 512 * 1024,
    allocation: 256 * 1024,
    vector: 64 * 1024,
    skipping: 512 * 1024,
};
pub(super) fn argument(canister: Principal) -> Result<Vec<u8>, String> {
    CanisterArgs {
        canister_id: canister,
    }
    .encode(1024)
    .map_err(|_| "cannot encode canister management target".into())
}
pub(crate) async fn read(client: &Client, canister: Principal) -> Result<CanisterStatus, String> {
    let bytes = client
        .update(
            Principal::management_canister(),
            canister,
            "canister_status",
            &argument(canister)?,
        )
        .await?;
    decode_canister_status(&bytes, LIMITS).map_err(|_| "invalid platform status reply".into())
}
