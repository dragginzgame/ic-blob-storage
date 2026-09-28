//! Incremental durable read occupancy; no transport, timeout eviction or unfencing.
pub mod access;
pub mod download;
pub mod transport;
use crate::model::{
    gateway::registry::GatewayScope,
    service::{
        configuration::ServiceConfiguration,
        read::session::{
            ReadChunkTarget, ReadSessionError, ReadSessionIntent, ReadSessionLimits,
            ReadSessionTicket, ReadSessionUsage,
            record::{ReadJournalRecord, ReadSessionRecord, ReadUsageRecord},
        },
        upload::UploadContext,
    },
};
use candid::Principal;
use ic_memory::ic_stable_structures::{BTreeMap, Memory};
use std::{collections::BTreeMap as HeapMap, num::NonZeroU32, ops::Bound};

/// Three distinct memories, exclusively granted by the host; no implicit registration.
pub struct ReadSessionMemories<M: Memory> {
    /// Configuration, high-water identity and aggregate occupancy.
    pub journal: M,
    /// One bounded intent per occupied session, removed only by exact completion.
    pub sessions: M,
    /// Counters for currently occupied tenants; empty rows are removed.
    pub tenants: M,
}
/// Operator accounting; neither these counters nor the same old backup prove freshness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadSessionSummary {
    /// Highest identity ever allocated; never reset or reused after completion.
    pub last_sequence: u64,
    /// All occupied sessions and reserved response buffers.
    pub usage: ReadSessionUsage,
    /// Restored owners permanently refuse admission and completion.
    pub fenced: bool,
}
/// Passive exact occupied-session inspection; cannot be used as a completion ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadSessionView {
    /// Local identity for bounded operator traversal.
    pub sequence: u64,
    /// Original tenant's exact chunk/reference/gateway.
    pub chunk: ReadChunkTarget,
}
/// Bounded active read records and maintained global/per-tenant counters.
/// Mutations must run synchronously in an IC update with traps propagated for
/// atomic rollback. Native memory is not a transaction substitute. Dropped tickets,
/// revocation and elapsed time retain capacity; only the exact callback completes.
/// The host must use one exclusive owner and enforce installation identity.
pub struct StableReadSessions<M: Memory> {
    journal: BTreeMap<u8, ReadJournalRecord, M>,
    sessions: BTreeMap<u64, ReadSessionRecord, M>,
    tenants: BTreeMap<Principal, ReadUsageRecord, M>,
    config: ServiceConfiguration,
    limits: ReadSessionLimits,
    fenced: bool,
}
impl<M: Memory> StableReadSessions<M> {
    /// Initialize fresh exclusive memories under explicit service and read limits.
    /// # Errors
    /// Rejects invalid limits or any already allocated memory.
    /// # Panics
    /// Aliased grants and stable failures trap; the host must propagate rollback.
    pub fn install(
        memory: ReadSessionMemories<M>,
        config: ServiceConfiguration,
        limits: ReadSessionLimits,
    ) -> Result<Self, ReadSessionError> {
        limits.validate()?;
        if memory.journal.size() != 0 || memory.sessions.size() != 0 || memory.tenants.size() != 0 {
            return Err(ReadSessionError::AlreadyAllocated);
        }
        let mut journal = BTreeMap::new(memory.journal);
        assert_eq!(memory.sessions.size(), 0, "distinct read memories");
        let sessions = BTreeMap::new(memory.sessions);
        assert_eq!(memory.tenants.size(), 0, "distinct read memories");
        let tenants = BTreeMap::new(memory.tenants);
        journal.insert(0, ReadJournalRecord::new(&config, limits));
        Ok(Self {
            journal,
            sessions,
            tenants,
            config,
            limits,
            fenced: false,
        })
    }
    /// Load and validate bounded occupancy without repair, reset or releasing slots.
    /// Restored owners remain inspection-only even with an original host ticket.
    /// # Errors
    /// Rejects missing stores, changed bindings/limits, orphaned rows or bad totals.
    /// # Panics
    /// Invalid binary records or collection headers trap.
    pub fn open(
        memory: ReadSessionMemories<M>,
        config: ServiceConfiguration,
        limits: ReadSessionLimits,
    ) -> Result<Self, ReadSessionError> {
        limits.validate()?;
        if memory.journal.size() == 0 || memory.sessions.size() == 0 || memory.tenants.size() == 0 {
            return Err(ReadSessionError::Missing);
        }
        let store = Self {
            journal: BTreeMap::load(memory.journal),
            sessions: BTreeMap::load(memory.sessions),
            tenants: BTreeMap::load(memory.tenants),
            config,
            limits,
            fenced: true,
        };
        let recorded = store.record()?;
        let mut total = ReadUsageRecord::empty();
        let mut tenants = HeapMap::<Principal, ReadUsageRecord>::new();
        for entry in store.sessions.iter() {
            let ticket = entry
                .value()
                .ticket()
                .ok_or(ReadSessionError::InvalidRecord)?;
            if *entry.key() != ticket.sequence || ticket.sequence > recorded.last() {
                return Err(ReadSessionError::InvalidRecord);
            }
            store.intent_binding(&ticket.intent)?;
            total = total.reserve(limits.sessions, limits.bytes, limits.reply_bytes)?;
            let own = tenants
                .entry(ticket.intent.context.actor)
                .or_insert(ReadUsageRecord::empty());
            *own = own.reserve(
                limits.tenant_sessions,
                limits.tenant_bytes,
                limits.reply_bytes,
            )?;
        }
        if total != recorded.usage() || tenants.len() as u64 != store.tenants.len() {
            return Err(ReadSessionError::InvalidRecord);
        }
        for (tenant, usage) in tenants {
            if store.tenants.get(&tenant) != Some(usage) {
                return Err(ReadSessionError::InvalidRecord);
            }
        }
        Ok(store)
    }
    /// Inspect aggregate retained occupancy as the configured operator.
    /// # Errors
    /// Rejects wrong actual service/caller or corrupt metadata.
    pub fn inspect(&self, context: UploadContext) -> Result<ReadSessionSummary, ReadSessionError> {
        self.operator(context)?;
        let record = self.record()?;
        Ok(ReadSessionSummary {
            last_sequence: record.last(),
            usage: record.usage().view(),
            fenced: self.fenced,
        })
    }
    /// Inspect at most 64 active records after a local identity, including when fenced.
    /// Released records disappear; monotonic identities keep later sessions distinct.
    /// # Errors
    /// Rejects caller/binding, excessive page size or corrupt retained records.
    pub fn inspect_active(
        &self,
        context: UploadContext,
        after: u64,
        limit: NonZeroU32,
    ) -> Result<Vec<ReadSessionView>, ReadSessionError> {
        self.operator(context)?;
        self.record()?;
        if limit.get() > 64 {
            return Err(ReadSessionError::Limits);
        }
        self.sessions
            .range((Bound::Excluded(after), Bound::Unbounded))
            .take(limit.get() as usize)
            .map(|entry| {
                let ticket = entry
                    .value()
                    .ticket()
                    .ok_or(ReadSessionError::InvalidRecord)?;
                Ok(ReadSessionView {
                    sequence: *entry.key(),
                    chunk: ticket.chunk(),
                })
            })
            .collect()
    }
    pub(crate) fn bind(&self, expected: &ServiceConfiguration) -> Result<(), ReadSessionError> {
        if self.config != *expected {
            return Err(ReadSessionError::Binding);
        }
        if self.fenced {
            return Err(ReadSessionError::Fenced);
        }
        Ok(())
    }
    pub(crate) fn limits(&self) -> ReadSessionLimits {
        self.limits
    }
    pub(crate) fn reserve(
        &mut self,
        intent: &ReadSessionIntent,
    ) -> Result<ReadSessionTicket, ReadSessionError> {
        self.bind(&self.config)?;
        self.intent_binding(intent)?;
        let mut journal = self.record()?;
        let sequence = journal.reserve()?;
        let own = self
            .tenants
            .get(&intent.context.actor)
            .unwrap_or(ReadUsageRecord::empty())
            .reserve(
                self.limits.tenant_sessions,
                self.limits.tenant_bytes,
                self.limits.reply_bytes,
            )?;
        let ticket = ReadSessionTicket {
            sequence,
            intent: *intent,
        };
        self.sessions
            .insert(sequence, ReadSessionRecord::new(sequence, intent));
        self.tenants.insert(intent.context.actor, own);
        self.journal.insert(0, journal);
        Ok(ticket)
    }
    pub(crate) fn check(
        &self,
        context: UploadContext,
        ticket: &ReadSessionTicket,
    ) -> Result<(), ReadSessionError> {
        self.bind(&self.config)?;
        if context != ticket.intent.context {
            return Err(ReadSessionError::Denied);
        }
        self.intent_binding(&ticket.intent)?;
        self.record()?;
        let recorded = self
            .sessions
            .get(&ticket.sequence)
            .ok_or(ReadSessionError::Stale)?
            .ticket()
            .ok_or(ReadSessionError::InvalidRecord)?;
        if recorded != *ticket {
            return Err(ReadSessionError::Stale);
        }
        Ok(())
    }
    pub(crate) fn complete(
        &mut self,
        context: UploadContext,
        ticket: &ReadSessionTicket,
    ) -> Result<(), ReadSessionError> {
        self.check(context, ticket)?;
        let mut journal = self.record()?;
        journal.release()?;
        let own = self
            .tenants
            .get(&context.actor)
            .ok_or(ReadSessionError::InvalidRecord)?
            .release(self.limits.reply_bytes)?;
        self.sessions.remove(&ticket.sequence);
        if own == ReadUsageRecord::empty() {
            self.tenants.remove(&context.actor);
        } else {
            self.tenants.insert(context.actor, own);
        }
        self.journal.insert(0, journal);
        Ok(())
    }
    fn record(&self) -> Result<ReadJournalRecord, ReadSessionError> {
        if self.journal.len() != 1 {
            return Err(ReadSessionError::InvalidRecord);
        }
        let record = self
            .journal
            .get(&0)
            .ok_or(ReadSessionError::InvalidRecord)?;
        if !record.matches(&self.config, self.limits) {
            return Err(ReadSessionError::Binding);
        }
        let usage = record.usage().view();
        if !record.valid_usage()
            || self.sessions.len() != u64::from(usage.sessions)
            || self.tenants.len() > self.sessions.len()
        {
            return Err(ReadSessionError::InvalidRecord);
        }
        Ok(record)
    }
    fn operator(&self, context: UploadContext) -> Result<(), ReadSessionError> {
        if context.service != self.config.bindings().service {
            return Err(ReadSessionError::Binding);
        }
        if context.actor != self.config.bindings().operator {
            return Err(ReadSessionError::Denied);
        }
        Ok(())
    }
    fn intent_binding(&self, intent: &ReadSessionIntent) -> Result<(), ReadSessionError> {
        let b = self.config.bindings();
        let object = intent.chunk.target.reference.object();
        let expected = GatewayScope::new(b.service, b.namespace, self.config.billing().cashier())
            .expect("validated scope");
        if intent.scope != expected
            || intent.context.service != b.service
            || object.service() != b.service
            || object.identity().namespace != b.namespace
        {
            return Err(ReadSessionError::Binding);
        }
        if intent.context.actor != object.tenant() {
            return Err(ReadSessionError::Denied);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests;
