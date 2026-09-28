//! Bounded fixture read conversion and delegation to the durable owner.
pub(crate) mod authority;
pub(crate) mod download;
pub(crate) mod sessions;
pub(crate) mod transport;
use super::{STATE, conversion};
use blob_test_protocol::{
    admission::{
        ContentDescriptor, ContentLookup, ContentObservation, ContentState, Request,
        input::{RetainedDescriptor, RetainedDescriptorInput},
    },
    storage::{
        Failure,
        read::{Cursor, Filter, Page, ScanInput, Scope},
    },
};
use ic_blob_storage::{
    model::{
        catalog::admission::{
            UploadRequestId,
            read::{UploadPageLimits, UploadRootState},
        },
        identity::ProviderRootHash,
        lifecycle::{
            LifecyclePhase, ReferenceId,
            binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        },
        service::upload::{
            UploadContext,
            content::{ContentLookup as Lookup, TenantContentView},
        },
    },
    ops::service::uploads::read::{
        UploadDescriptorView, UploadScanCursor, UploadScanFilter, UploadScanScope,
    },
};
use std::num::{NonZeroU128, NonZeroUsize};
fn number(value: u128) -> Result<NonZeroU128, Failure> {
    NonZeroU128::new(value).ok_or(Failure::Invalid)
}
pub(super) fn lookup(execution: UploadContext, input: ContentLookup) -> Result<Lookup, Failure> {
    if input.service != execution.service {
        return Err(Failure::Binding);
    }
    Ok(Lookup {
        tenant: input.tenant,
        namespace: number(input.namespace)?,
        root: ProviderRootHash::try_from(input.root.as_slice()).expect("fixed root"),
    })
}
pub(super) fn observation(view: TenantContentView) -> ContentObservation {
    let request = view.request;
    ContentObservation {
        request: Request {
            service: request.object.first.object().service(),
            tenant: request.object.first.object().tenant(),
            namespace: request.object.first.object().identity().namespace.get(),
            id: request.id.get().get(),
            root: *request.object.root.as_bytes(),
            bytes: request.object.bytes,
        },
        state: content_state(view.state),
    }
}
pub(crate) fn content_state(state: UploadRootState) -> ContentState {
    match state {
        UploadRootState::Reserved => ContentState::Reserved,
        UploadRootState::ExposurePossible => ContentState::ExposurePossible,
        UploadRootState::Cancelled => ContentState::Cancelled,
        UploadRootState::Confirmed(phase) => match phase {
            LifecyclePhase::Live => ContentState::Live,
            LifecyclePhase::DeletionPending => ContentState::DeletionPending,
            LifecyclePhase::ProviderDeleted => ContentState::ProviderDeleted,
            LifecyclePhase::Settled => ContentState::Settled,
        },
    }
}
pub(super) fn descriptor(view: UploadDescriptorView) -> ContentDescriptor {
    ContentDescriptor {
        content: observation(view.content),
        headers: view
            .headers
            .into_iter()
            .map(|h| (h.name, h.value))
            .collect(),
    }
}
pub(crate) fn content(
    execution: UploadContext,
    input: ContentLookup,
) -> Result<Option<ContentObservation>, Failure> {
    let input = lookup(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .lookup_content(execution, input)
        })
        .map(|v| v.map(observation))
        .map_err(conversion::failure)
}
pub(crate) fn declaration(
    execution: UploadContext,
    input: ContentLookup,
) -> Result<Option<ContentDescriptor>, Failure> {
    let input = lookup(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .content_descriptor(execution, input)
        })
        .map(|v| v.map(descriptor))
        .map_err(conversion::failure)
}
pub(crate) fn retained(
    execution: UploadContext,
    input: RetainedDescriptorInput,
) -> Result<Option<RetainedDescriptor>, Failure> {
    let (root, reference) = retained_target(execution, input)?;
    STATE
        .with_borrow(|state| {
            state
                .as_ref()
                .unwrap()
                .uploads
                .retained_content_descriptor(execution, root, reference)
        })
        .map(|v| {
            v.map(|v| RetainedDescriptor {
                reference: input,
                descriptor: descriptor(v.descriptor),
            })
        })
        .map_err(conversion::failure)
}
pub(super) fn retained_target(
    execution: UploadContext,
    input: RetainedDescriptorInput,
) -> Result<(ProviderRootHash, ReferenceKey), Failure> {
    let content = lookup(execution, input.content)?;
    let object = ObjectBinding::new(
        execution.service,
        content.tenant,
        ObjectIdentity {
            namespace: content.namespace,
            object: number(input.object)?,
            incarnation: number(input.incarnation)?,
        },
    )
    .map_err(|_| Failure::Invalid)?;
    let reference = ReferenceKey::new(object, ReferenceId::new(number(input.reference)?));
    Ok((content.root, reference))
}
fn scope(input: Scope, namespace: u128) -> Result<UploadScanScope, Failure> {
    let namespace = number(namespace)?;
    Ok(match input {
        Scope::Tenant(tenant) => UploadScanScope::Tenant { tenant, namespace },
        Scope::Service => UploadScanScope::Service { namespace },
    })
}
fn filter(input: Filter) -> UploadScanFilter {
    match input {
        Filter::All => UploadScanFilter::All,
        Filter::Active => UploadScanFilter::Active,
        Filter::DeletionPending => UploadScanFilter::DeletionPending,
        Filter::Outstanding => UploadScanFilter::Outstanding,
    }
}
fn wire_filter(input: UploadScanFilter) -> Filter {
    match input {
        UploadScanFilter::All => Filter::All,
        UploadScanFilter::Active => Filter::Active,
        UploadScanFilter::DeletionPending => Filter::DeletionPending,
        UploadScanFilter::Outstanding => Filter::Outstanding,
    }
}
fn cursor(input: Cursor) -> Result<UploadScanCursor, Failure> {
    Ok(UploadScanCursor {
        service: input.service,
        scope: scope(input.scope, input.namespace)?,
        filter: filter(input.filter),
        after_tenant: input.after_tenant,
        after_request: UploadRequestId::new(number(input.after_request)?),
    })
}
fn wire_cursor(input: UploadScanCursor) -> Cursor {
    let (scope, namespace) = match input.scope {
        UploadScanScope::Tenant { tenant, namespace } => (Scope::Tenant(tenant), namespace.get()),
        UploadScanScope::Service { namespace } => (Scope::Service, namespace.get()),
    };
    Cursor {
        service: input.service,
        scope,
        namespace,
        filter: wire_filter(input.filter),
        after_tenant: input.after_tenant,
        after_request: input.after_request.get().get(),
    }
}
pub(crate) fn scan(execution: UploadContext, input: ScanInput) -> Result<Page, Failure> {
    if input.service != execution.service {
        return Err(Failure::Binding);
    }
    let scope = scope(input.scope, input.namespace)?;
    let cursor = input.cursor.map(cursor).transpose()?;
    // Explicit tiny fixture bounds test empty filtered pages and continuation.
    let limits = UploadPageLimits {
        max_scan: NonZeroUsize::MIN,
        max_results: NonZeroUsize::MIN,
    };
    STATE
        .with_borrow(|state| {
            state.as_ref().unwrap().uploads.scan(
                execution,
                scope,
                filter(input.filter),
                cursor,
                limits,
            )
        })
        .map(|v| Page {
            entries: v.entries.into_iter().map(observation).collect(),
            next: v.next.map(wire_cursor),
            scanned: v.scanned as u64,
        })
        .map_err(conversion::failure)
}
