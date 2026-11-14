//! Experimental, in-memory control model for one application and candidate binding.
//!
//! This is the sole local owner of grants, root matching, reference bindings and
//! declared-byte accounting. Registered objects use the existing reference owner.
//! Roots identify objects only within this provisional model; no provider deletion
//! scope is qualified. No certificate, provider message or settlement is produced.
//!
//! Hosts must supply independently captured callers and clock readings, never
//! caller claims from request fields. These synchronous operations perform no
//! external awaits. Ordinary errors precede mutation. There is no persistence,
//! upgrade, serialization, cloning or reset contract. Dropping this model loses
//! its history: it MUST NOT back real upload authority or provider obligations.

mod types;
pub use types::{
    Error, Limits, ReferenceStatus, Root, UploadId, UploadRequest, UploadState, UploadView, Usage,
};

use std::collections::BTreeMap;

use ic_platform_types::Principal;

use crate::caffeine::{Header, check_manifest};
use crate::references::{
    ObjectReferences, ReferenceError, ReferenceId, ReferenceState, RetentionState,
};

struct Upload {
    view: UploadView,
    admitted_at: u64,
}

struct Object {
    bytes: u64,
    references: Option<ObjectReferences>,
}

#[derive(Clone, Copy)]
struct Reference {
    root: Root,
    // Predetermined references retain the grant identity even after cancellation,
    // so an old release cannot affect a later reservation of the same root.
    upload: Option<UploadId>,
}

/// Bounded local rehearsal of the service rules, with no provider effects.
pub struct LocalModel {
    application: Principal,
    limits: Limits,
    uploads: BTreeMap<UploadId, Upload>,
    objects: BTreeMap<Root, Object>,
    references: BTreeMap<ReferenceId, Reference>,
    reserved_bytes: u64,
    live_bytes: u64,
    unresolved_bytes: u64,
}

impl LocalModel {
    /// Start an empty model with a fixed application authority and resource bounds.
    /// # Errors
    /// Rejects anonymous/management application authorities.
    pub fn new(application: Principal, limits: Limits) -> Result<Self, Error> {
        valid_principal(application)?;
        Ok(Self {
            application,
            limits,
            uploads: BTreeMap::new(),
            objects: BTreeMap::new(),
            references: BTreeMap::new(),
            reserved_bytes: 0,
            live_bytes: 0,
            unresolved_bytes: 0,
        })
    }

    /// Reserve a new object, or attach a use to a matching Open object.
    ///
    /// Exact request replay returns its current historical disposition without
    /// renewing deadlines, references or byte usage. A replay does not revalidate
    /// the supplied manifest: its original declaration was already admitted.
    /// New declarations must pass manifest preflight before any mutation.
    /// # Errors
    /// Rejects wrong authority, conflicting identities, expired/invalid grants,
    /// invalid manifests, competing reservations, Closing objects or capacity.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn authorize(
        &mut self,
        caller: Principal,
        request: UploadRequest,
        chunks: &[[u8; 32]],
        headers: &[Header<'_>],
        now: u64,
    ) -> Result<UploadView, Error> {
        self.application(caller)?;
        if let Some(upload) = self.uploads.get(&request.id) {
            return if upload.view.request == request {
                Ok(upload.view)
            } else {
                Err(Error::Conflict)
            };
        }
        valid_principal(request.uploader)?;
        if now >= request.expires_at {
            return Err(Error::Expired);
        }
        if self.references.contains_key(&request.first_reference) {
            return Err(Error::Conflict);
        }
        if self.uploads.len() >= self.limits.uploads.get()
            || self.references.len() >= self.limits.references.get()
        {
            return Err(Error::Capacity);
        }
        check_manifest(
            request.root,
            request.bytes,
            chunks,
            headers,
            self.limits.manifest,
        )
        .map_err(Error::Manifest)?;
        let state = if let Some(object) = self.objects.get(&request.root) {
            if object.bytes != request.bytes {
                return Err(Error::Conflict);
            }
            let references = object.references.as_ref().ok_or(Error::ObjectReserved)?;
            if references.retention_state() == RetentionState::Closing {
                return Err(Error::Reference(ReferenceError::Closing));
            }
            UploadState::Attached
        } else {
            if request.bytes
                > self.limits.logical_bytes.get() - self.reserved_bytes - self.live_bytes
                || request.bytes
                    > self.limits.exposure_bytes.get() - self.reserved_bytes - self.unresolved_bytes
            {
                return Err(Error::Capacity);
            }
            UploadState::Reserved
        };
        // All fallible checks precede writes. The global reference budget also
        // bounds each object's slots, so retain cannot exceed its local budget.
        if state == UploadState::Attached {
            self.objects
                .get_mut(&request.root)
                .unwrap()
                .references
                .as_mut()
                .unwrap()
                .retain(request.first_reference)
                .map_err(Error::Reference)?;
        } else {
            self.objects.insert(
                request.root,
                Object {
                    bytes: request.bytes,
                    references: None,
                },
            );
            self.reserved_bytes += request.bytes;
        }
        let view = UploadView { request, state };
        self.uploads.insert(
            request.id,
            Upload {
                view,
                admitted_at: now,
            },
        );
        self.references.insert(
            request.first_reference,
            Reference {
                root: request.root,
                upload: Some(request.id),
            },
        );
        Ok(view)
    }

