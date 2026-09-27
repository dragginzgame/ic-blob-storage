//! Incremental durable root claims. Upload intent and accounting must join their
//! transaction before this component can support any certificate/provider effect.
use crate::model::{
    identity::ProviderRootHash,
    lifecycle::{
        binding::ObjectBinding,
        roots::{
            RootClaimError, RootClaimOutcome, plan_claim,
            record::{RootClaimRecord, RootObjectKeyRecord, RootStoreMetadataRecord},
        },
    },
    service::{configuration::ServiceConfiguration, upload::UploadContext},
};
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
use thiserror::Error;

const METADATA: [u8; 33] = [0; 33];
fn root_key(root: ProviderRootHash) -> [u8; 33] {
    let mut key = [1; 33];
    key[1..].copy_from_slice(root.as_bytes());
    key
}

/// Immutable lifetime claims in two distinct, exclusively host-granted memories.
///
/// The owning IC message must commit both indexes together with the upload intent
/// and its accounting, without awaiting. Storage traps must propagate for platform
/// rollback. Native memory offers no such transaction guarantee. This component
/// does not enroll tenants, allocate IDs, authenticate providers or prove uploads.
pub struct StableRootClaims<M: Memory> {
    roots: BTreeMap<[u8; 33], RootClaimRecord, M>,
    objects: BTreeMap<RootObjectKeyRecord, [u8; 32], M>,
    config: ServiceConfiguration,
    fenced: bool,
}

impl<M: Memory> StableRootClaims<M> {
    /// Install only in two fresh memories supplied by the host's `ic-memory` runtime.
    /// # Errors
    /// Rejects either allocated memory, never overwriting existing obligations.
    /// # Panics
    /// Traps on aliased memories, allocation or encoding failures. The host must
    /// propagate the trap to roll back all writes in the IC message.
    pub fn install(
        roots: M,
        objects: M,
        config: ServiceConfiguration,
    ) -> Result<Self, RootStoreError> {
        if roots.size() != 0 || objects.size() != 0 {
            return Err(RootStoreError::AlreadyAllocated);
        }
        let mut roots = BTreeMap::new(roots);
        assert_eq!(
            objects.size(),
            0,
            "root and object memories must be distinct"
        );
        let objects = BTreeMap::new(objects);
        roots.insert(
            METADATA,
            RootClaimRecord::Metadata(RootStoreMetadataRecord::new(&config)),
        );
        Ok(Self {
            roots,
            objects,
            config,
            fenced: false,
        })
    }

    /// Load retained indexes and validate their exact bijection before inspection.
    ///
    /// Reopening always fences mutation. Host installation/release checks and a
    /// complete obligation inventory remain necessary; this does not unfreeze an
    /// older backup. Only service, namespace and lifetime root limit bind this store.
    /// # Errors
    /// Rejects missing memory, changed configuration, malformed claims, excess
    /// lifetime history and missing/conflicting/orphaned reverse-index entries.
    /// # Panics
    /// Corrupt stable collection/record encodings trap; no replacement is seeded.
    pub fn open(
        roots: M,
        objects: M,
        config: ServiceConfiguration,
    ) -> Result<Self, RootStoreError> {
        if roots.size() == 0 || objects.size() == 0 {
            return Err(RootStoreError::Missing);
        }
        let roots = BTreeMap::load(roots);
        let objects = BTreeMap::load(objects);
        if roots.get(&METADATA)
            != Some(RootClaimRecord::Metadata(RootStoreMetadataRecord::new(
                &config,
            )))
        {
            return Err(RootStoreError::Binding);
        }
        if roots.len() - 1 > config.limits().catalog.max_objects.get() as u64
            || objects.len() != roots.len() - 1
        {
            return Err(RootStoreError::InvalidRecord);
        }
        let store = Self {
            roots,
            objects,
            config,
            fenced: true,
        };
        for entry in store.roots.iter() {
            let key = entry.key();
            if *key == METADATA {
                continue;
            }
            if key[0] != 1 {
                return Err(RootStoreError::InvalidRecord);
            }
            let object = store.binding(entry.value())?;
            let reverse = store.objects.get(&RootObjectKeyRecord::new(object));
            if reverse.as_ref().map(<[u8; 32]>::as_slice) != Some(&key[1..]) {
                return Err(RootStoreError::InvalidRecord);
            }
        }
        Ok(store)
    }

