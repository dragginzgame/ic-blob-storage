//! Provider callback projection through the existing root/lifecycle owner.
use super::{Memory, StableUploads, UploadStoreError, UploadStoreRecord, key};
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use std::collections::BTreeSet;

impl<M: Memory> StableUploads<M> {
    // Callers authenticate/scope/fence before this access. Bound by lifetime history.
    pub(crate) fn gateway_deletions(
        &self,
        protected: &BTreeSet<ProviderRootHash>,
    ) -> Result<Vec<Vec<u8>>, UploadStoreError> {
        self.mutable()?;
        let mut roots = Vec::new();
        for entry in &self.confirmed {
            let record = entry.value();
            if !record.valid(self.config.limits().catalog) {
                return Err(UploadStoreError::InvalidRecord);
            }
            if record.phase() != LifecyclePhase::DeletionPending || record.gateway_acknowledged() {
                continue;
            }
            let UploadStoreRecord::Permission(permission) = self
                .permissions
                .get(entry.key())
                .ok_or(UploadStoreError::InvalidRecord)?
            else {
                return Err(UploadStoreError::InvalidRecord);
            };
            let view = permission.view().ok_or(UploadStoreError::InvalidRecord)?;
            let request = view.permission.request;
            self.trusted_request(request)?;
            if view.phase != super::UploadPhase::Registered
                || key(request) != *entry.key()
                || self
                    .indexed_permission(request.object.root, request.object.first.object())?
                    .permission
                    .request
                    != request
            {
                return Err(UploadStoreError::InvalidRecord);
            }
            if !protected.contains(&request.object.root) {
                roots.push(request.object.root.as_bytes().to_vec());
                if roots.len() == ic_blob_storage_contracts::provider::gc::MAX_ROOTS {
                    break;
                }
            }
        }
        Ok(roots)
    }
    pub(crate) fn acknowledge_gateway_roots(
        &mut self,
        roots: &BTreeSet<ProviderRootHash>,
        protected: &BTreeSet<ProviderRootHash>,
    ) -> Result<(), UploadStoreError> {
        self.mutable()?;
        let mut changes = Vec::with_capacity(roots.len());
        // Validate the entire batch before writes, including on native backing.
        for root in roots {
            let object = self
                .roots
                .binding_for_owner(*root)?
                .ok_or(UploadStoreError::InvalidRecord)?;
            let view = self.indexed_permission(*root, object)?;
            if protected.contains(root) {
                return Err(UploadStoreError::InvalidRecord);
            }
            let mut record = self.confirmed_record(view.permission.request)?;
            record.acknowledge_gateway()?;
            changes.push((key(view.permission.request), record));
        }
        for (owner, record) in changes {
            self.confirmed.insert(owner, record);
        }
        Ok(())
    }
}
