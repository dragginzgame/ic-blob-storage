//! Installed provider mapping and original declaration for an exposed unfinished upload.
use super::{StableUploads, exact, failure};
use crate::model::catalog::admission::UploadPhase;
use crate::policy::upload::completion::may_attest;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure;
use ic_blob_storage_contracts::dto::upload::completion::UploadVerificationPlan;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestInspection;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use ic_memory::ic_stable_structures::Memory;

pub(crate) fn plan<M: Memory>(
    store: &StableUploads<M>,
    authority: CompletionAuthority,
    scope: &CaffeineDownloadScope,
    context: UploadContext,
    permission: UploadAdmissionRequest,
) -> Result<UploadVerificationPlan, UploadAttestationFailure> {
    if !may_attest(authority, context) {
        return Err(UploadAttestationFailure::Denied);
    }
    if scope.owner() != authority.service() || scope.namespace() != authority.namespace() {
        return Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Binding,
        ));
    }
    let view = exact(store, authority, context, permission)?;
    store.mutable().map_err(failure)?;
    if view.phase != UploadPhase::ExposurePossible {
        return Err(UploadAttestationFailure::Phase);
    }
    let manifest = super::super::manifests::observation(store, permission, &view)
        .map_err(|_| UploadAttestationFailure::Permission(UploadAdmissionFailure::Internal))?;
    let UploadManifestInspection::Prepared(declaration) = manifest.manifest else {
        return Err(UploadAttestationFailure::Phase);
    };
    // Revoked/suspended exposed uploads still need reconciliation; no new upload is allowed.
    Ok(UploadVerificationPlan {
        permission,
        verifier: authority.verifier(),
        owner: scope.owner(),
        project: scope.project().to_owned(),
        admitted_at_ns: view.admitted_at_ns,
        declaration,
    })
}
