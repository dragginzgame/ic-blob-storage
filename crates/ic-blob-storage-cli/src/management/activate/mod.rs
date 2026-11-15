//! Explicit admission reopening after an acknowledged start; no retry or state repair.

use super::{Context, start};
use ic_platform::agent::{RequestId, RequestStatusResponse};

pub(super) async fn acknowledge(
    context: &Context,
    id: &RequestId,
    attempt: u64,
    bytes: &[u8],
    wasm: &[u8],
) -> Result<String, String> {
    super::acknowledge(id, attempt, bytes, true)?;
    let view = start::inspect(context, wasm).await?;
    if view.maintenance.attempt != attempt || !view.maintenance.active {
        return Err("original activation acknowledged historically; current admission differs, no resubmission".into());
    }
    Ok(format!(
        "Activation observed: request_id={id} attempt={attempt} active=true; current replacement artifact, configuration and admission matched."
    ))
}

pub(super) async fn observe(
    context: &Context,
    id: &RequestId,
    attempt: u64,
    original: RequestStatusResponse,
    wasm: &[u8],
) -> Result<String, String> {
    match original {
        RequestStatusResponse::Replied(reply) => {
            acknowledge(context, id, attempt, &reply.arg, wasm).await
        }
        RequestStatusResponse::Rejected(_) => {
            Err("original activation rejected; preserve custody, no resubmission".into())
        }
        _ => Err("original activation outcome unresolved; active state cannot replace its acknowledgment".into()),
    }
}
