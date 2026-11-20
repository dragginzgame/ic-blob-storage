//! Heap observations of completed service commands for a host metrics adapter.
//! Durable byte gauges remain available through the authorized usage inspection.

use crate::{local::Error, protocol::Command};
use ic_metrics::{MeasurementCounter, MeasurementHistogram};
use std::cell::RefCell;

///
/// CommandKind
/// The commands accepted by the synchronous service boundary.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandKind {
    /// Admit an upload or attach an application reference.
    Authorize,
    /// Register an admitted object.
    Register,
    /// Acquire an application reference.
    Retain,
    /// Release a reference or cancel a reservation.
    Release,
    /// Retire an expired grant.
    Expire,
    /// Close admission.
    Drain,
    /// Reopen admission.
    Activate,
}

impl From<&Command> for CommandKind {
    fn from(command: &Command) -> Self {
        match command {
            Command::Authorize { .. } => Self::Authorize,
            Command::Register(_) => Self::Register,
            Command::Retain(..) => Self::Retain,
            Command::Release(_) => Self::Release,
            Command::Expire(_) => Self::Expire,
            Command::Drain(_) => Self::Drain,
            Command::Activate(_) => Self::Activate,
        }
    }
}

///
/// CommandCounts
/// Disjoint outcome counts for one command kind. A committed retry counts as another call.
///

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CommandCounts {
    /// Successful returns, including historical successes.
    pub succeeded: MeasurementCounter,
    /// Caller lacked authority.
    pub denied: MeasurementCounter,
    /// Service capacity refused the request.
    pub capacity: MeasurementCounter,
    /// The grant expired before registration.
    pub expired: MeasurementCounter,
    /// All remaining ordinary refusals.
    pub other_refusals: MeasurementCounter,
}

///
/// Metrics
/// One service incarnation's command outcomes and distribution of declared object sizes.
///

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metrics {
    commands: [CommandCounts; CommandKind::Activate as usize + 1],
    /// One byte-length observation per successful first registration. Inclusive
    /// upper bounds are 64 KiB, 1 MiB (one Caffeine chunk), 16 MiB and 256 MiB.
    pub registered_bytes: MeasurementHistogram<4>,
}

impl Metrics {
    /// Read one command's outcome counts, including exact-zero and overflow flags.
    #[must_use]
    pub fn commands(&self, kind: CommandKind) -> CommandCounts {
        self.commands[kind as usize]
    }
}

///
/// MetricsStorage
/// Host-owned heap store for one Blob service. Only Blob records observations.
///

pub struct MetricsStorage(RefCell<Metrics>);

impl MetricsStorage {
    /// Copy current observations without recording or consuming them.
    #[must_use]
    pub fn snapshot(&self) -> Metrics {
        self.0.borrow().clone()
    }

    pub(crate) fn record(
        &self,
        kind: CommandKind,
        refusal: Option<Error>,
        registered_bytes: Option<u64>,
    ) {
        let mut metrics = self.0.borrow_mut();
        let counts = &mut metrics.commands[kind as usize];
        let counter = match refusal {
            None => &mut counts.succeeded,
            Some(Error::Denied) => &mut counts.denied,
            Some(Error::Capacity) => &mut counts.capacity,
            Some(Error::Expired) => &mut counts.expired,
            Some(_) => &mut counts.other_refusals,
        };
        counter.add(1);
        if let Some(bytes) = registered_bytes {
            metrics.registered_bytes.record(bytes);
        }
    }
}

impl Default for MetricsStorage {
    fn default() -> Self {
        const EMPTY: MeasurementHistogram<4> =
            match MeasurementHistogram::new([64 << 10, 1 << 20, 16 << 20, 256 << 20]) {
                Ok(histogram) => histogram,
                Err(_) => panic!("fixed byte bounds must increase"),
            };

        Self(RefCell::new(Metrics {
            commands: Default::default(),
            registered_bytes: EMPTY,
        }))
    }
}
