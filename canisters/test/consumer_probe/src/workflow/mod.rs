//! Application transactions surround shared service clients without borrowing across awaits.
use crate::ops;
use blob_test_protocol::consumer::{AssetView, Failure, Fault, Run};
use candid::Principal;
use ic_blob_storage::dto::reference::ReferenceReceiptLookup;
pub(crate) async fn register(actor: Principal, input: &Run) -> Result<AssetView, Failure> {
    let id = input.registration.asset;
    let send = ops::mutate(actor, |r| {
        r.prepare(&input.registration)?;
        r.start(id, false)
    })?;
    ops::fault(input.fault, Fault::AfterIntent);
    if send {
        let command = ops::read(actor, |r| r.command(id, false))?;
        let receipt = ops::apply(command, input.max_reply_bytes).await?;
        ops::fault(input.fault, Fault::AfterRetain);
        ops::mutate(actor, |r| r.complete(id, false, receipt))?;
    }
    let view = ops::view(actor, id)?;
    if view.cancelled || view.published || !matches!(view.retain_result, Some(Ok(_))) {
        return Ok(view);
    }
    ops::descriptor(view.registration.retain).await?;
    if input.hold {
        ops::hold().await?;
    }
    // Recheck the tombstone and perform publication in one synchronous transaction.
    ops::mutate(actor, |r| r.publish(id))?;
    ops::fault(input.fault, Fault::AfterPublish);
    ops::view(actor, id)
}
pub(crate) async fn release(
    actor: Principal,
    id: u128,
    fault: Fault,
) -> Result<AssetView, Failure> {
    if ops::mutate(actor, |r| r.start(id, true))? {
        let command = ops::read(actor, |r| r.command(id, true))?;
        let receipt = ops::apply(command, 4096).await?;
        ops::mutate(actor, |r| r.complete(id, true, receipt))?;
        ops::fault(fault, Fault::AfterRelease);
    }
    ops::view(actor, id)
}
pub(crate) async fn recover(
    actor: Principal,
    id: u128,
    release: bool,
) -> Result<AssetView, Failure> {
    let command = ops::read(actor, |r| {
        r.authorize(actor, true)?;
        r.command(id, release)
    })?;
    match ops::receipt(command).await? {
        ReferenceReceiptLookup::Absent => Err(Failure::Pending),
        ReferenceReceiptLookup::Found(receipt) => {
            ops::mutate(actor, |r| r.complete(id, release, receipt))?;
            ops::view(actor, id)
        }
    }
}
