//! One bounded, authenticated management-status observation, shared by local callers.

use ic_platform_legacy::{
    Principal,
    agent::Agent,
    candid_codec::{
        arguments::encode_args,
        decoder::{Limits, decode_one},
    },
    management_canister_types::{CanisterIdRecord, CanisterStatusResult},
};

pub(super) const LIMITS: Limits = Limits {
    bytes: 64 * 1024,
    header: 8 * 1024,
    types: 128,
    work: 512 * 1024,
    allocation: 256 * 1024,
};

pub(super) fn argument(canister: Principal) -> Result<Vec<u8>, String> {
    encode_args(
        &(CanisterIdRecord {
            canister_id: canister,
        },),
        1024,
    )
    .map_err(|_| "cannot encode canister management target".into())
}

pub(crate) async fn read(
    agent: &Agent,
    canister: Principal,
) -> Result<CanisterStatusResult, String> {
    let target = canister
        .as_slice()
        .try_into()
        .map_err(|_| "invalid target")?;
    let management = Principal::management_canister()
        .as_slice()
        .try_into()
        .map_err(|_| "invalid management recipient")?;
    // Replicated status remains usable while the target itself is stopped.
    let bytes = agent
        .update(&management, "canister_status")
        .with_effective_canister_id(target)
        .with_arg(argument(canister)?)
        .call_and_wait()
        .await
        .map_err(|_| "platform status authentication failed")?;
    decode_one(&bytes, LIMITS).map_err(|_| "invalid platform status reply".into())
}
