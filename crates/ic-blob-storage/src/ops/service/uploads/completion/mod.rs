//! Authenticate exact verifier statements and commit them with upload accounting.
pub mod reply;
pub mod verification;
use super::{StableUploads, admission};
use crate::{
    dto::upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionRequest},
        completion::{
            UploadAttestationFailure, UploadAttestationLookup, UploadAttestationMutation,
            UploadAttestationReceipt, UploadAttestationRequest, UploadAttestationResponse,
        },
    },
    model::{
        catalog::admission::UploadPhase,
        service::upload::{
            UploadContext, UploadPermissionView,
            completion::CompletionAuthority,
            record::lifecycle::{AttestationRecord, CompletionRecord},
        },
    },
    policy::upload::completion::may_attest,
};
use ic_memory::ic_stable_structures::Memory;
/// Canonical verifier update; linking the library exports no endpoint.
pub const UPLOAD_ATTEST_METHOD: &str = "blob_attest_upload";
/// Canonical historical receipt query; absence never authorizes a new upload.
pub const UPLOAD_ATTESTATION_METHOD: &str = "blob_upload_attestation";

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
    let permission = admission::parse_binding(context.service, input)
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

/// Canonical verifier-only historical manifest query; no fetch or attestation is implied.
pub const UPLOAD_VERIFICATION_MANIFEST_METHOD: &str = "blob_verification_manifest";
pub(crate) fn manifest<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<
    crate::dto::upload::manifest::UploadManifestResponse,
    crate::dto::upload::manifest::UploadManifestFailure,
> {
    use crate::dto::upload::manifest::UploadManifestFailure;
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
