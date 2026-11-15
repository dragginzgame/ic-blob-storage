//! Experimental, in-memory control model for one application and candidate binding.
//!
//! This is the sole local owner of grants, root matching, reference bindings and
//! declared-byte accounting. Registered objects use the existing reference owner.
//! Every admitted root keeps its original upload lifetime, even after cancellation.
//! Roots identify objects only within this provisional model; no provider deletion
//! scope is qualified. No certificate, provider message or settlement is produced.
//!
//! Hosts must supply independently captured callers and clock readings, never
//! caller claims from request fields. These synchronous operations perform no
//! external awaits. Ordinary errors precede mutation. This heap owner has no
//! standalone upgrade, serialization, cloning or reset contract. The bounded
//! [`persistence::Service`] owns its durable representation and maintenance gate.
//! Dropping a bare model loses history; it MUST NOT back real upload authority.

pub mod persistence;
mod types;
pub use types::{
    Error, Limits, ObjectView, ReferenceStatus, ReferenceView, Root, UploadId, UploadRequest,
    UploadState, UploadView, Usage,
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
    // Permanent root-to-lifetime binding. Its upload owns declaration and disposition.
    upload: UploadId,
    references: Option<ObjectReferences>,
}

/// Bounded local rehearsal of the service rules, with no provider effects.
pub struct LocalModel {
    application: Principal,
    limits: Limits,
    uploads: BTreeMap<UploadId, Upload>,
    objects: BTreeMap<Root, Object>,
    references: BTreeMap<ReferenceId, Root>,
    reserved_bytes: u64,
    live_bytes: u64,
    unresolved_bytes: u64,
    active: bool,
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
            active: true,
        })
    }

    /// Reserve a new object, or attach a use to a matching Open object.
    ///
    /// Exact request replay returns its current historical disposition without
    /// renewing deadlines, references or byte usage. A replay does not revalidate
    /// the supplied manifest: its original declaration was already admitted.
    /// New declarations must pass manifest preflight before any mutation.
    /// Cancelled/expired roots cannot start another lifetime, even for a new uploader.
    /// Retained upload IDs are resolved before expiry checks: `Expired` from this
    /// operation means the ID was never admitted and its requested deadline has passed.
    /// Preventing later admission of that exact request requires monotonic `now`.
    /// # Errors
    /// Rejects wrong authority, conflicting identities, expired/invalid grants,
    /// invalid manifests, competing reservations, retired roots, Closing objects or capacity.
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
        self.admit()?;
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
            if self.uploads[&object.upload].view.request.bytes != request.bytes {
                return Err(Error::Conflict);
            }
            let references = self.registered(request.root)?;
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
                    upload: request.id,
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
        self.references
            .insert(request.first_reference, request.root);
        Ok(view)
    }

    /// Resolve a root to its original grant and consume it in one local transition.
    ///
    /// Creates the known first reference and transfers reserved bytes to live and
    /// unresolved declared usage together. Returns no upload authority. Repeating
    /// registration is refused. Additional uses never replace the root's original
    /// uploader, deadline or first reference. This accepts an already decoded root;
    /// it defines neither a provider wire format nor a certificate endpoint.
    /// # Errors
    /// Rejects unknown roots, wrong uploaders, consumed/retired or expired grants and a
    /// clock earlier than admission, without releasing their reservations.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn register(&mut self, caller: Principal, root: Root, now: u64) -> Result<(), Error> {
        let id = self.objects.get(&root).ok_or(Error::UnknownObject)?.upload;
        let upload = &self.uploads[&id];
        let request = upload.view.request;
        if caller != request.uploader {
            return Err(Error::Denied);
        }
        self.admit()?;
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
    /// Its root and names remain occupied. Repeating expiry of that grant succeeds.
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
    /// objects, retired roots, Closing objects or exhausted lifetime reference capacity.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn retain(&mut self, caller: Principal, root: Root, id: ReferenceId) -> Result<(), Error> {
        self.application(caller)?;
        if let Some(target) = self.references.get(&id).copied() {
            if target != root {
                return Err(Error::Conflict);
            }
            return match self.reference_status(id, root) {
                ReferenceStatus::Live | ReferenceStatus::Released => Ok(()),
                ReferenceStatus::Reserved | ReferenceStatus::Retired => {
                    Err(Error::ReferenceUnavailable)
                }
            };
        }
        self.admit()?;
        if self.references.len() >= self.limits.references.get() {
            return Err(Error::Capacity);
        }
        self.registered(root)?;
        let object = self.objects.get_mut(&root).unwrap();
        object
            .references
            .as_mut()
            .unwrap()
            .retain(id)
            .map_err(Error::Reference)?;
        self.references.insert(id, root);
        Ok(())
    }

    /// Release a reference, or cancel its still-unconsumed first-reference grant.
    ///
    /// Final live release removes logical usage but preserves unresolved usage.
    /// Cancellation frees reserved bytes but permanently retires the root.
    /// Repeated release succeeds without changing accounting or reusing names.
    /// No new slot is needed, including at capacity.
    /// # Errors
    /// Rejects wrong authority or an unknown reference. Refusals do not mutate state.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn release(&mut self, caller: Principal, id: ReferenceId) -> Result<(), Error> {
        self.application(caller)?;
        let root = *self.references.get(&id).ok_or(Error::UnknownReference)?;
        match self.reference_status(id, root) {
            ReferenceStatus::Reserved => {
                self.retire(self.objects[&root].upload, UploadState::Cancelled)
            }
            ReferenceStatus::Live => {
                let object = self.objects.get_mut(&root).unwrap();
                let references = object.references.as_mut().unwrap();
                references.release(id).map_err(Error::Reference)?;
                if references.retention_state() == RetentionState::Closing {
                    self.live_bytes -= self.uploads[&object.upload].view.request.bytes;
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

    /// Inspect the immutable root and current disposition as the application authority.
    /// Released/retired references preserve their target; inspection never reopens them.
    /// # Errors
    /// Rejects wrong authority or an unknown reference.
    pub fn inspect_reference(
        &self,
        caller: Principal,
        id: ReferenceId,
    ) -> Result<ReferenceView, Error> {
        self.application(caller)?;
        let root = *self.references.get(&id).ok_or(Error::UnknownReference)?;
        Ok(ReferenceView {
            root,
            status: self.reference_status(id, root),
        })
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

    /// Inspect a known root's original grant, local retention and live-use count.
    ///
    /// One synchronous observation uses existing records without scanning history
    /// or allocating slots. It remains available at capacity and after retirement.
    /// Registered history persists when retention becomes Closing; no settlement
    /// or provider availability is inferred. It does not authorize a later effect.
    /// # Errors
    /// Rejects wrong application authority or a root that was never admitted.
    /// # Panics
    /// Panics if private record invariants are violated by an implementation defect.
    pub fn inspect_object(&self, caller: Principal, root: Root) -> Result<ObjectView, Error> {
        self.application(caller)?;
        let object = self.objects.get(&root).ok_or(Error::UnknownObject)?;
        Ok(ObjectView {
            original_upload: self.uploads[&object.upload].view,
            retention: object
                .references
                .as_ref()
                .map(ObjectReferences::retention_state),
            live_references: object
                .references
                .as_ref()
                .map_or(0, ObjectReferences::live_count),
        })
    }

    fn application(&self, caller: Principal) -> Result<(), Error> {
        if caller == self.application {
            Ok(())
        } else {
            Err(Error::Denied)
        }
    }

    fn admit(&self) -> Result<(), Error> {
        if self.active {
            Ok(())
        } else {
            Err(Error::Maintenance)
        }
    }

    fn registered(&self, root: Root) -> Result<&ObjectReferences, Error> {
        let object = self.objects.get(&root).ok_or(Error::UnknownObject)?;
        match self.uploads[&object.upload].view.state {
            UploadState::Reserved => Err(Error::ObjectReserved),
            UploadState::Cancelled | UploadState::Expired => Err(Error::RootRetired),
            UploadState::Registered => Ok(object.references.as_ref().unwrap()),
            UploadState::Attached => unreachable!("an attachment cannot own the root"),
        }
    }

    fn reference_status(&self, id: ReferenceId, root: Root) -> ReferenceStatus {
        let references = match self.registered(root) {
            Ok(references) => references,
            Err(Error::ObjectReserved) => return ReferenceStatus::Reserved,
            Err(Error::RootRetired) => return ReferenceStatus::Retired,
            _ => unreachable!("admitted references have an object owner"),
        };
        match references.reference_state(id).unwrap() {
            ReferenceState::Live => ReferenceStatus::Live,
            ReferenceState::Released => ReferenceStatus::Released,
        }
    }

    fn retire(&mut self, id: UploadId, state: UploadState) {
        let upload = self.uploads.get_mut(&id).unwrap();
        upload.view.state = state;
        let request = upload.view.request;
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
