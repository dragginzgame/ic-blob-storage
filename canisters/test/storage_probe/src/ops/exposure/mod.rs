//! Exact stored-state exposure and scoped atomic-commit fault injection.
use super::{STATE, TRAP_WRITE};
use blob_test_protocol::storage::{WriteFault, exposure::ExposureInput};
use ic_blob_storage::test_support::exposure;
use ic_blob_storage_contracts::{
    dto::upload::{
        admission::{UploadAdmissionRequest, UploadAdmissionResponse},
        certificate::CaffeineUploadCertificateResponse,
        exposure::UploadExposureFailure,
    },
    upload::binding::UploadContext,
};
thread_local! {
    static CERTIFICATE_FIXTURE: std::cell::Cell<Option<ExposureInput>> = const { std::cell::Cell::new(None) };
}
pub(crate) fn configure_certificate(context: UploadContext, input: ExposureInput) {
    STATE.with_borrow(|s| {
        if context.actor != s.as_ref().unwrap().operator {
            ic_platform::cdk::api::trap("fixture operator only");
        }
    });
    CERTIFICATE_FIXTURE.set(Some(input));
}
pub(crate) fn certificate(context: UploadContext, root: &str) -> CaffeineUploadCertificateResponse {
    let faults = CERTIFICATE_FIXTURE.get();
    if let Some(faults) = faults {
        assert_eq!(
            ic_blob_storage_contracts::identity::ProviderRootHash::try_from(
                faults.permission.upload.root.as_slice()
            )
            .expect("fixed root")
            .to_string(),
            root,
            "fixture root selection"
        );
        TRAP_WRITE.set(faults.trap_write.then_some(WriteFault::Permissions));
    }
    let result = STATE.with_borrow_mut(|s| {
        ic_blob_storage::test_support::certificate::issue(
            &mut s.as_mut().unwrap().uploads,
            context,
            root,
            ic_platform::cdk::api::time(),
        )
    });
    TRAP_WRITE.set(None);
    if faults.is_some_and(|faults| faults.trap_after) {
        ic_platform::cdk::api::trap("fixture after certificate exposure write");
    }
    result.unwrap_or_else(|_| ic_platform::cdk::api::trap("certificate issuance refused"))
}
pub(crate) fn commit(
    context: UploadContext,
    input: ExposureInput,
) -> Result<UploadAdmissionResponse, UploadExposureFailure> {
    TRAP_WRITE.set(input.trap_write.then_some(WriteFault::Permissions));
    let result = STATE.with_borrow_mut(|s| {
        exposure::commit(
            &mut s.as_mut().unwrap().uploads,
            context,
            input.permission,
            ic_platform::cdk::api::time(),
        )
    });
    TRAP_WRITE.set(None);
    if input.trap_after {
        ic_platform::cdk::api::trap("fixture after exposure write");
    }
    result?;
    inspect(context, input.permission)
}
pub(crate) fn inspect(
    context: UploadContext,
    input: UploadAdmissionRequest,
) -> Result<UploadAdmissionResponse, UploadExposureFailure> {
    STATE.with_borrow(|s| exposure::inspect(&s.as_ref().unwrap().uploads, context, input))
}
