//! Experimental tree-root callbacks over the existing Service, for local qualification.
//!
//! One immutable gateway binding and bounded acknowledgment history use a second
//! host-granted native root. Service remains the sole reference/accounting owner.
//! This candidate profile does not establish provider key scope, authority expiry,
//! physical deletion or billing closure. No endpoints are exported by this module.

use super::access::Access;
use super::{MAX_RECORDS, OpenError, Service};
use crate::{
    local::{Error, Root, UploadId, valid_principal},
    protocol::{Command, Outcome},
    references::RetentionState,
};
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

/// Maximum roots in one callback, including duplicate positions.
pub const MAX_ROOTS: usize = MAX_RECORDS;

const LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 16_384,
    allocation: 262_144,
    nodes: 16_384,
    depth: 16,
};

#[derive(Deserialize, Serialize)]
struct Record {
    version: u32,
    installation: Principal,
    gateway: Principal,
    // Sorted unique roots, each bound to its original registered upload lifetime.
    acknowledged: Vec<(Root, UploadId)>,
}

/// Refused callback; ordinary errors occur before any receipt mutation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackError {
    /// Actual caller is not the immutable configured gateway.
    Denied,
    /// Root count exceeds the finite callback bound.
    Capacity,
    /// At least one root is unknown, reserved, retired or still referenced.
    NotClosing,
}

/// Composes a Service with provider receipts without copying retention or usage state.
/// Hosts capture callers, mount the selected external ABI and grant both allocations.
pub struct Callbacks {
    service: Service,
    record: Record,
    access: Access,
}

impl Callbacks {
    /// Bind a fresh receipt allocation to this service and one explicit gateway.
    /// # Errors
    /// Rejects special/self gateway principals or nonempty/unavailable receipt storage.
    /// # Panics
    /// Backing IO failure must trap the entire IC execution, including Service creation.
    pub fn create(service: Service, access: Access, gateway: Principal) -> Result<Self, OpenError> {
        valid_principal(gateway).map_err(|_| OpenError::Configuration)?;
        if gateway == service.installation {
            return Err(OpenError::Configuration);
        }
        let record = Record {
            version: 1,
            installation: service.installation,
            gateway,
            acknowledged: vec![],
        };
        let bytes = ic_wire::encode(&record, ic_wire::Encoding::Payload, LIMITS)
            .map_err(OpenError::Codec)?;
        access
            .update(|mut view, root| {
                if !view
                    .read()
                    .is_empty(root)
                    .map_err(ic_storage::retained::Error::from)?
                {
                    return Err(ic_storage::retained::Error::State);
                }
                view.insert(root, b"control", &bytes)
                    .map_err(ic_storage::retained::Error::from)?;
                Ok(())
            })
            .map_err(OpenError::Storage)?
            .map_err(OpenError::Storage)?;
        Ok(Self {
            service,
            record,
            access,
        })
    }

    /// Reopen the original binding and validate every receipt against Service history.
    /// Missing state never creates a new binding or forgets acknowledgment history.
    /// # Errors
    /// Rejects invalid storage, format, gateway, owner or non-Closing lifetime receipts.
    /// # Panics
    /// Backing IO failure must trap the entire IC execution.
    pub fn open(service: Service, access: Access) -> Result<Self, OpenError> {
        let bytes = access
            .read(|view, root| {
                view.get(root, b"control")
                    .map(|value| value.map(|v| v.as_slice().to_vec()))
            })
            .map_err(OpenError::Storage)?
            .map_err(ic_storage::retained::Error::from)
            .map_err(OpenError::Storage)?
            .ok_or(OpenError::Record)?;
        let record: Record = ic_wire::decode(&bytes, ic_wire::Encoding::Payload, LIMITS)
            .map_err(OpenError::Codec)?;
        valid_principal(record.gateway).map_err(|_| OpenError::Record)?;
        if record.version != 1
            || record.installation != service.installation
            || record.gateway == service.installation
            || record.acknowledged.len() > MAX_RECORDS
            || record
                .acknowledged
                .windows(2)
                .any(|pair| pair[0].0 >= pair[1].0)
            || record
                .acknowledged
                .iter()
                .any(|&(root, id)| closing(&service, root) != Some(id))
        {
            return Err(OpenError::Record);
        }
        Ok(Self {
            service,
            record,
            access,
        })
    }

