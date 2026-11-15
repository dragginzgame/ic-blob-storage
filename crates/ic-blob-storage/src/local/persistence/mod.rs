//! Bounded durable control state over a host-granted Storage lease.
//!
//! One Wire record is rewritten per successful mutation (at most 64 uploads and
//! 64 references). This is an experimental finite profile, not a large-store design.
//! IC execution provides rollback: persistence/encoding failures must trap, never
//! be caught and converted into an ordinary reply after a mutation. Native callers
//! must discard this owner after a panic. No provider authority is implemented.

mod record;

use ic_platform_types::Principal;
use ic_storage::{Memory, RuntimeMemory, adapter::StableCell};

use super::{Error, LocalModel, valid_principal};
use crate::caffeine::Header;
use crate::protocol::{
    Command, Inspection, Installation, Maintenance, ManagementView, Observation, Outcome,
};
use record::{Record, STATE_LIMITS, STATE_VERSION};

/// Permanent allocation key contributed to the host's single Storage bootstrap.
pub const STORAGE_KEY: &str = "blob.control.v1";
/// Authority label for this library's allocation requirement.
pub const STORAGE_AUTHORITY: &str = "blob";
/// Hard pilot limit; released/cancelled identities continue occupying these slots.
pub const MAX_RECORDS: usize = 64;

/// Refused creation or reopening. Never repair or initialize on a load failure.
#[derive(Debug)]
pub enum OpenError {
    /// Invalid authority or unsupported resource limits.
    Configuration,
    /// Collection admission failed before the state was published.
    Storage(ic_storage::adapter::CollectionError),
    /// Bounded record decoding failed.
    Codec(ic_wire::Error),
    /// Wrong format, installation binding, or inconsistent domain records.
    Record,
}

/// The sole library policy model and its durable backing; hosts own neither a second
/// ledger nor admission flags. A lease may outlive the host's bootstrap runtime.
pub struct Service<M: Memory> {
    model: LocalModel,
    manager: Principal,
    installation: Principal,
    attempt: u64,
    cell: StableCell<Vec<u8>, M>,
}

impl<M: Memory> Service<M> {
    /// Create only in an empty granted allocation, after checking the configuration.
    /// # Errors
    /// Refuses invalid configuration, nonempty storage or initial growth refusal.
    /// # Panics
    /// Traps on backing IO failure. The IC must roll back the entire message.
    pub fn create(
        memory: RuntimeMemory<M>,
        installation: Principal,
        input: Installation,
    ) -> Result<Self, OpenError> {
        validate(input)?;
        valid_principal(installation).map_err(|_| OpenError::Configuration)?;
        let model = LocalModel::new(input.application, input.limits)
            .map_err(|_| OpenError::Configuration)?;
        let bytes = Record::capture(&model, input.manager, installation, 0).encode()?;
        let cell = StableCell::create(memory, bytes).map_err(OpenError::Storage)?;
        Ok(Self {
            model,
            manager: input.manager,
            installation,
            attempt: 0,
            cell,
        })
    }

    /// Load the finite record, validate it fully and reconstruct indexes/accounting.
    /// No new installation, history repair or provider retry is attempted.
    /// # Errors
    /// Refuses missing, malformed, oversized, inconsistent or foreign state.
    /// # Panics
    /// Traps on backing IO failure; callers must not substitute empty state.
    pub fn open(memory: RuntimeMemory<M>, installation: Principal) -> Result<Self, OpenError> {
        let cell = StableCell::<Vec<u8>, M>::load(memory, STATE_LIMITS.bytes)
            .map_err(OpenError::Storage)?;
        let record: Record =
            ic_wire::native::decode(cell.get(), STATE_LIMITS).map_err(OpenError::Codec)?;
        let manager = record.input.manager;
        let attempt = record.maintenance.attempt;
        let model = record.restore(installation)?;
        Ok(Self {
            model,
            manager,
            installation,
            attempt,
            cell,
        })
    }