    /// Consume the bound uploader's grant in one local transition.
    ///
    /// Creates the known first reference and transfers reserved bytes to live and
    /// unresolved declared usage together. Returns no upload authority. Repeating
    /// registration, including through an attached upload ID, is refused.
    /// # Errors
    /// Rejects unknown/wrong-uploader, consumed/retired or expired grants and a
    /// clock earlier than admission, without releasing their reservations.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn register(&mut self, caller: Principal, id: UploadId, now: u64) -> Result<(), Error> {
        let upload = self.uploads.get(&id).ok_or(Error::UnknownUpload)?;
        let request = upload.view.request;
        if caller != request.uploader {
            return Err(Error::Denied);
        }
        if upload.view.state != UploadState::Reserved {
            return Err(Error::NotReserved);
        }
        if now < upload.admitted_at {
            return Err(Error::ClockReversed);
        }
        if now >= request.expires_at {
            return Err(Error::Expired);
        }
        let references = ObjectReferences::new(request.first_reference, self.limits.references);
        self.objects.get_mut(&request.root).unwrap().references = Some(references);
        self.uploads.get_mut(&id).unwrap().view.state = UploadState::Registered;
        self.reserved_bytes -= request.bytes;
        self.live_bytes += request.bytes;
        self.unresolved_bytes += request.bytes;
        Ok(())
    }

    /// Explicitly retire an expired, unconsumed grant and release its byte reservation.
    /// Retired names/history remain occupied. Repeating expiry of that grant succeeds.
    /// # Errors
    /// Rejects wrong authority, unknown/non-reserved grants or an unexpired deadline.
    pub fn expire(&mut self, caller: Principal, id: UploadId, now: u64) -> Result<(), Error> {
        self.application(caller)?;
        let upload = self.uploads.get(&id).ok_or(Error::UnknownUpload)?;
        if upload.view.state == UploadState::Expired {
            return Ok(());
        }
        if upload.view.state != UploadState::Reserved {
            return Err(Error::NotReserved);
        }
        if now < upload.view.request.expires_at {
            return Err(Error::NotExpired);
        }
        self.retire(id, UploadState::Expired);
        Ok(())
    }

    /// Retain a named application use of an already registered Open object.
    /// Historical successful retains replay without resurrection or extra usage.
    /// # Errors
    /// Rejects wrong authority/target, reserved/retired names, unknown/unregistered
    /// objects, Closing objects or exhausted lifetime reference capacity.
    pub fn retain(&mut self, caller: Principal, root: Root, id: ReferenceId) -> Result<(), Error> {
        self.application(caller)?;
        if let Some(reference) = self.references.get(&id).copied() {
            if reference.root != root {
                return Err(Error::Conflict);
            }
            return match self.reference_status(id, reference) {
                ReferenceStatus::Live | ReferenceStatus::Released => Ok(()),
                ReferenceStatus::Reserved | ReferenceStatus::Retired => {
                    Err(Error::ReferenceUnavailable)
                }
            };
        }
        if self.references.len() >= self.limits.references.get() {
            return Err(Error::Capacity);
        }
        let object = self.objects.get_mut(&root).ok_or(Error::UnknownObject)?;
        object
            .references
            .as_mut()
            .ok_or(Error::ObjectReserved)?
            .retain(id)
            .map_err(Error::Reference)?;
        self.references.insert(id, Reference { root, upload: None });
        Ok(())
    }

    /// Release a reference, or cancel its still-unconsumed first-reference grant.
    ///
    /// Final live release removes logical usage but preserves unresolved usage.
    /// Repeated release succeeds without touching a later reservation of the root.
    /// No new slot is needed, including at capacity.
    /// # Errors
    /// Rejects wrong authority or an unknown reference. Refusals do not mutate state.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn release(&mut self, caller: Principal, id: ReferenceId) -> Result<(), Error> {
        self.application(caller)?;
        let reference = *self.references.get(&id).ok_or(Error::UnknownReference)?;
        match self.reference_status(id, reference) {
            ReferenceStatus::Reserved => {
                self.retire(reference.upload.unwrap(), UploadState::Cancelled)
            }
            ReferenceStatus::Live => {
                let object = self.objects.get_mut(&reference.root).unwrap();
                let references = object.references.as_mut().unwrap();
                references.release(id).map_err(Error::Reference)?;
                if references.retention_state() == RetentionState::Closing {
                    self.live_bytes -= object.bytes;
                }
            }
            ReferenceStatus::Released | ReferenceStatus::Retired => {}
        }
        Ok(())
    }

    /// Inspect one exact upload as its bound uploader or the application authority.
    /// # Errors
    /// Rejects unknown IDs or callers other than those two actors.
    pub fn inspect_upload(&self, caller: Principal, id: UploadId) -> Result<UploadView, Error> {
        let view = self.uploads.get(&id).ok_or(Error::UnknownUpload)?.view;
        if caller != self.application && caller != view.request.uploader {
            return Err(Error::Denied);
        }
        Ok(view)
    }

    /// Inspect current reference liveness as the application authority.
    /// # Errors
    /// Rejects wrong authority or an unknown reference.
    pub fn inspect_reference(
        &self,
        caller: Principal,
        id: ReferenceId,
    ) -> Result<ReferenceStatus, Error> {
        self.application(caller)?;
        let reference = *self.references.get(&id).ok_or(Error::UnknownReference)?;
        Ok(self.reference_status(id, reference))
    }

    /// Inspect separate declared counters and lifetime record occupancy.
    /// # Errors
    /// Rejects callers other than the application authority.
    pub fn usage(&self, caller: Principal) -> Result<Usage, Error> {
        self.application(caller)?;
        Ok(Usage {
            reserved_bytes: self.reserved_bytes,
            live_bytes: self.live_bytes,
            unresolved_bytes: self.unresolved_bytes,
            uploads: self.uploads.len(),
            references: self.references.len(),
        })
    }

    fn application(&self, caller: Principal) -> Result<(), Error> {
        if caller == self.application {
            Ok(())
        } else {
            Err(Error::Denied)
        }
    }

    fn reference_status(&self, id: ReferenceId, reference: Reference) -> ReferenceStatus {
        if let Some(upload) = reference.upload {
            match self.uploads[&upload].view.state {
                UploadState::Reserved => return ReferenceStatus::Reserved,
                UploadState::Cancelled | UploadState::Expired => return ReferenceStatus::Retired,
                UploadState::Registered | UploadState::Attached => {}
            }
        }
        match self.objects[&reference.root]
            .references
            .as_ref()
            .unwrap()
            .reference_state(id)
            .unwrap()
        {
            ReferenceState::Live => ReferenceStatus::Live,
            ReferenceState::Released => ReferenceStatus::Released,
        }
    }

    fn retire(&mut self, id: UploadId, state: UploadState) {
        let upload = self.uploads.get_mut(&id).unwrap();
        upload.view.state = state;
        let request = upload.view.request;
        self.objects.remove(&request.root);
        self.reserved_bytes -= request.bytes;
    }
}

fn valid_principal(principal: Principal) -> Result<(), Error> {
    if principal == Principal::anonymous() || principal == Principal::management_canister() {
        Err(Error::InvalidPrincipal)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
