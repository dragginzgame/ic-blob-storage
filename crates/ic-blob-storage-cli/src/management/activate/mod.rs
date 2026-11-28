//! Explicit admission reopening after an acknowledged start; no retry or state repair.

use ic_platform::native::RequestStatus;

use super::{Context, start};

pub(super) async fn acknowledge(
    context: &Context,
    id: &[u8; 32],
    attempt: u64,
    bytes: &[u8],
    wasm: &[u8],
    positional: bool,
) -> Result<String, String> {
    super::acknowledge(id, attempt, bytes, true, positional)?;
    let view = start::inspect(context, wasm).await?;
    if view.maintenance.attempt != attempt || !view.maintenance.active {
        return Err("original activation acknowledged historically; current admission differs, no resubmission".into());
    }

    Ok(format!(
        "Activation observed: request_id={display_id} attempt={attempt} active=true; current replacement artifact, configuration and admission matched.",
        display_id = crate::overview::hex(id)
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &[u8; 32],
    attempt: u64,
    original: RequestStatus,
    wasm: &[u8],
    positional: bool,
) -> Result<String, String> {
    match original {
        RequestStatus::Replied(reply) => {
            acknowledge(context, id, attempt, &reply, wasm, positional).await
        }
        RequestStatus::Rejected { .. } => {
            Err("original activation rejected; preserve custody, no resubmission".into())
        }
        _ => Err("original activation outcome unresolved; active state cannot replace its acknowledgment".into()),
    }
}