    /// Claim under an authenticated tenant context; exact replay consumes no slot.
    ///
    /// The enclosing admission workflow must also check enrollment and all quota
    /// bounds, then persist upload intent/accounting in this same IC transaction.
    /// A successful root claim alone grants no permission to expose a certificate.
    /// # Errors
    /// Rejects wrong service/tenant/namespace, restored stores, conflicting identity
    /// reuse and exhausted lifetime capacity. Typed rejection writes nothing.
    /// # Panics
    /// Storage writes may trap; the host must propagate failure for IC rollback.
    pub fn claim(
        &mut self,
        context: UploadContext,
        root: ProviderRootHash,
        object: ObjectBinding,
    ) -> Result<RootClaimOutcome, RootStoreError> {
        self.check_service(context)?;
        if object.service() != context.service {
            return Err(RootClaimError::WrongService.into());
        }
        if context.actor != object.tenant() {
            return Err(RootStoreError::Denied);
        }
        if object.identity().namespace != self.config.bindings().namespace {
            return Err(RootStoreError::WrongNamespace);
        }
        if self.fenced {
            return Err(RootStoreError::Fenced);
        }
        let key = root_key(root);
        let existing = self
            .roots
            .get(&key)
            .map(|record| self.binding(record))
            .transpose()?;
        let object_key = RootObjectKeyRecord::new(object);
        let outcome = plan_claim(
            context.service,
            object,
            existing,
            self.objects.contains_key(&object_key),
            self.slots() < self.config.limits().catalog.max_objects.get() as u64,
        )?;
        if outcome == RootClaimOutcome::Claimed {
            self.roots
                .insert(key, RootClaimRecord::Claim(object_key.clone()));
            self.objects.insert(object_key, *root.as_bytes());
        }
        Ok(outcome)
    }

    /// Tenant-only correlation lookup. Unknown and foreign roots both return none.
    /// # Errors
    /// Rejects the wrong running service or a corrupt retained binding.
    pub fn lookup(
        &self,
        context: UploadContext,
        root: ProviderRootHash,
    ) -> Result<Option<ObjectBinding>, RootStoreError> {
        self.check_service(context)?;
        let binding = self.binding_for_owner(root)?;
        Ok(binding.filter(|object| object.tenant() == context.actor))
    }

    // Only a containing owner may authorize disclosure across tenants.
    pub(super) fn binding_for_owner(
        &self,
        root: ProviderRootHash,
    ) -> Result<Option<ObjectBinding>, RootStoreError> {
        self.roots
            .get(&root_key(root))
            .map(|record| self.binding(record))
            .transpose()
    }

    /// Lifetime count for trusted host accounting, not a tenant-authorized endpoint.
    #[must_use]
    pub fn slots(&self) -> u64 {
        self.roots.len() - 1
    }

    pub(super) fn contains(&self, root: ProviderRootHash) -> bool {
        self.roots.contains_key(&root_key(root))
    }

    /// Every reopened store remains inspection-only, with no reset/unfence path.
    #[must_use]
    pub const fn is_fenced(&self) -> bool {
        self.fenced
    }

    fn check_service(&self, context: UploadContext) -> Result<(), RootStoreError> {
        if context.service != self.config.bindings().service {
            return Err(RootClaimError::WrongService.into());
        }
        Ok(())
    }

    fn binding(&self, record: RootClaimRecord) -> Result<ObjectBinding, RootStoreError> {
        let RootClaimRecord::Claim(key) = record else {
            return Err(RootStoreError::InvalidRecord);
        };
        let binding = key
            .binding(self.config.bindings().service)
            .ok_or(RootStoreError::InvalidRecord)?;
        if binding.identity().namespace != self.config.bindings().namespace {
            return Err(RootStoreError::InvalidRecord);
        }
        Ok(binding)
    }
}

/// Rejected stable claim operation; malformed binary storage traps separately.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum RootStoreError {
    /// Either supplied memory already has allocated contents.
    #[error("root memory already allocated")]
    AlreadyAllocated,
    /// One required index memory is absent.
    #[error("root memory missing")]
    Missing,
    /// Stored schema or scope differs from expected configuration.
    #[error("root store binding mismatch")]
    Binding,
    /// Invalid claim or inconsistent forward/reverse indexes.
    #[error("invalid root store record")]
    InvalidRecord,
    /// Caller does not own the exact object being claimed.
    #[error("root claim denied")]
    Denied,
    /// Object belongs to another provider namespace.
    #[error("wrong root namespace")]
    WrongNamespace,
    /// Reopened state cannot regain operational authority from its own counters.
    #[error("root store fenced")]
    Fenced,
    /// Shared model uniqueness or lifetime-capacity rejection.
    #[error(transparent)]
    Claim(#[from] RootClaimError),
}

#[cfg(test)]
mod tests;
