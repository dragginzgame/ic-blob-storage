//! Bounded durable control state over a host-granted Storage lease.
//!
//! One Postcard record is rewritten per successful mutation (at most 64 uploads and
//! 64 references). This is an experimental finite profile, not a large-store design.
//! IC execution provides rollback: persistence/encoding failures must trap, never
//! be caught and converted into an ordinary reply after a mutation. Native callers
//! must discard this owner after a panic. No provider authority is implemented.

pub mod callbacks;
mod record;
#[cfg(test)]
mod tests;

#[cfg(feature = "metrics")]
use {
    crate::metrics::{CommandKind, Metrics, MetricsStorage},
    std::rc::Rc,
};

use ic_platform_types::Principal;
use ic_storage::{Memory, RuntimeMemory, adapter::StableCell};

use super::{Error, LocalModel, valid_principal};
use crate::{
    caffeine::Header,
    protocol::{
        Command, Inspection, Installation, Maintenance, ManagementView, Observation, Outcome,
    },
};
use record::{Record, STATE_LIMITS, STATE_VERSION};

/// Permanent allocation key contributed to the host's single Storage bootstrap.
pub const STORAGE_KEY: &str = "blob.control.v1";
/// Authority label for this library's allocation requirement.
pub const STORAGE_AUTHORITY: &str = "blob";
/// Hard pilot limit; released/cancelled identities continue occupying these slots.
pub const MAX_RECORDS: usize = 64;

///
/// OpenError
/// Cause of a refused creation or reopening. Hosts must preserve the allocation on failure.
///

#[derive(Debug)]
pub enum OpenError {
    /// Invalid authority or unsupported resource limits.
    Configuration,
    /// Collection admission failed before the state was published.
    Storage(ic_storage::StableCellPayloadError),
    /// Fresh creation was requested for an existing allocation.
    Occupied,
    /// Bounded record decoding failed.
    Codec(crate::storage_codec::Error),
    /// Wrong format, installation binding, or inconsistent domain records.
    Record,
}

