//! Indexed tenant reads and bounded current-state traversal; no effect authority.
use super::{
    Memory, StableUploads, UploadContext, UploadPhase, UploadRequest, UploadStoreError,
    UploadStoreRecord, key, metadata, validation,
};
use crate::model::catalog::admission::read::UploadPageLimits;
use crate::model::lifecycle::ReferenceState;
use crate::model::service::upload::UploadAdmissionError;
use crate::model::service::upload::UploadManifestState;
use crate::model::service::upload::content::ContentLookup;
use crate::model::service::upload::content::TenantContentView;
use crate::model::service::upload::download::ContentHeader;
use candid::Principal;
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::HashParseError;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::identity::batch::ProviderRootBatch;
use ic_blob_storage_contracts::upload::binding::UploadRequestId;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadRootState;
use ic_blob_storage_contracts::upload::history::UploadScanFilter;
use std::{
    num::NonZeroU128,
    ops::Bound::{Excluded, Included, Unbounded},
};

mod authority;
pub(crate) mod download;
pub mod verification;

/// Owned copy of one bounded declaration, not a provider/publication capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadDescriptorView {
    /// Exact original request and current local state.
    pub content: TenantContentView,
    /// Original accepted headers, without rewriting name/case/value.
    pub headers: Vec<ContentHeader>,
}
/// Descriptor checked with the exact currently live reference in the same read.
/// A saved response can become stale; publication/release coordination is external.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedUploadDescriptorView {
    /// Confirmed declaration and local phase.
    pub descriptor: UploadDescriptorView,
    /// Exact live reference, not another reference keeping the object alive.
    pub reference: ReferenceKey,
}
/// Explicit scope checked independently of the cursor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadScanScope {
    /// Only the authenticated tenant's operation range is inspected.
    Tenant {
        /// Tenant must equal the actual caller.
        tenant: Principal,
        /// Installed namespace, never inferred from a content digest.
        namespace: NonZeroU128,
    },
    /// Operator-only scan across tenants in the installed namespace.
    Service {
        /// Installed namespace; the operator is taken from configuration.
        namespace: NonZeroU128,
    },
}
/// Untrusted forward position, not a snapshot, receipt, freshness or retry authority.
/// Insertions and phase changes behind this position require a fresh sweep.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadScanCursor {
    /// Service that produced the position.
    pub service: Principal,
    /// Complete original scan scope.
    pub scope: UploadScanScope,
    /// Original filter; filtered progress cannot silently change meaning.
    pub filter: UploadScanFilter,
    /// Tenant of the last inspected operation.
    pub after_tenant: Principal,
    /// Last inspected request identity, not necessarily a returned match.
    pub after_request: UploadRequestId,
}
/// Bounded current observations in tenant/request-ID order, never a completed sweep proof.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadScanPage {
    /// Matching operations only; no manifests, reference histories or file bytes.
    pub entries: Vec<TenantContentView>,
    /// Continue after the last inspected row, including empty filtered pages.
    pub next: Option<UploadScanCursor>,
    /// Operation rows inspected, including filtered-out history.
    pub scanned: usize,
}

/// Operator observation at one supplied root position; never provider effect authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UploadRootObservation {
    /// Exact local identity and current reservation/confirmed state.
    Known(TenantContentView),
    /// No local claim; this does not authorize deletion or retry.
    Unknown,
    /// Malformed raw root, preserved in its original position.
    Malformed(HashParseError),
}
impl<M: Memory> StableUploads<M> {
    /// Inspect a bounded provider-root batch as the configured service operator.
    ///
    /// Checks service, namespace and operator even for empty batches. Each valid
    /// position uses indexed reads, without scanning history or loading manifests.
    /// Order, duplicates, malformed inputs and all retained phases are preserved.
    /// Hosts must bound decoding and construct the batch with trusted limits.
    /// This is an operator reconciliation aid, not a gateway callback contract,
    /// completion fact, deletion boolean, freshness proof or permission to retry.
    /// # Errors
    /// Rejects scope/authority before reading roots, or inconsistent private indexes.
    pub fn observe_roots(
        &self,
        execution: UploadContext,
        namespace: NonZeroU128,
        batch: &ProviderRootBatch,
    ) -> Result<Vec<UploadRootObservation>, UploadStoreError> {
        self.scan_authority(execution, UploadScanScope::Service { namespace })?;
        self.observe_roots_for_owner(batch)
    }

