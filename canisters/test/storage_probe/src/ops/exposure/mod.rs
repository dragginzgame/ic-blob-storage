//! Local evidence substitution around shared exposure and certificate response gates.
use super::{STATE, TRAP_WRITE};
use blob_test_protocol::storage::{
    WriteFault,
    exposure::{ExposureInput, ExposureOutcome, ExposureScenario},
};
use ic_blob_storage::policy::upload::exposure::UploadExposureHostEvidence;
use ic_blob_storage::workflow::uploads::exposure;
use ic_blob_storage::workflow::uploads::exposure::UploadExposureResult;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureBlocker;
use ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::binding::UploadObject;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_blob_storage_contracts::upload::binding::UploadRequest;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;
use std::num::NonZeroU128;
thread_local! {
    // Private test setup only. Upgrade clears it; production ingress never supplies facts.
    static CERTIFICATE_FIXTURE: std::cell::Cell<Option<ExposureInput>> = const { std::cell::Cell::new(None) };
}
pub(crate) fn configure_certificate(context: UploadContext, input: ExposureInput) {
    STATE.with_borrow(|s| {
        if context.actor != s.as_ref().unwrap().operator {
            ic_cdk::trap("fixture operator only");
        }
    });
    CERTIFICATE_FIXTURE.set(Some(input));
}
pub(crate) fn certificate(
    context: UploadContext,
    root: &str,
) -> ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse {
    let input = CERTIFICATE_FIXTURE
        .get()
        .unwrap_or_else(|| ic_cdk::trap("fixture host evidence absent"));
    let now = ic_cdk::api::time();
    let evidence =
        substitute(input, now).unwrap_or_else(|_| ic_cdk::trap("fixture invalid evidence"));
    TRAP_WRITE.set(input.trap_write.then_some(WriteFault::Permissions));
    let result = STATE.with_borrow_mut(|s| {
        ic_blob_storage::workflow::uploads::certificate::issue(
            &mut s.as_mut().unwrap().uploads,
            context,
            root,
            evidence,
            now,
        )
    });
    TRAP_WRITE.set(None);
    if input.trap_after {
        ic_cdk::trap("fixture after certificate exposure write");
    }
    result.unwrap_or_else(|_| ic_cdk::trap("certificate issuance refused"))
}
fn substitute(
    input: ExposureInput,
    now: u64,
) -> Result<UploadExposureHostEvidence, UploadExposureFailure> {
    let p = input.permission;
    let u = p.upload;
    let invalid = || UploadExposureFailure::Permission(UploadAdmissionFailure::Invalid);
    let positive = |n| NonZeroU128::new(n).ok_or_else(invalid);
    let object = ObjectBinding::new(
        u.service,
        u.tenant,
        ObjectIdentity {
            namespace: positive(u.namespace)?,
            object: positive(u.object)?,
            incarnation: positive(u.incarnation)?,
        },
    )
    .map_err(|_| invalid())?;
    let mut permission = UploadPermission {
        request: UploadRequest {
            id: UploadRequestId::new(positive(u.upload)?),
            object: UploadObject {
                root: ProviderRootHash::try_from(u.root.as_slice()).expect("fixed root"),
                bytes: u.bytes,
                first: ReferenceKey::new(object, ReferenceId::new(positive(u.first_reference)?)),
            },
        },
        uploader: p.uploader,
        expires_at_ns: p.expires_at_ns,
    };
    if input.scenario == ExposureScenario::ForeignSubstitute {
        permission.expires_at_ns ^= 1;
    }
    let established = input.scenario != ExposureScenario::Unknown;
    Ok(UploadExposureHostEvidence {
        permission,
        observed_at_ns: if input.scenario == ExposureScenario::StaleSubstitute {
            now.wrapping_sub(1)
        } else {
            now
        },
        namespace_binding: established,
        trusted_uploader: established,
        current_owner: established,
        durable_commit: established,
    })
}
pub(crate) fn preview(
    context: UploadContext,
    input: ExposureInput,
) -> Result<Vec<UploadExposureBlocker>, UploadExposureFailure> {
    let now = ic_cdk::api::time();
    let evidence = substitute(input, now)?;
    STATE.with_borrow(|s| {
        exposure::inspect_preparation(
            &s.as_ref().unwrap().uploads,
            context,
            input.permission,
            evidence,
            now,
        )
        .map(ic_blob_storage::ops::service::uploads::exposure::blockers)
    })
}
pub(crate) fn commit(
    context: UploadContext,
    input: ExposureInput,
) -> Result<ExposureOutcome, UploadExposureFailure> {
    let now = ic_cdk::api::time();
    let evidence = substitute(input, now)?;
    TRAP_WRITE.set(input.trap_write.then_some(WriteFault::Permissions));
    let result = STATE.with_borrow_mut(|s| {
        exposure::commit(
            &mut s.as_mut().unwrap().uploads,
            context,
            input.permission,
            evidence,
            now,
        )
    });
    TRAP_WRITE.set(None);
    if input.trap_after {
        ic_cdk::trap("fixture after exposure write");
    }
    result.map(|r| match r {
        UploadExposureResult::Blocked(a) => ExposureOutcome::Blocked(
            ic_blob_storage::ops::service::uploads::exposure::blockers(a),
        ),
        UploadExposureResult::Exposed(v) => ExposureOutcome::Exposed(v),
    })
}
pub(crate) fn inspect(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadExposureFailure> {
    STATE.with_borrow(|s| exposure::inspect(&s.as_ref().unwrap().uploads, context, input))
}