#[cfg(feature = "logging")]
type LogRecorder = dyn Fn(ic_logging::Level, &'static str, &str);

///
/// Service
/// Owns the policy model, admission state and durable ledger. The memory lease
/// can outlive the host’s bootstrap runtime.
///

pub struct Service<M: Memory> {
    model: LocalModel,
    manager: Principal,
    installation: Principal,
    attempt: u64,
    cell: StableCell<Vec<u8>, M>,
    #[cfg(feature = "logging")]
    log: Option<Box<LogRecorder>>,
    #[cfg(feature = "metrics")]
    metrics: Option<Rc<MetricsStorage>>,
}

impl<M: Memory> Service<M> {
    /// Create only in an empty granted allocation, after checking the configuration.
    /// # Errors
    /// Refuses invalid configuration or an occupied allocation.
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
        let cell = create_cell(memory, bytes)?;

        Ok(Self {
            model,
            manager: input.manager,
            installation,
            attempt: 0,
            cell,
            #[cfg(feature = "logging")]
            log: None,
            #[cfg(feature = "metrics")]
            metrics: None,
        })
    }

    /// Load the finite record, validate it fully and reconstruct indexes/accounting.
    /// No new installation, history repair or provider retry is attempted.
    /// # Errors
    /// Refuses missing, malformed, oversized, inconsistent or foreign state.
    /// # Panics
    /// Traps on backing IO failure; callers must not substitute empty state.
    pub fn open(memory: RuntimeMemory<M>, installation: Principal) -> Result<Self, OpenError> {
        let cell = load_cell(memory, STATE_LIMITS)?;
        let record: Record =
            crate::storage_codec::decode(cell.get(), STATE_LIMITS, 3).map_err(OpenError::Codec)?;
        let manager = record.input.manager;
        let attempt = record.maintenance.attempt;
        let model = record.restore(installation)?;

        Ok(Self {
            model,
            manager,
            installation,
            attempt,
            cell,
            #[cfg(feature = "logging")]
            log: None,
            #[cfg(feature = "metrics")]
            metrics: None,
        })
    }

    /// Create a service that records commands into the host's heap store.
    /// # Errors
    /// Returns the same installation errors as [`Self::create`].
    #[cfg(feature = "metrics")]
    pub fn create_with_metrics(
        memory: RuntimeMemory<M>,
        installation: Principal,
        input: Installation,
        storage: Rc<MetricsStorage>,
    ) -> Result<Self, OpenError> {
        let mut service = Self::create(memory, installation, input)?;
        service.metrics = Some(storage);

        Ok(service)
    }

    /// Reopen the durable ledger and use the supplied heap store for subsequent commands.
    /// Supply a fresh store on upgrade to start a new observation window.
    /// # Errors
    /// Returns the same reopening errors as [`Self::open`].
    #[cfg(feature = "metrics")]
    pub fn open_with_metrics(
        memory: RuntimeMemory<M>,
        installation: Principal,
        storage: Rc<MetricsStorage>,
    ) -> Result<Self, OpenError> {
        let mut service = Self::open(memory, installation)?;
        service.metrics = Some(storage);

        Ok(service)
    }

    /// Snapshot the bound host storage. An uninstrumented service has no observations.
    /// The trusted host owns transport and access policy for this aggregate.
    #[cfg(feature = "metrics")]
    #[must_use]
    pub fn metrics(&self) -> Option<Metrics> {
        self.metrics.as_ref().map(|storage| storage.snapshot())
    }

    /// Send selected operational events to the host's single heap log.
    /// Bind after creation or reopening. Rebinding replaces the previous recorder.
    /// The host supplies time and retention and should handle append refusals locally.
    /// The recorder runs synchronously in the command's execution and must not trap.
    #[cfg(feature = "logging")]
    #[must_use]
    pub fn with_logging(
        mut self,
        record: impl Fn(ic_logging::Level, &'static str, &str) + 'static,
    ) -> Self {
        self.log = Some(Box::new(record));

        self
    }

    /// Apply one synchronous command and persist before returning its outcome.
    /// Caller and time must be independently captured by the host. Time uses the
    /// host's units (the standalone host uses IC nanoseconds).
    /// # Errors
    /// Ordinary refusals preserve the ledger and count as refused commands when observed.
    /// # Panics
    /// Persistence/encoding failure must roll back this IC execution; do not catch it.
    pub fn execute(
        &mut self,
        caller: Principal,
        command: Command,
        now: u64,
    ) -> Result<Outcome, Error> {
        #[cfg(feature = "metrics")]
        let kind = CommandKind::from(&command);
        #[cfg(feature = "metrics")]
        let previous_exposure = self.model.unresolved_bytes;

        #[cfg(feature = "logging")]
        let (previous_live_bytes, was_active) = (self.model.live_bytes, self.model.active);

        let result = self.apply(caller, command, now);

        #[cfg(feature = "logging")]
        if let Some(record) = &self.log {
            use ic_logging::Level::{Info, Warn};

            // Each registered object has positive declared length. Only first registration
            // adds live bytes, and only the final release subtracts them.
            let event = match &result {
                Ok(_) if self.model.live_bytes > previous_live_bytes => {
                    Some((Info, "object registered"))
                }
                Ok(_) if self.model.live_bytes < previous_live_bytes => {
                    Some((Info, "final reference released"))
                }
                Ok(Outcome::Maintenance(state)) if state.active != was_active => Some((
                    Info,
                    if state.active {
                        "admission reopened"
                    } else {
                        "admission drained"
                    },
                )),
                Err(Error::Capacity) => Some((Warn, "capacity limit reached")),
                Err(Error::Expired) => Some((Warn, "request deadline elapsed")),
                _ => None,
            };
            if let Some((level, message)) = event {
                record(level, "blob", message);
            }
        }

        #[cfg(feature = "metrics")]
        if let Some(metrics) = &self.metrics {
            // A successful registration adds exactly the admitted object's declared bytes.
            let registered_bytes = (kind == CommandKind::Register && result.is_ok())
                .then(|| self.model.unresolved_bytes - previous_exposure);
            metrics.record(kind, result.as_ref().err().copied(), registered_bytes);
        }

        result
    }

    fn apply(&mut self, caller: Principal, command: Command, now: u64) -> Result<Outcome, Error> {
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

fn create_cell<M: Memory>(
    memory: RuntimeMemory<M>,
    bytes: Vec<u8>,
) -> Result<StableCell<Vec<u8>, M>, OpenError> {
    if memory.size() != 0 {
        return Err(OpenError::Occupied);
    }

    Ok(StableCell::create(memory, bytes))
}

fn load_cell<M: Memory>(
    memory: RuntimeMemory<M>,
    maximum: usize,
) -> Result<StableCell<Vec<u8>, M>, OpenError> {
    ic_storage::stable_cell_payload_length(&memory, maximum).map_err(OpenError::Storage)?;

    StableCell::load(memory).map_err(OpenError::Storage)
}

fn validate(input: Installation) -> Result<(), OpenError> {
    valid_principal(input.application).map_err(|_| OpenError::Configuration)?;
    valid_principal(input.manager).map_err(|_| OpenError::Configuration)?;
    if input.limits.uploads.get() > MAX_RECORDS || input.limits.references.get() > MAX_RECORDS {
        return Err(OpenError::Configuration);
    }

    Ok(())
}