    /// Apply one synchronous command and persist before returning its outcome.
    /// Caller and time must be independently captured by the host. Time uses the
    /// host's units (the standalone host uses IC nanoseconds).
    /// # Errors
    /// Ordinary domain and authorization refusals leave both owners unchanged.
    /// # Panics
    /// Persistence/encoding failure must roll back this IC execution; do not catch it.
    pub fn execute(
        &mut self,
        caller: Principal,
        command: Command,
        now: u64,
    ) -> Result<Outcome, Error> {
        let outcome = match command {
            Command::Authorize {
                request,
                chunks,
                headers,
            } => {
                let headers: Vec<_> = headers
                    .iter()
                    .map(|(name, value)| Header { name, value })
                    .collect();
                Outcome::Upload(
                    self.model
                        .authorize(caller, request, &chunks, &headers, now)?,
                )
            }
            Command::Register(root) => {
                self.model.register(caller, root, now)?;
                Outcome::Done
            }
            Command::Retain(root, id) => {
                self.model.retain(caller, root, id)?;
                Outcome::Done
            }
            Command::Release(id) => {
                self.model.release(caller, id)?;
                Outcome::Done
            }
            Command::Expire(id) => {
                self.model.expire(caller, id, now)?;
                Outcome::Done
            }
            Command::Drain(attempt) => {
                self.manager(caller)?;
                if attempt == 0 {
                    return Err(Error::MaintenanceAttempt);
                }
                if attempt != self.attempt {
                    if self.attempt.checked_add(1) != Some(attempt) {
                        return Err(Error::MaintenanceAttempt);
                    }
                    self.attempt = attempt;
                    self.model.active = false;
                }
                Outcome::Maintenance(self.maintenance())
            }
            Command::Activate(attempt) => {
                self.manager(caller)?;
                if attempt == 0 || attempt != self.attempt {
                    return Err(Error::MaintenanceAttempt);
                }
                self.model.active = true;
                Outcome::Maintenance(self.maintenance())
            }
        };
        let bytes = Record::capture(&self.model, self.manager, self.installation, self.attempt)
            .encode()
            .expect("bounded Blob state encoding");
        self.cell.set(bytes);
        Ok(outcome)
    }

    /// Inspect existing state under ordinary authorization, including while drained.
    /// # Errors
    /// Refuses unauthorized actors and unknown domain identities.
    pub fn inspect(&self, caller: Principal, request: Inspection) -> Result<Observation, Error> {
        match request {
            Inspection::Upload(id) => self
                .model
                .inspect_upload(caller, id)
                .map(Observation::Upload),
            Inspection::Reference(id) => self
                .model
                .inspect_reference(caller, id)
                .map(Observation::Reference),
            Inspection::Object(root) => self
                .model
                .inspect_object(caller, root)
                .map(Observation::Object),
            Inspection::Usage => self.model.usage(caller).map(Observation::Usage),
            Inspection::Overview(uploader) => self
                .model
                .overview(caller, uploader)
                .map(Observation::Overview),
            Inspection::Maintenance => {
                self.manager(caller)?;
                Ok(Observation::Maintenance(self.maintenance()))
            }
        }
    }

    /// Inspect the installation's identity and configuration without writing storage.
    /// Hosts must capture the caller and expose this through replicated execution
    /// when the result will inform management decisions. Available while drained.
    /// # Errors
    /// Refuses any caller other than the configured manager.
    pub fn inspect_management(&self, caller: Principal) -> Result<ManagementView, Error> {
        self.manager(caller)?;
        Ok(ManagementView {
            installation: self.installation,
            configuration: Installation {
                application: self.model.application,
                manager: self.manager,
                limits: self.model.limits,
            },
            state_version: STATE_VERSION,
            maintenance: self.maintenance(),
        })
    }

    /// Host lifecycle check: true only after an explicit maintenance attempt closed admission.
    #[must_use]
    pub fn is_drained(&self) -> bool {
        self.attempt != 0 && !self.model.active
    }

    fn manager(&self, caller: Principal) -> Result<(), Error> {
        if caller == self.manager {
            Ok(())
        } else {
            Err(Error::Denied)
        }
    }
    fn maintenance(&self) -> Maintenance {
        Maintenance {
            attempt: self.attempt,
            active: self.model.active,
        }
    }
}

fn validate(input: Installation) -> Result<(), OpenError> {
    valid_principal(input.application).map_err(|_| OpenError::Configuration)?;
    valid_principal(input.manager).map_err(|_| OpenError::Configuration)?;
    if input.limits.uploads.get() > MAX_RECORDS || input.limits.references.get() > MAX_RECORDS {
        return Err(OpenError::Configuration);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
