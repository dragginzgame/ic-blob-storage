//! Bounded upload, manifest and aggregate accounting records.
pub(crate) mod lifecycle;
use super::LifecycleChange;
use super::ObjectBinding;
use super::Principal;
use super::ProviderRootHash;
use super::ServiceConfiguration;
use super::UploadManifestState;
use super::UploadPermissionView;
use super::UploadPhase;
use super::UploadRequest;
use super::UploadRequestId;
use super::manifest;
use crate::model::catalog::admission::UploadUsage;
use candid::{CandidType, DecoderConfig, Deserialize, decode_one_with_config};
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkHash;
use ic_blob_storage_contracts::upload::binding::UploadObject;
use ic_blob_storage_contracts::upload::binding::UploadPermission;
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::{
    borrow::Cow,
    num::{NonZeroU64, NonZeroU128},
};

use ic_blob_storage_contracts::configuration::envelope::MAX_MANIFEST_RECORD_BYTES as MANIFEST_BYTES;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct UploadConfigurationRecord {
    version: u8,
    service: Principal,
    operator: Principal,
    payment_account: Principal,
    namespace: u128,
    max_tenants: u64,
    max_object_bytes: u64,
    max_headers: u64,
    max_header_bytes: u64,
    max_chunks: u64,
    max_tenant_chunks: u64,
    max_objects: u64,
    max_tenant_objects: u64,
    max_physical_bytes: u128,
    max_liability_bytes: u128,
    max_tenant_logical_bytes: u128,
    max_references: u64,
    max_receipts: u64,
    max_active: u64,
    max_tenant_active: u64,
}
impl UploadConfigurationRecord {
    pub(crate) fn new(config: &ServiceConfiguration) -> Self {
        let b = config.bindings();
        let l = config.limits();
        Self {
            version: 1,
            service: b.service,
            operator: b.operator,
            payment_account: b.payment_account,
            namespace: b.namespace.get(),
            max_tenants: l.max_tenants.get() as u64,
            max_object_bytes: l.max_object_bytes.get(),
            max_headers: l.max_headers.get() as u64,
            max_header_bytes: l.max_header_bytes.get() as u64,
            max_chunks: l.manifests.max_chunks.get() as u64,
            max_tenant_chunks: l.manifests.max_tenant_chunks.get() as u64,
            max_objects: l.catalog.max_objects.get() as u64,
            max_tenant_objects: l.catalog.max_tenant_objects.get() as u64,
            max_physical_bytes: l.catalog.max_physical_bytes.get(),
            max_liability_bytes: l.catalog.max_liability_bytes.get(),
            max_tenant_logical_bytes: l.catalog.max_tenant_logical_bytes.get(),
            max_references: l.catalog.max_references_per_object.get() as u64,
            max_receipts: l.catalog.max_receipts_per_object.get() as u64,
            max_active: l.uploads.max_active.get() as u64,
            max_tenant_active: l.uploads.max_tenant_active.get() as u64,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
enum UploadPhaseRecord {
    Reserved,
    ExposurePossible,
    Cancelled,
    Confirmed,
}
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct UploadPermissionRecord {
    version: u8,
    service: Principal,
    tenant: Principal,
    namespace: u128,
    object: u128,
    incarnation: u128,
    reference: u128,
    request: u128,
    root: [u8; 32],
    bytes: u64,
    uploader: Principal,
    expires: u64,
    admitted: u64,
    generation: u64,
    revoked: bool,
    manifest: bool,
    phase: UploadPhaseRecord,
}
impl UploadPermissionRecord {
    pub(crate) fn new(input: UploadPermission, now: u64, generation: NonZeroU64) -> Self {
        let object = input.request.object.first.object();
        let id = object.identity();
        Self {
            version: 1,
            service: object.service(),
            tenant: object.tenant(),
            namespace: id.namespace.get(),
            object: id.object.get(),
            incarnation: id.incarnation.get(),
            reference: input.request.object.first.reference().get().get(),
            request: input.request.id.get().get(),
            root: *input.request.object.root.as_bytes(),
            bytes: input.request.object.bytes,
            uploader: input.uploader,
            expires: input.expires_at_ns,
            admitted: now,
            generation: generation.get(),
            revoked: false,
            manifest: false,
            phase: UploadPhaseRecord::Reserved,
        }
    }
    pub(crate) fn view(&self) -> Option<UploadPermissionView> {
        if self.version != 1
            || self.bytes == 0
            || self.admitted >= self.expires
            || matches!(self.phase, UploadPhaseRecord::Reserved) && self.revoked
            || matches!(self.phase, UploadPhaseRecord::Cancelled) && !self.revoked
            || matches!(
                self.phase,
                UploadPhaseRecord::ExposurePossible | UploadPhaseRecord::Confirmed
            ) && !self.manifest
        {
            return None;
        }
        if self.uploader == Principal::anonymous()
            || self.uploader == Principal::management_canister()
        {
            return None;
        }
        let object = ObjectBinding::new(
            self.service,
            self.tenant,
            ObjectIdentity {
                namespace: NonZeroU128::new(self.namespace)?,
                object: NonZeroU128::new(self.object)?,
                incarnation: NonZeroU128::new(self.incarnation)?,
            },
        )
        .ok()?;
        Some(UploadPermissionView {
            permission: UploadPermission {
                request: UploadRequest {
                    id: UploadRequestId::new(NonZeroU128::new(self.request)?),
                    object: UploadObject {
                        root: ProviderRootHash::try_from(self.root.as_slice()).ok()?,
                        bytes: self.bytes,
                        first: ReferenceKey::new(
                            object,
                            ReferenceId::new(NonZeroU128::new(self.reference)?),
                        ),
                    },
                },
                uploader: self.uploader,
                expires_at_ns: self.expires,
            },
            admitted_at_ns: self.admitted,
            tenant_generation: NonZeroU64::new(self.generation)?,
            revoked: self.revoked,
            manifest: if self.manifest {
                UploadManifestState::Bound
            } else {
                UploadManifestState::Unprepared
            },
            phase: match self.phase {
                UploadPhaseRecord::Reserved => UploadPhase::Reserved,
                UploadPhaseRecord::ExposurePossible => UploadPhase::ExposurePossible,
                UploadPhaseRecord::Cancelled => UploadPhase::Cancelled,
                UploadPhaseRecord::Confirmed => UploadPhase::Confirmed,
            },
        })
    }
    pub(crate) fn bind_manifest(&mut self) {
        assert_eq!(self.phase, UploadPhaseRecord::Reserved);
        self.manifest = true;
    }
    pub(crate) fn expose(&mut self) {
        assert_eq!(self.phase, UploadPhaseRecord::Reserved);
        assert!(self.manifest);
        self.phase = UploadPhaseRecord::ExposurePossible;
    }
    pub(crate) fn confirm(
        &mut self,
    ) -> Result<LifecycleChange, crate::model::catalog::admission::UploadError> {
        match self.phase {
            UploadPhaseRecord::Confirmed => Ok(LifecycleChange::Unchanged),
            UploadPhaseRecord::ExposurePossible => {
                self.phase = UploadPhaseRecord::Confirmed;
                Ok(LifecycleChange::Changed)
            }
            _ => Err(crate::model::catalog::admission::UploadError::InvalidPhase(
                self.view().expect("validated permission").phase,
            )),
        }
    }
    pub(crate) fn revoke(&mut self) -> LifecycleChange {
        if self.revoked {
            return LifecycleChange::Unchanged;
        }
        self.revoked = true;
        if self.phase == UploadPhaseRecord::Reserved {
            self.phase = UploadPhaseRecord::Cancelled;
        }
        LifecycleChange::Changed
    }
}
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum UploadStoreRecord {
    Configuration(UploadConfigurationRecord),
    Permission(UploadPermissionRecord),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct UploadUsageRecord {
    version: u8,
    operations: u64,
    active: u64,
    bytes: u128,
    chunks: u64,
    logical: u128,
    physical: u128,
    liability: u128,
}
impl UploadUsageRecord {
    pub(crate) const fn empty() -> Self {
        Self {
            version: 1,
            operations: 0,
            active: 0,
            bytes: 0,
            chunks: 0,
            logical: 0,
            physical: 0,
            liability: 0,
        }
    }
    pub(crate) fn admit(&mut self, bytes: u64) {
        self.logical += u128::from(bytes);
        self.physical += u128::from(bytes);
        self.liability += u128::from(bytes);
        self.operations = self.operations.checked_add(1).expect("bounded operations");
        self.active = self.active.checked_add(1).expect("bounded active");
        self.bytes = self
            .bytes
            .checked_add(u128::from(bytes))
            .expect("bounded bytes");
        self.chunks = self
            .chunks
            .checked_add(chunks(bytes))
            .expect("bounded leaves");
    }
    pub(crate) fn cancel(&mut self, bytes: u64) {
        self.logical -= u128::from(bytes);
        self.physical -= u128::from(bytes);
        self.liability -= u128::from(bytes);
        self.finish_reservation(bytes);
    }
    pub(crate) fn finish_reservation(&mut self, bytes: u64) {
        self.active = self.active.checked_sub(1).expect("reserved operation");
        self.bytes = self
            .bytes
            .checked_sub(u128::from(bytes))
            .expect("reserved bytes");
    }
    pub(crate) fn view(self) -> Option<UploadUsage> {
        if self.version != 1
            || self.active > self.operations
            || self.bytes > self.logical
            || self.logical > self.physical
            || self.physical > self.liability
        {
            return None;
        }
        Some(UploadUsage {
            operations: usize::try_from(self.operations).ok()?,
            active_reservations: usize::try_from(self.active).ok()?,
            reserved_bytes: self.bytes,
            logical_bytes: self.logical,
            physical_bytes: self.physical,
            liability_bytes: self.liability,
        })
    }
    pub(crate) const fn chunks(self) -> u64 {
        self.chunks
    }
    pub(crate) fn replace_lifecycle(
        &mut self,
        bytes: u64,
        before: ic_blob_storage_contracts::upload::history::LifecyclePhase,
        after: ic_blob_storage_contracts::upload::history::LifecyclePhase,
    ) {
        let old = lifecycle::contribution(bytes, before);
        let new = lifecycle::contribution(bytes, after);
        self.logical = self.logical - old.0 + new.0;
        self.physical = self.physical - old.1 + new.1;
        self.liability = self.liability - old.2 + new.2;
    }
}
pub(crate) fn chunks(bytes: u64) -> u64 {
    bytes.div_ceil(ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES as u64)
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
struct UploadHeaderRecord {
    name: String,
    value: String,
}
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct UploadManifestRecord {
    version: u8,
    chunks: Vec<[u8; 32]>,
    headers: Vec<UploadHeaderRecord>,
}
impl UploadManifestRecord {
    pub(crate) fn into_view(self) -> manifest::UploadManifestView {
        manifest::UploadManifestView {
            chunks: self.chunks,
            headers: self
                .headers
                .into_iter()
                .map(|h| crate::model::service::upload::download::ContentHeader {
                    name: h.name,
                    value: h.value,
                })
                .collect(),
        }
    }
    /// Select from an immutable manifest already root-bound at admission and
    /// validated on reopen. This range check does not independently authenticate
    /// a new manifest or rebuild its tree.
    pub(crate) fn read_leaf(
        &self,
        content_bytes: u64,
        index: u64,
    ) -> Option<(
        ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkRange,
        CaffeineChunkHash,
    )> {
        let chunk_bytes =
            ic_blob_storage_contracts::identity::caffeine::CAFFEINE_CHUNK_BYTES as u64;
        if self.version != 1 || self.chunks.len() as u64 != content_bytes.div_ceil(chunk_bytes) {
            return None;
        }
        let leaf = self.chunks.get(usize::try_from(index).ok()?)?;
        let offset = index.checked_mul(chunk_bytes)?;
        let bytes = usize::try_from(content_bytes.checked_sub(offset)?.min(chunk_bytes)).ok()?;
        Some((
            ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineChunkRange {
                index,
                offset,
                bytes,
            },
            CaffeineChunkHash::try_from(leaf.as_slice()).ok()?,
        ))
    }
    pub(crate) fn into_headers(
        self,
    ) -> Vec<crate::model::service::upload::download::ContentHeader> {
        self.headers
            .into_iter()
            .map(|h| crate::model::service::upload::download::ContentHeader {
                name: h.name,
                value: h.value,
            })
            .collect()
    }
    pub(crate) fn new(input: manifest::UploadManifest<'_>) -> Self {
        Self {
            version: 1,
            chunks: input.chunks.iter().map(|v| *v.as_bytes()).collect(),
            headers: input
                .headers
                .iter()
                .map(|h| UploadHeaderRecord {
                    name: h.name.into(),
                    value: h.value.into(),
                })
                .collect(),
        }
    }
    pub(crate) fn matches(&self, input: manifest::UploadManifest<'_>) -> bool {
        self.chunks.len() == input.chunks.len()
            && self
                .chunks
                .iter()
                .zip(input.chunks)
                .all(|(a, b)| a == b.as_bytes())
    }
    pub(crate) fn validate(&self, config: &ServiceConfiguration, request: UploadRequest) -> bool {
        if self.version != 1
            || self.chunks.len() > config.manifest_limits().max_chunks.get()
            || self.headers.len() > config.limits().max_headers.get()
        {
            return false;
        }
        let chunks = self
            .chunks
            .iter()
            .map(|v| CaffeineChunkHash::try_from(v.as_slice()).expect("fixed leaf"))
            .collect::<Vec<_>>();
        let headers = self
            .headers
            .iter()
            .map(|h| CaffeineHeader {
                name: &h.name,
                value: &h.value,
            })
            .collect::<Vec<_>>();
        manifest::validate(
            config,
            request,
            manifest::UploadManifest {
                chunks: &chunks,
                headers: &headers,
            },
        )
        .is_ok()
    }
}

macro_rules! codec {
    ($record:ty,$bound:expr,$layout:expr) => {
        impl Storable for $record {
            fn to_bytes(&self) -> Cow<'_, [u8]> {
                let bytes = candid::encode_one(self).expect("upload record encoding");
                assert!(bytes.len() <= $bound, "upload record bound");
                Cow::Owned(bytes)
            }
            fn into_bytes(self) -> Vec<u8> {
                self.to_bytes().into_owned()
            }
            fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
                assert!(bytes.len() <= $bound, "upload record bound");
                let mut limits = DecoderConfig::new();
                limits
                    .set_decoding_quota(2_000_000)
                    .set_skipping_quota(1000)
                    .set_max_type_len(64)
                    .set_max_header_len(2048)
                    .set_full_error_message(false);
                decode_one_with_config(&bytes, &limits).expect("valid upload record")
            }
            const BOUND: Bound = $layout;
        }
    };
}
pub(crate) use codec;
codec!(
    UploadStoreRecord,
    2048,
    Bound::Bounded {
        max_size: 2048,
        is_fixed_size: false
    }
);
codec!(
    UploadUsageRecord,
    512,
    Bound::Bounded {
        max_size: 512,
        is_fixed_size: false
    }
);
// The codec still enforces 64 KiB. Variable-size B-tree pages avoid sizing every
// node for several maximum manifests when most objects have only a few leaves.
codec!(UploadManifestRecord, MANIFEST_BYTES, Bound::Unbounded);

#[cfg(test)]
mod tests;
