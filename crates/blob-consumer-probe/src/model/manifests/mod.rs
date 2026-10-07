//! Two lifetime uploader intents inside the existing bounded fixture record.
#[cfg(test)]
mod tests;
use super::ConsumerRecord;
use blob_test_protocol::consumer::{
    Failure,
    manifests::{ManifestIntent, ManifestIntentView},
};
use candid::{CandidType, Principal};
use ic_blob_storage::{
    dto::upload::manifest::{
        UploadManifestDeclaration, UploadManifestFailure, UploadManifestInspection,
        UploadManifestResponse,
    },
    model::{
        identity::{
            ProviderRootHash,
            caffeine::{
                CaffeineHeader,
                manifest::{CaffeineChunkHash, CaffeineChunkManifest, CaffeineManifestLimits},
            },
        },
        service::upload::manifest::validate_upload_metadata,
    },
};
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct ManifestIntentRecord {
    intent: ManifestIntent,
    cancelled: bool,
    started: bool,
    result: Option<Result<UploadManifestDeclaration, UploadManifestFailure>>,
}
pub(crate) fn limits() -> CaffeineManifestLimits {
    CaffeineManifestLimits {
        max_content_bytes: 10.try_into().unwrap(),
        max_chunks: 1.try_into().unwrap(),
        max_headers: 8.try_into().unwrap(),
        max_header_bytes: 1024.try_into().unwrap(),
    }
}
fn declaration(intent: &ManifestIntent, input: &UploadManifestDeclaration) -> Result<(), Failure> {
    let limits = limits();
    if input.chunks.len() > limits.max_chunks.get()
        || input.headers.len() > limits.max_headers.get()
    {
        return Err(Failure::Capacity);
    }
    let mut remaining = limits.max_header_bytes.get();
    for h in &input.headers {
        for len in [h.name.len(), h.value.len(), 3] {
            remaining = remaining.checked_sub(len).ok_or(Failure::Capacity)?;
        }
    }
    let headers = input
        .headers
        .iter()
        .map(|h| CaffeineHeader {
            name: &h.name,
            value: &h.value,
        })
        .collect::<Vec<_>>();
    let chunks = input
        .chunks
        .iter()
        .map(|c| CaffeineChunkHash::try_from(c.as_slice()).expect("fixed leaf"))
        .collect::<Vec<_>>();
    let upload = intent.request.permission.upload;
    validate_upload_metadata(
        &headers,
        upload.bytes,
        limits.max_headers.get(),
        limits.max_header_bytes.get(),
    )
    .map_err(|_| Failure::Invalid)?;
    CaffeineChunkManifest::new(
        ProviderRootHash::try_from(upload.root.as_slice()).expect("fixed root"),
        upload.bytes,
        &chunks,
        &headers,
        limits,
    )
    .map_err(|_| Failure::Invalid)?;
    Ok(())
}
impl ConsumerRecord {
    fn check_manifest(&self, intent: &ManifestIntent) -> Result<(), Failure> {
        let p = intent.request.permission;
        let u = p.upload;
        if intent.id == 0
            || u.service != self.service
            || p.uploader != self.tenant
            || [Principal::anonymous(), Principal::management_canister()].contains(&u.tenant)
            || [
                u.namespace,
                u.upload,
                u.object,
                u.incarnation,
                u.first_reference,
            ]
            .contains(&0)
        {
            return Err(Failure::Invalid);
        }
        declaration(intent, &intent.request.declaration)
    }
    pub(crate) fn validate_manifests(&self) -> Result<(), Failure> {
        if self.manifests.len() > 2 {
            return Err(Failure::Capacity);
        }
        let mut checked = Self::new(self.operator, self.service, self.tenant);
        for m in &self.manifests {
            checked.save_manifest(&m.intent)?;
            if m.result.is_some() && !m.started {
                return Err(Failure::State);
            }
            if let Some(Ok(result)) = &m.result {
                declaration(&m.intent, result)?;
                if result.chunks != m.intent.request.declaration.chunks {
                    return Err(Failure::Conflict);
                }
            }
        }
        if checked.manifests.len() != self.manifests.len() {
            return Err(Failure::Conflict);
        }
        Ok(())
    }
    pub(crate) fn save_manifest(&mut self, intent: &ManifestIntent) -> Result<(), Failure> {
        self.check_manifest(intent)?;
        if let Some(old) = self.manifests.iter().find(|m| m.intent.id == intent.id) {
            return if old.intent == *intent {
                Ok(())
            } else {
                Err(Failure::Conflict)
            };
        }
        let u = intent.request.permission.upload;
        if self.manifests.iter().any(|m| {
            let old = m.intent.request.permission.upload;
            old.service == u.service
                && old.tenant == u.tenant
                && old.namespace == u.namespace
                && old.upload == u.upload
        }) {
            return Err(Failure::Conflict);
        }
        if self.manifests.len() == 2 {
            return Err(Failure::Capacity);
        }
        self.manifests.push(ManifestIntentRecord {
            intent: intent.clone(),
            cancelled: false,
            started: false,
            result: None,
        });
        Ok(())
    }
    pub(crate) fn manifest_view(&self, id: u128) -> Result<ManifestIntentView, Failure> {
        let m = self
            .manifests
            .iter()
            .find(|m| m.intent.id == id)
            .ok_or(Failure::Unknown)?;
        Ok(ManifestIntentView {
            intent: m.intent.clone(),
            cancelled: m.cancelled,
            started: m.started,
            result: m.result.clone(),
        })
    }
    pub(crate) fn start_manifest(&mut self, id: u128) -> Result<bool, Failure> {
        let m = self
            .manifests
            .iter_mut()
            .find(|m| m.intent.id == id)
            .ok_or(Failure::Unknown)?;
        if m.cancelled {
            return Err(Failure::State);
        }
        if m.result.is_some() {
            return Ok(false);
        }
        if m.started {
            return Err(Failure::Pending);
        }
        m.started = true;
        Ok(true)
    }
    pub(crate) fn cancel_manifest(&mut self, id: u128) -> Result<(), Failure> {
        let m = self
            .manifests
            .iter_mut()
            .find(|m| m.intent.id == id)
            .ok_or(Failure::Unknown)?;
        // Keep dispatch uncertainty and accepted/refused history independently of
        // cancellation. Only the tenant can withdraw permission at the service.
        m.cancelled = true;
        Ok(())
    }
    pub(crate) fn acknowledge_manifest(
        &mut self,
        id: u128,
        response: Result<UploadManifestResponse, UploadManifestFailure>,
    ) -> Result<(), Failure> {
        let m = self
            .manifests
            .iter_mut()
            .find(|m| m.intent.id == id)
            .ok_or(Failure::Unknown)?;
        if !m.started {
            return Err(Failure::State);
        }
        let result = match response {
            Ok(response) => {
                if response.permission != m.intent.request.permission {
                    return Err(Failure::Conflict);
                }
                let UploadManifestInspection::Prepared(retained) = response.manifest else {
                    return Err(Failure::Pending);
                };
                declaration(&m.intent, &retained)?;
                if retained.chunks != m.intent.request.declaration.chunks {
                    return Err(Failure::Conflict);
                }
                Ok(retained)
            }
            Err(error) => Err(error),
        };
        if m.result.as_ref().is_some_and(|old| *old != result) {
            return Err(Failure::Conflict);
        }
        m.result = Some(result);
        Ok(())
    }
}
