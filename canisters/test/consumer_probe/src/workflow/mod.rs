//! Application transactions surround shared service clients without borrowing across awaits.
pub(crate) mod manifests;
pub(crate) mod tenants;
use crate::ops;
use blob_test_protocol::consumer::{AssetView, Failure, Fault, RegistrationSource, Run};
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
        match input.registration.source {
            RegistrationSource::Existing(command) => {
                let receipt = ops::apply(command, input.max_reply_bytes).await?;
                ops::fault(input.fault, Fault::AfterRetain);
                ops::mutate(actor, |r| r.complete(id, false, receipt))?;
            }
            RegistrationSource::Fresh(permission) => {
                let response = ops::admission_status(permission).await?;
                ops::fault(input.fault, Fault::AfterUploadObservation);
                ops::mutate(actor, |r| r.acknowledge_admission(id, Ok(response)))?;
            }
        }
    }
    let view = ops::view(actor, id)?;
    if view.cancelled || view.published || !ops::read(actor, |r| r.owns(id))? {
        return Ok(view);
    }
    ops::descriptor(view.registration.source).await?;
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
    let source = ops::read(actor, |r| {
        r.authorize(actor, true)?;
        Ok(r.view(id)?.registration.source)
    })?;
    if !release && let RegistrationSource::Fresh(permission) = source {
        let response = ops::admission_status(permission).await?;
        ops::mutate(actor, |r| r.acknowledge_admission(id, Ok(response)))?;
        return ops::view(actor, id);
    }
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

pub(crate) async fn admit(actor: Principal, input: &Run) -> Result<AssetView, Failure> {
    let RegistrationSource::Fresh(permission) = input.registration.source else {
        return Err(Failure::Invalid);
    };
    let id = input.registration.asset;
    let send = ops::mutate(actor, |r| {
        r.prepare(&input.registration)?;
        r.start_admission(id)
    })?;
    ops::fault(input.fault, Fault::AfterIntent);
    if send {
        let result = ops::admit(permission, input.max_reply_bytes).await?;
        ops::fault(input.fault, Fault::AfterAdmission);
        ops::mutate(actor, |r| r.acknowledge_admission(id, result))?;
    }
    ops::view(actor, id)
}

pub(crate) fn prepare(
    actor: Principal,
    input: &blob_test_protocol::consumer::Registration,
) -> Result<AssetView, Failure> {
    ops::mutate(actor, |r| {
        r.prepare(input)?;
        r.view(input.asset)
    })
}

pub(crate) async fn revoke(
    actor: Principal,
    input: blob_test_protocol::consumer::Revocation,
) -> Result<AssetView, Failure> {
    let id = input.asset;
    let send = ops::mutate(actor, |r| r.start_revocation(id))?;
    ops::fault(input.fault, Fault::AfterIntent);
    if send {
        let permission = ops::read(actor, |r| r.revocation_permission(id))?;
        let response = ops::revoke(permission, input.max_reply_bytes).await?;
        ops::fault(input.fault, Fault::AfterRevocation);
        ops::mutate(actor, |r| r.acknowledge_revocation(id, response))?;
    }
    ops::view(actor, id)
}
pub(crate) async fn recover_revocation(actor: Principal, id: u128) -> Result<AssetView, Failure> {
    let permission = ops::read(actor, |r| {
        r.authorize(actor, true)?;
        if !r.view(id)?.revocation_started {
            return Err(Failure::State);
        }
        r.revocation_permission(id)
    })?;
    let response = ops::admission_status(permission).await?;
    ops::mutate(actor, |r| r.acknowledge_revocation(id, Ok(response)))?;
    ops::view(actor, id)
}
