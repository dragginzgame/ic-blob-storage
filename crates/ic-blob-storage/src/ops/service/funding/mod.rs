//! Incremental durable funding bookkeeping with explicit host credit evidence.
pub mod access;
mod admission;
pub mod assessment;
pub mod client;
pub mod credit;
pub mod history;
pub mod outcome;
pub mod renewal;
pub mod reply;
pub mod summary;
use crate::model::{
    billing::{
        allocation::{FundingAllocation, FundingAllocationError, FundingAllocationView},
        journal::{
            FundingIntent, FundingIntentAdmission, FundingIntentError, FundingIntentView,
            FundingTransportContext, FundingTransportOutcome,
            record::{FundingAccountingRecord, FundingIntentRecord, FundingJournalRecord},
        },
    },
    service::{configuration::ServiceConfiguration, upload::UploadContext},
};
use crate::ops::caffeine::funding::request::{CashierTopUpRequest, TopUpRequestError};
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
use thiserror::Error;

/// Two distinct, exclusively owned memories granted by the integrating host.
pub struct FundingMemories<M: Memory> {
    /// Configuration/accounting and bounded receipt index; one authoritative owner.
    pub accounting: M,
    /// Bounded lifetime exact intent/outcome history, keyed by operation identity.
    pub intents: M,
}
/// Durable sequential attachment journal. It never sends cycles or infers credit.
///
/// Hosts must apply mutations synchronously in an IC update and propagate traps
/// for whole-message rollback. Native memory has no such transaction guarantee.
/// These local reservations do not establish spendability, provider authority,
/// account activity, execution-fee coverage or safe dispatch. Hosts must separately
/// qualify all of those gates before using any attempted operation for an effect.
/// Reopening validates complete bounded history and fences mutation until the
/// installation consumes an independent current-instance continuity proof.
/// This must be the sole journal for its service/account allocation. The host still
/// owns callback authentication and complete account activity. Canonical Cashier
/// method/argument binding is local evidence, not provider deployment qualification.
pub struct StableFundingJournal<M: Memory> {
    accounting: BTreeMap<[u8; 32], FundingAccountingRecord, M>,
    intents: BTreeMap<u128, FundingIntentRecord, M>,
    config: ServiceConfiguration,
    fenced: bool,
}
impl<M: Memory> StableFundingJournal<M> {
    pub(crate) fn set_recovery_fence(&mut self, fenced: bool) {
        self.fenced = fenced;
    }
    pub(crate) const fn inspection_configuration(&self) -> &ServiceConfiguration {
        &self.config
    }
    /// Install a fresh journal with an explicit attachment allocation and reserve.
    /// # Errors
    /// Rejects allocated memory and history limits that cannot fit 32-bit Wasm.
    /// # Panics
    /// Aliased memories and stable writes trap; the host must propagate IC rollback.
    pub fn install(
        memory: FundingMemories<M>,
        config: ServiceConfiguration,
        allocation: FundingAllocation,
    ) -> Result<Self, FundingJournalError> {
        let record = FundingJournalRecord::new(&config, allocation);
        validate_envelope(&config, allocation)?;
        if memory.accounting.size() != 0 || memory.intents.size() != 0 {
            return Err(FundingJournalError::AlreadyAllocated);
        }
        let mut accounting = BTreeMap::new(memory.accounting);
        assert_eq!(memory.intents.size(), 0, "distinct funding memories");
        let intents = BTreeMap::new(memory.intents);
        accounting.insert([0; 32], FundingAccountingRecord::Totals(record));
        Ok(Self {
            accounting,
            intents,
            config,
            fenced: false,
        })
    }
    /// Load retained state without initialization, repair, migration or unfencing.
    /// Checks service/operator/Cashier/account/namespace and allocation, plus every
    /// exact intent and recomputed total. Host installation/release checks are separate.
    /// # Errors
    /// Rejects missing stores, changed bindings, invalid history or accounting.
    /// # Panics
    /// Corrupt binary records/collection headers trap; hosts must reject restoration.
    pub fn open(
        memory: FundingMemories<M>,
        config: ServiceConfiguration,
        allocation: FundingAllocation,
    ) -> Result<Self, FundingJournalError> {
        let mut reconstructed = FundingJournalRecord::new(&config, allocation);
        validate_envelope(&config, allocation)?;
        if memory.accounting.size() == 0 || memory.intents.size() == 0 {
            return Err(FundingJournalError::Missing);
        }
        let store = Self {
            accounting: BTreeMap::load(memory.accounting),
            intents: BTreeMap::load(memory.intents),
            config,
            fenced: true,
        };
        let recorded = store.totals()?;
        if !recorded.matches(&config, allocation) {
            return Err(FundingJournalError::Binding);
        }
        if store.intents.len() > recorded.limit() || store.intents.len() != recorded.count() {
            return Err(FundingJournalError::InvalidRecord);
        }
        let mut receipts = 0_u64;
        for entry in store.intents.iter() {
            let record = entry.value();
            let view = record.view().ok_or(FundingJournalError::InvalidRecord)?;
            if *entry.key() != view.intent.operation.get() || !recorded.scope_matches(view.intent) {
                return Err(FundingJournalError::InvalidRecord);
            }
            reconstructed = reconstructed.reserve(view.intent)?;
            reconstructed = reconstructed.resolve(
                record
                    .transfer()
                    .ok_or(FundingJournalError::InvalidRecord)?,
            )?;
            if let Some(credit) = record.credit() {
                if store.accounting.get(&credit.receipt_digest())
                    != Some(FundingAccountingRecord::Receipt {
                        operation: view.intent.operation.get(),
                    })
                {
                    return Err(FundingJournalError::InvalidRecord);
                }
                receipts += 1;
                reconstructed = reconstructed.confirm_credit(credit.accepted_cycles())?;
            }
            if let Some(additional) = record.renewed_allocation() {
                reconstructed = reconstructed.renew(additional)?;
            }
        }
        if reconstructed != recorded || store.accounting.len() != receipts + 1 {
            return Err(FundingJournalError::InvalidRecord);
        }
        Ok(store)
    }
    /// Reserve the full offer once; exact replay only reports retained progress.
    /// New externally supplied operation IDs must increase; no local counter proves
    /// freshness after restore. Pending/uncertain attempts block later reservations.
    /// # Errors
    /// Rejects authority, scope, fence, changed/stale identity, capacity or allocation.
    /// # Panics
    /// Stable writes trap and must roll back the complete IC update.
    pub fn prepare(
        &mut self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<FundingIntentAdmission, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        let totals = self.totals()?;
        Self::scope(&totals, input)?;
        Self::encode_request(input)?;
        if let Some(record) = self.exact(input)? {
            return Ok(FundingIntentAdmission::Existing(
                record
                    .view()
                    .ok_or(FundingJournalError::InvalidRecord)?
                    .state,
            ));
        }
        if input.operation.get() <= totals.last() {
            return Err(FundingJournalError::StaleIdentity);
        }
        let next = totals.reserve(input)?;
        self.intents
            .insert(input.operation.get(), FundingIntentRecord::new(input));
        self.write_totals(&next);
        Ok(FundingIntentAdmission::Created)
    }
    /// Persist possible dispatch before the host's separately authorized effect.
    /// A second call always rejects, including after a lost acknowledgment. This
    /// marker alone is not spending permission; no expiry or retry path exists.
    /// # Errors
    /// Rejects authority, scope, fence, absent/changed intent or any prior attempt.
    /// # Panics
    /// Stable failure must propagate for whole-update rollback.
    pub fn mark_attempted(
        &mut self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<CashierTopUpRequest, FundingJournalError> {
        self.authorize(execution)?;
        self.mutable()?;
        Self::scope(&self.totals()?, input)?;
        let record = self.required(input)?.attempted()?;
        let request = Self::encode_request(input)?;
        self.intents.insert(input.operation.get(), record);
        Ok(request)
    }
    /// Retain exact host-authenticated transport evidence and release only its return.
    ///
    /// The host must correlate the complete intent and actual callback target before
    /// calling this, and capture an unbounded refund before another await. This API
    /// authenticates the operator context, not the provider. Never expose it as a
    /// public provider-fact endpoint. Timeouts/lost replies leave the intent uncertain.
    /// Exact terminal replay is unchanged; conflicting evidence rejects.
    /// # Errors
    /// Rejects authority/scope/fence, absent/changed identity, premature/conflicting
    /// evidence, invalid refund or overflowing lifetime return totals.
    /// # Panics
    /// Stable failures must roll back both intent and accounting writes on the IC.
    pub fn record_transport(
        &mut self,
        execution: UploadContext,
        input: FundingIntent,
        source: FundingTransportContext,
        outcome: FundingTransportOutcome,
    ) -> Result<bool, FundingJournalError> {
        self.complete_transport(execution, input, source, outcome, None)
    }
    /// Inspect an exact intent as the operator, including under the restore fence.
    /// Absence is not proof of no external effect and cannot authorize payment.
    /// # Errors
    /// Rejects authority, scope, changed identity or invalid records.
    pub fn lookup(
        &self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<Option<FundingIntentView>, FundingJournalError> {
        self.authorize(execution)?;
        Self::scope(&self.totals()?, input)?;
        self.exact(input)?
            .map(|r| r.view().ok_or(FundingJournalError::InvalidRecord))
            .transpose()
    }
    /// Read local allocation accounting, never provider credit or spendable cycles.
    /// # Errors
    /// Rejects caller/service authority or a missing accounting row.
    pub fn allocation(
        &self,
        execution: UploadContext,
    ) -> Result<FundingAllocationView, FundingJournalError> {
        self.authorize(execution)?;
        Ok(self.totals()?.view())
    }
    /// Inspect the canonical request retained by an exact intent, including while fenced.
    /// This binds method, explicit account, optional target and attachment, without
    /// sending a call or granting retry authority. The provider has no operation-ID
    /// argument; service/namespace/operation correlation remains local.
    /// # Errors
    /// Rejects authority, scope, absent/changed identity, record or encoding failures.
    pub fn request(
        &self,
        execution: UploadContext,
        input: FundingIntent,
    ) -> Result<CashierTopUpRequest, FundingJournalError> {
        self.authorize(execution)?;
        Self::scope(&self.totals()?, input)?;
        let original = self
            .required(input)?
            .view()
            .ok_or(FundingJournalError::InvalidRecord)?
            .intent;
        Self::encode_request(original)
    }
    fn encode_request(input: FundingIntent) -> Result<CashierTopUpRequest, FundingJournalError> {
        Ok(CashierTopUpRequest::new(
            input.cashier,
            input.account,
            input.offered,
            input.target_balance,
        )?)
    }
    /// Reopened owners always remain inspection-only.
    #[must_use]
    pub const fn is_fenced(&self) -> bool {
        self.fenced
    }
    fn authorize(&self, execution: UploadContext) -> Result<(), FundingJournalError> {
        if execution.service != self.config.bindings().service {
            return Err(FundingJournalError::Binding);
        }
        if execution.actor != self.config.bindings().operator {
            return Err(FundingJournalError::NotOperator);
        }
        Ok(())
    }
    fn scope(
        totals: &FundingJournalRecord,
        input: FundingIntent,
    ) -> Result<(), FundingJournalError> {
        if totals.scope_matches(input) {
            Ok(())
        } else {
            Err(FundingJournalError::Binding)
        }
    }
    fn mutable(&self) -> Result<(), FundingJournalError> {
        if self.fenced {
            Err(FundingJournalError::Fenced)
        } else {
            Ok(())
        }
    }
    fn totals(&self) -> Result<FundingJournalRecord, FundingJournalError> {
        self.accounting
            .get(&[0; 32])
            .and_then(FundingAccountingRecord::totals)
            .ok_or(FundingJournalError::InvalidRecord)
    }
    fn write_totals(&mut self, totals: &FundingJournalRecord) {
        self.accounting
            .insert([0; 32], FundingAccountingRecord::Totals(*totals));
    }
    fn exact(
        &self,
        input: FundingIntent,
    ) -> Result<Option<FundingIntentRecord>, FundingJournalError> {
        let Some(record) = self.intents.get(&input.operation.get()) else {
            return Ok(None);
        };
        if record
            .view()
            .ok_or(FundingJournalError::InvalidRecord)?
            .intent
            != input
        {
            return Err(FundingIntentError::Conflict.into());
        }
        Ok(Some(record))
    }
    fn required(&self, input: FundingIntent) -> Result<FundingIntentRecord, FundingJournalError> {
        self.exact(input)?.ok_or(FundingJournalError::Unknown)
    }
}
/// Typed journal rejection before any write; binary/storage failures trap.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum FundingJournalError {
    /// Original credit, exact grant identity or chronology prevents budget renewal.
    #[error(transparent)]
    Renewal(#[from] crate::model::billing::journal::renewal::FundingRenewalError),
    /// Invalid or conflicting host credit evidence.
    #[error(transparent)]
    Credit(#[from] crate::model::billing::journal::credit::FundingCreditError),
    /// Fresh installation cannot overwrite allocated memory.
    #[error("funding memory allocated")]
    AlreadyAllocated,
    /// Restoration cannot initialize absent memory.
    #[error("funding memory missing")]
    Missing,
    /// Service, operator, provider/account scope or allocation changed.
    #[error("funding binding mismatch")]
    Binding,
    /// Trusted transport context does not match the original service/target.
    #[error("funding transport binding mismatch")]
    TransportBinding,
    /// The actual caller is not the configured operator.
    #[error("funding operator required")]
    NotOperator,
    /// Configuration cannot fit the supported Wasm index width.
    #[error("funding history bound exceeds Wasm width")]
    UnsupportedEnvelope,
    /// Retained rows, identities or totals disagree.
    #[error("invalid funding journal")]
    InvalidRecord,
    /// Restored instances cannot mutate or dispatch.
    #[error("funding journal fenced")]
    Fenced,
    /// A fresh intent must sort after all retained operation identities.
    #[error("funding identity precedes retained history")]
    StaleIdentity,
    /// Exact intent is absent.
    #[error("unknown funding intent")]
    Unknown,
    /// Invalid exact transition.
    #[error(transparent)]
    Intent(#[from] FundingIntentError),
    /// Allocation, pending attempt or history limit prevents the operation.
    #[error(transparent)]
    Allocation(#[from] FundingAllocationError),
    /// Canonical provider request encoding failed before mutation.
    #[error(transparent)]
    Request(#[from] TopUpRequestError),
}
#[cfg(test)]
mod tests;

pub(crate) fn validate_envelope(
    config: &ServiceConfiguration,
    allocation: FundingAllocation,
) -> Result<(), FundingJournalError> {
    if FundingJournalRecord::new(config, allocation).limit() > u64::from(u32::MAX) {
        Err(FundingJournalError::UnsupportedEnvelope)
    } else {
        Ok(())
    }
}
