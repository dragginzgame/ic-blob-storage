//! Durable uploader intent precedes dispatch; uncertainty permits only inspection.
use crate::ops;
use blob_test_protocol::consumer::{
    Failure,
    manifests::{ManifestDispatch, ManifestFault, ManifestIntent, ManifestIntentView},
};
use candid::Principal;
pub(crate) fn save(
    actor: Principal,
    input: &ManifestIntent,
) -> Result<ManifestIntentView, Failure> {
    ops::mutate(actor, |r| {
        r.save_manifest(input)?;
        r.manifest_view(input.id)
    })
}
pub(crate) async fn dispatch(
    actor: Principal,
    input: ManifestDispatch,
) -> Result<ManifestIntentView, Failure> {
    if input.max_reply_bytes == 0 {
        return Err(Failure::Invalid);
    }
    let send = ops::mutate(actor, |r| r.start_manifest(input.id))?;
    ops::manifests::fault(input.fault, ManifestFault::BeforeDispatch);
    if send {
        let request = ops::read(actor, |r| Ok(r.manifest_view(input.id)?.intent.request))?;
        let response = ops::manifests::prepare(&request, input.max_reply_bytes).await?;
        ops::manifests::fault(input.fault, ManifestFault::AfterPreparation);
        if input.hold {
            ops::hold().await?;
        }
        ops::mutate(actor, |r| r.acknowledge_manifest(input.id, response))?;
    }
    view(actor, input.id)
}
pub(crate) fn cancel(actor: Principal, id: u128) -> Result<ManifestIntentView, Failure> {
    ops::mutate(actor, |r| {
        r.cancel_manifest(id)?;
        r.manifest_view(id)
    })
}
pub(crate) async fn recover(actor: Principal, id: u128) -> Result<ManifestIntentView, Failure> {
    let saved = ops::read(actor, |r| {
        r.authorize(actor, true)?;
        r.manifest_view(id)
    })?;
    if !saved.started {
        return Err(Failure::State);
    }
    if saved.result.is_some() {
        return Ok(saved);
    }
    let response = ops::manifests::inspect(saved.intent.request.permission).await?;
    ops::mutate(actor, |r| r.acknowledge_manifest(id, Ok(response)))?;
    view(actor, id)
}
pub(crate) fn view(actor: Principal, id: u128) -> Result<ManifestIntentView, Failure> {
    ops::read(actor, |r| r.manifest_view(id))
}
pub(crate) async fn inspect(
    actor: Principal,
    id: u128,
) -> Result<ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse, Failure> {
    let saved = view(actor, id)?;
    ops::manifests::inspect(saved.intent.request.permission).await
}