    // Internal synchronous access after workflow authority checks. No public bypass.
    pub(crate) fn observe_roots_for_owner(
        &self,
        batch: &ProviderRootBatch,
    ) -> Result<Vec<UploadRootObservation>, UploadStoreError> {
        batch
            .entries()
            .iter()
            .map(|entry| {
                let root = match entry {
                    Ok(root) => *root,
                    Err(error) => return Ok(UploadRootObservation::Malformed(*error)),
                };
                let Some(object) = self.roots.binding_for_owner(root)? else {
                    return Ok(UploadRootObservation::Unknown);
                };
                Ok(UploadRootObservation::Known(
                    self.indexed_content(root, object)?,
                ))
            })
            .collect()
    }
    /// Find the original upload through logarithmic root/request lookups.
    /// Foreign and absent roots both return `None`; suspension and fencing preserve reads.
    /// # Errors
    /// Rejects wrong service/namespace/tenant or inconsistent private indexes.
    pub fn lookup_content(
        &self,
        execution: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<TenantContentView>, UploadStoreError> {
        validation::tenant(&self.config, execution, input.tenant, input.namespace)?;
        let Some(object) = self.roots.lookup(execution, input.root)? else {
            return Ok(None);
        };
        Ok(Some(self.indexed_content(input.root, object)?))
    }
    fn indexed_content(
        &self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<TenantContentView, UploadStoreError> {
        let view = self.indexed_permission(root, object)?;
        self.content_view(view.permission.request, view.phase)
    }
    pub(super) fn indexed_permission(
        &self,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<crate::model::service::upload::UploadPermissionView, UploadStoreError> {
        let id = self
            .root_requests
            .get(root.as_bytes())
            .ok_or(UploadStoreError::InvalidRecord)?;
        let Some(UploadStoreRecord::Permission(record)) =
            self.permissions.get(&(object.tenant(), id))
        else {
            return Err(UploadStoreError::InvalidRecord);
        };
        let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
        let request = view.permission.request;
        if request.object.root != root
            || request.object.first.object() != object
            || request.id.get().get() != id
        {
            return Err(UploadStoreError::InvalidRecord);
        }
        Ok(view)
    }
    /// Copy the first validated metadata of a tenant's prepared operation.
    /// Loads one bounded manifest record, never the file or other operations' metadata.
    /// # Errors
    /// Rejects scope/authority or inconsistent records. Unknown/unprepared roots return `None`.
    pub fn content_descriptor(
        &self,
        execution: UploadContext,
        input: ContentLookup,
    ) -> Result<Option<UploadDescriptorView>, UploadStoreError> {
        let Some(content) = self.lookup_content(execution, input)? else {
            return Ok(None);
        };
        self.descriptor(content)
    }
    /// Observe a declaration only with confirmed completion and the exact live reference.
    /// Released/unknown references and changed object lifetimes return `None` even if
    /// another reference is live. This synchronous read performs no provider action.
    /// # Errors
    /// Rejects scope/tenant authority or inconsistent retained state.
    pub fn retained_content_descriptor(
        &self,
        execution: UploadContext,
        root: ProviderRootHash,
        reference: ReferenceKey,
    ) -> Result<Option<RetainedUploadDescriptorView>, UploadStoreError> {
        let Some(content) = self.retained_content(execution, root, reference)? else {
            return Ok(None);
        };
        let descriptor = self
            .descriptor(content)?
            .ok_or(UploadStoreError::InvalidRecord)?;
        Ok(Some(RetainedUploadDescriptorView {
            descriptor,
            reference,
        }))
    }
    fn retained_content(
        &self,
        execution: UploadContext,
        root: ProviderRootHash,
        reference: ReferenceKey,
    ) -> Result<Option<TenantContentView>, UploadStoreError> {
        let object = reference.object();
        validation::object(&self.config, execution, object)?;
        let Some(content) = self.lookup_content(
            execution,
            ContentLookup {
                tenant: object.tenant(),
                namespace: object.identity().namespace,
                root,
            },
        )?
        else {
            return Ok(None);
        };
        if content.request.object.first.object() != object
            || content.state != UploadRootState::Confirmed(LifecyclePhase::Live)
        {
            return Ok(None);
        }
        if self.reference_state(key(content.request), reference.reference().get().get())?
            != Some(ReferenceState::Active)
        {
            return Ok(None);
        }
        Ok(Some(content))
    }
    fn descriptor(
        &self,
        content: TenantContentView,
    ) -> Result<Option<UploadDescriptorView>, UploadStoreError> {
        let permission = self
            .required(content.request)?
            .view()
            .ok_or(UploadStoreError::InvalidRecord)?;
        if permission.manifest == UploadManifestState::Unprepared {
            return Ok(None);
        }
        let record = self
            .manifests
            .get(&key(content.request))
            .ok_or(UploadStoreError::InvalidRecord)?;
        Ok(Some(UploadDescriptorView {
            content,
            headers: record.into_headers(),
        }))
    }
    /// Scan bounded history under tenant authority or configured operator authority.
    /// `limits` must come from trusted host policy, not unrestricted caller input.
    /// Empty filtered pages advance. Pages are current views: start a new sweep to
    /// find insertions/phase changes behind a cursor. Restored reads confer no unfence.
    /// # Errors
    /// Rejects scope/authority/cursor mismatch before traversal, or invalid retained rows.
    pub fn scan(
        &self,
        execution: UploadContext,
        scope: UploadScanScope,
        filter: UploadScanFilter,
        cursor: Option<UploadScanCursor>,
        limits: UploadPageLimits,
    ) -> Result<UploadScanPage, UploadStoreError> {
        self.scan_authority(execution, scope)?;
        if let Some(c) = cursor {
            let tenant_matches = match scope {
                UploadScanScope::Tenant { tenant, .. } => c.after_tenant == tenant,
                UploadScanScope::Service { .. } => true,
            };
            if c.service != execution.service
                || c.scope != scope
                || c.filter != filter
                || !tenant_matches
            {
                return Err(UploadStoreError::CursorScope);
            }
        }
        let (initial, end) = match scope {
            UploadScanScope::Tenant { tenant, .. } => {
                (Included((tenant, 1)), Included((tenant, u128::MAX)))
            }
            UploadScanScope::Service { .. } => (Excluded(metadata()), Unbounded),
        };
        let start = cursor.map_or(initial, |c| {
            Excluded((c.after_tenant, c.after_request.get().get()))
        });
        let mut rows = self.permissions.range((start, end)).peekable();
        let mut page = UploadScanPage {
            entries: Vec::new(),
            next: None,
            scanned: 0,
        };
        let mut last = None;
        while page.scanned < limits.max_scan.get() && page.entries.len() < limits.max_results.get()
        {
            let Some(entry) = rows.next() else {
                break;
            };
            let UploadStoreRecord::Permission(record) = entry.value() else {
                return Err(UploadStoreError::InvalidRecord);
            };
            let view = record.view().ok_or(UploadStoreError::InvalidRecord)?;
            if key(view.permission.request) != *entry.key() {
                return Err(UploadStoreError::InvalidRecord);
            }
            page.scanned += 1;
            last = Some((entry.key().0, view.permission.request.id));
            let content = self.content_view(view.permission.request, view.phase)?;
            if filter.includes(content.state) {
                page.entries.push(content);
            }
        }
        if rows.peek().is_some() {
            page.next = last.map(|(tenant, id)| UploadScanCursor {
                service: execution.service,
                scope,
                filter,
                after_tenant: tenant,
                after_request: id,
            });
        }
        Ok(page)
    }
    fn scan_authority(
        &self,
        execution: UploadContext,
        scope: UploadScanScope,
    ) -> Result<(), UploadStoreError> {
        match scope {
            UploadScanScope::Tenant { tenant, namespace } => {
                validation::tenant(&self.config, execution, tenant, namespace)?;
            }
            UploadScanScope::Service { namespace } => {
                if execution.service != self.config.bindings().service {
                    return Err(UploadAdmissionError::WrongService.into());
                }
                if namespace != self.config.bindings().namespace {
                    return Err(UploadAdmissionError::WrongNamespace.into());
                }
                if execution.actor != self.config.bindings().operator {
                    return Err(UploadAdmissionError::NotOperator.into());
                }
            }
        }
        Ok(())
    }
    fn content_view(
        &self,
        request: UploadRequest,
        phase: UploadPhase,
    ) -> Result<TenantContentView, UploadStoreError> {
        let state = match phase {
            UploadPhase::Reserved => UploadRootState::Reserved,
            UploadPhase::ExposurePossible => UploadRootState::ExposurePossible,
            UploadPhase::Cancelled => UploadRootState::Cancelled,
            UploadPhase::Confirmed => UploadRootState::Confirmed(
                self.confirmed
                    .get(&key(request))
                    .filter(|r| r.valid(self.config.limits().catalog))
                    .ok_or(UploadStoreError::InvalidRecord)?
                    .phase(),
            ),
        };
        Ok(TenantContentView { request, state })
    }
}