    /// Borrow the existing owner for ordinary authorized inspection and lifecycle checks.
    #[must_use]
    pub const fn service(&self) -> &Service {
        &self.service
    }

    /// Execute an ordinary command through the existing Service's caller and accounting rules.
    /// # Errors
    /// Preserves Service's command refusals without changing callback receipts.
    /// # Panics
    /// Persistence failure must trap the whole message, as with Service directly.
    pub fn execute(
        &mut self,
        caller: Principal,
        command: Command,
        now: u64,
    ) -> Result<Outcome, Error> {
        self.service.execute(caller, command, now)
    }

    /// Public, positional retention projection. Unknown/reserved/retired roots stay protected.
    /// False identifies only a known Closing lifetime, not physical deletion or closure.
    /// # Errors
    /// Rejects excess positions before projection. Hosts validate 32-byte keys first.
    pub fn liveness(&self, roots: &[Root]) -> Result<Vec<bool>, CallbackError> {
        bounded(roots)?;
        Ok(roots
            .iter()
            .map(|&root| closing(&self.service, root).is_none())
            .collect())
    }

    /// List unacknowledged Closing roots in stable key order for the configured gateway.
    /// Maintenance still permits observation and cleanup; no upload authority is issued.
    /// # Errors
    /// Rejects non-gateway callers, including operators/controllers, even for an empty list.
    pub fn candidates(&self, caller: Principal) -> Result<Vec<Root>, CallbackError> {
        self.gateway(caller)?;
        Ok(self
            .service
            .model
            .objects
            .keys()
            .copied()
            .filter(|root| {
                closing(&self.service, *root).is_some()
                    && self
                        .record
                        .acknowledged
                        .binary_search_by_key(root, |entry| entry.0)
                        .is_err()
            })
            .collect())
    }

    /// Retain an authenticated callback for exact Closing lifetimes, without settling exposure.
    /// Validate the whole batch before writes. Duplicate/lost-reply replay is idempotent;
    /// acknowledgment consumes no new upload/reference capacity and never erases history.
    /// # Errors
    /// Rejects wrong caller, excess positions or any root without a Closing lifetime.
    /// # Panics
    /// Encoding/storage failure must trap this IC message, never become an ordinary error.
    pub fn acknowledge(&mut self, caller: Principal, roots: &[Root]) -> Result<(), CallbackError> {
        self.gateway(caller)?;
        bounded(roots)?;
        let receipts: Vec<_> = roots
            .iter()
            .map(|&root| {
                closing(&self.service, root)
                    .map(|id| (root, id))
                    .ok_or(CallbackError::NotClosing)
            })
            .collect::<Result<_, _>>()?;
        self.record.acknowledged.extend(receipts);
        self.record.acknowledged.sort_unstable();
        self.record.acknowledged.dedup();
        let bytes = ic_wire::encode(&self.record, ic_wire::Encoding::Payload, LIMITS)
            .expect("bounded callback receipts");
        self.access
            .update(|mut view, root| {
                view.insert(root, b"control", &bytes)
                    .expect("native receipt write")
            })
            .expect("native receipt scope");
        Ok(())
    }

    fn gateway(&self, caller: Principal) -> Result<(), CallbackError> {
        if caller == self.record.gateway {
            Ok(())
        } else {
            Err(CallbackError::Denied)
        }
    }
}

fn bounded(roots: &[Root]) -> Result<(), CallbackError> {
    if roots.len() > MAX_ROOTS {
        Err(CallbackError::Capacity)
    } else {
        Ok(())
    }
}

fn closing(service: &Service, root: Root) -> Option<UploadId> {
    let view = service
        .model
        .inspect_object(service.model.application, root)
        .ok()?;
    (view.retention == Some(RetentionState::Closing)).then_some(view.original_upload.request.id)
}

#[cfg(test)]
mod tests;

/// Declare the callback receipt root alongside the service root.
#[must_use]
pub const fn storage_root(id: u64, participant: u64) -> ic_storage::retained::Root {
    ic_storage::retained::Root {
        id,
        participant,
        schema: 0x424c4f4243423031,
        entries: 1,
        key_bytes: 7,
        value_bytes: 16_384,
    }
}
