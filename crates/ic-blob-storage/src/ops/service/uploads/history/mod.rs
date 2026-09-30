//! Passive boundary conversion around the existing bounded stable scan.
pub mod reply;
use crate::{
    dto::{
        reference::ReferenceUpload,
        upload::history::{
            UploadContentState, UploadHistoryCursor, UploadHistoryEntry, UploadHistoryFailure,
            UploadHistoryFilter, UploadHistoryPage, UploadHistoryRequest, UploadHistoryScope,
        },
    },
    model::{
        catalog::admission::{
            UploadRequestId,
            read::{UploadPageLimits, UploadRootState},
        },
        lifecycle::LifecyclePhase,
        service::upload::{UploadAdmissionError, UploadContext, content::TenantContentView},
    },
    ops::service::uploads::{
        StableUploads, UploadStoreError,
        read::{UploadScanCursor, UploadScanFilter, UploadScanScope},
    },
};
use ic_memory::ic_stable_structures::Memory;
use std::num::NonZeroU128;

/// Canonical bounded history query. Linking exports no endpoint.
pub const UPLOAD_HISTORY_METHOD: &str = "blob_upload_history";

fn number(value: u128) -> Result<NonZeroU128, UploadHistoryFailure> {
    NonZeroU128::new(value).ok_or(UploadHistoryFailure::Invalid)
}
fn scope(
    input: UploadHistoryScope,
    namespace: u128,
) -> Result<UploadScanScope, UploadHistoryFailure> {
    let namespace = number(namespace)?;
    Ok(match input {
        UploadHistoryScope::Tenant(tenant) => UploadScanScope::Tenant { tenant, namespace },
        UploadHistoryScope::Service => UploadScanScope::Service { namespace },
    })
}
fn filter(input: UploadHistoryFilter) -> UploadScanFilter {
    match input {
        UploadHistoryFilter::All => UploadScanFilter::All,
        UploadHistoryFilter::Active => UploadScanFilter::Active,
        UploadHistoryFilter::DeletionPending => UploadScanFilter::DeletionPending,
        UploadHistoryFilter::Outstanding => UploadScanFilter::Outstanding,
    }
}
fn cursor(input: UploadHistoryCursor) -> Result<UploadScanCursor, UploadHistoryFailure> {
    Ok(UploadScanCursor {
        service: input.service,
        scope: scope(input.scope, input.namespace)?,
        filter: filter(input.filter),
        after_tenant: input.after_tenant,
        after_request: UploadRequestId::new(number(input.after_request)?),
    })
}
pub(super) fn present(view: TenantContentView) -> UploadHistoryEntry {
    let request = view.request;
    let first = request.object.first;
    let object = first.object();
    UploadHistoryEntry {
        request: ReferenceUpload {
            service: object.service(),
            tenant: object.tenant(),
            namespace: object.identity().namespace.get(),
            upload: request.id.get().get(),
            object: object.identity().object.get(),
            incarnation: object.identity().incarnation.get(),
            first_reference: first.reference().get().get(),
            root: *request.object.root.as_bytes(),
            bytes: request.object.bytes,
        },
        state: match view.state {
            UploadRootState::Reserved => UploadContentState::Reserved,
            UploadRootState::ExposurePossible => UploadContentState::ExposurePossible,
            UploadRootState::Cancelled => UploadContentState::Cancelled,
            UploadRootState::Confirmed(phase) => match phase {
                LifecyclePhase::Live => UploadContentState::Live,
                LifecyclePhase::DeletionPending => UploadContentState::DeletionPending,
                LifecyclePhase::ProviderDeleted => UploadContentState::ProviderDeleted,
                LifecyclePhase::Settled => UploadContentState::Settled,
            },
        },
    }
}
fn failure(error: UploadStoreError) -> UploadHistoryFailure {
    use UploadAdmissionError as A;
    use UploadHistoryFailure as F;
    match error {
        UploadStoreError::Binding
        | UploadStoreError::Admission(A::WrongService | A::WrongNamespace) => F::Binding,
        UploadStoreError::Admission(A::NotProject | A::NotOperator | A::NotObserver) => F::Denied,
        UploadStoreError::CursorScope => F::CursorScope,
        _ => F::Internal,
    }
}
pub(crate) fn inspect<M: Memory>(
    uploads: &StableUploads<M>,
    context: UploadContext,
    input: UploadHistoryRequest,
    limits: UploadPageLimits,
) -> Result<UploadHistoryPage, UploadHistoryFailure> {
    if input.service != context.service {
        return Err(UploadHistoryFailure::Binding);
    }
    let page = uploads
        .scan(
            context,
            scope(input.scope, input.namespace)?,
            filter(input.filter),
            input.cursor.map(cursor).transpose()?,
            limits,
        )
        .map_err(failure)?;
    Ok(UploadHistoryPage {
        request: input,
        entries: page.entries.into_iter().map(present).collect(),
        next: page.next.map(|position| UploadHistoryCursor {
            service: input.service,
            namespace: input.namespace,
            scope: input.scope,
            filter: input.filter,
            after_tenant: position.after_tenant,
            after_request: position.after_request.get().get(),
        }),
        scanned: u64::try_from(page.scanned).map_err(|_| UploadHistoryFailure::Internal)?,
        fenced: uploads.is_fenced(),
    })
}
