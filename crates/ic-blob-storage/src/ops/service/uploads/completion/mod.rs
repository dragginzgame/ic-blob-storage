//! Authenticate exact verifier statements and commit them with upload accounting.
pub mod verification;
use super::{StableUploads, admission};
use crate::model::catalog::admission::UploadPhase;
use crate::model::service::upload::UploadPermissionView;
use crate::model::service::upload::record::lifecycle::AttestationRecord;
use crate::model::service::upload::record::lifecycle::CompletionRecord;
use crate::policy::upload::completion::may_attest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationLookup;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationMutation;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationReceipt;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_memory::ic_stable_structures::Memory;

fn exact<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadPermissionView, UploadAttestationFailure> {
    let bindings = store.config.bindings();
    if authority.service() != bindings.service
        || authority.namespace() != bindings.namespace
        || context.service != authority.service()
    {
        return Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Binding,
        ));
    }
    let permission =
        ic_blob_storage_contracts::upload::admission::parse_binding(context.service, input)
            .map_err(UploadAttestationFailure::Permission)?;
    super::validation::object(
        &store.config,
        context,
        permission.request.object.first.object(),
    )
    .map_err(|_| UploadAttestationFailure::Permission(UploadAdmissionFailure::Binding))?;
    let view = store
        .required(permission.request)
        .map_err(failure)?
        .view()
        .ok_or(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Internal,
        ))?;
    if view.permission != permission {
        return Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Conflict,
        ));
    }
    Ok(view)
}
fn failure(error: super::UploadStoreError) -> UploadAttestationFailure {
    UploadAttestationFailure::Permission(admission::failure(error))
}
fn receipt(
    permission: UploadAdmissionRequest,
    record: AttestationRecord,
) -> UploadAttestationReceipt {
    UploadAttestationReceipt {
        request: UploadAttestationRequest {
            permission,
            content_digest: record.content_digest,
            observed_at_ns: record.observed_at_ns,
        },
        verifier: record.verifier,
        accepted_at_ns: record.accepted_at_ns,
    }
}
pub(crate) fn inspect<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAttestationResponse, UploadAttestationFailure> {
    let observer = context.actor == authority.verifier()
        || context.actor == input.uploader
        || context.actor == input.upload.tenant;
    if !observer {
        return Err(UploadAttestationFailure::Denied);
    }
    let view = exact(store, authority, context, input)?;
    let attestation = if view.phase == UploadPhase::Confirmed {
        match store
            .confirmed_record(view.permission.request)
            .map_err(failure)?
            .completion()
        {
            CompletionRecord::HostFact => UploadAttestationLookup::Absent,
            CompletionRecord::Attested(record) => {
                UploadAttestationLookup::Found(receipt(input, record))
            }
        }
    } else {
        UploadAttestationLookup::Absent
    };
    Ok(UploadAttestationResponse {
        permission: input,
        attestation,
        fenced: store.is_fenced(),
    })
}
pub(crate) fn attest<M: Memory>(
    store: &mut StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: &UploadAttestationRequest,
    now: u64,
) -> Result<UploadAttestationMutation, UploadAttestationFailure> {
    if !may_attest(authority, context) {
        return Err(UploadAttestationFailure::Denied);
    }
    let view = exact(store, authority, context, input.permission)?;
    store.mutable().map_err(failure)?;
    if view.phase == UploadPhase::Confirmed {
        let UploadAttestationLookup::Found(original) =
            inspect(store, authority, context, input.permission)?.attestation
        else {
            return Err(UploadAttestationFailure::Phase);
        };
        if original.request != *input || original.verifier != authority.verifier() {
            return Err(UploadAttestationFailure::Conflict);
        }
        return Ok(UploadAttestationMutation {
            receipt: original,
            changed: false,
        });
    }
    if view.phase != UploadPhase::ExposurePossible {
        return Err(UploadAttestationFailure::Phase);
    }
    if input.observed_at_ns < view.admitted_at_ns || input.observed_at_ns > now {
        return Err(UploadAttestationFailure::Observation);
    }
    let record = AttestationRecord {
        verifier: context.actor,
        content_digest: input.content_digest,
        observed_at_ns: input.observed_at_ns,
        accepted_at_ns: now,
    };
    store
        .complete_upload(view.permission.request, CompletionRecord::Attested(record))
        .map_err(failure)?;
    // Receipt, first reference, permission and accounting share the caller's synchronous IC update.
    Ok(UploadAttestationMutation {
        receipt: receipt(input.permission, record),
        changed: true,
    })
}

pub(crate) fn manifest<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse,
    ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure,
> {
    use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
    if !may_attest(authority, context) {
        return Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Denied,
        ));
    }
    let view = exact(store, authority, context, input).map_err(|e| match e {
        UploadAttestationFailure::Permission(e) => UploadManifestFailure::Permission(e),
        _ => UploadManifestFailure::Permission(UploadAdmissionFailure::Internal),
    })?;
    super::manifests::observation(store, input, &view)
}
