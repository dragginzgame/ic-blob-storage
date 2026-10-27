//! Bounded v1 records; all persisted transition arithmetic belongs to this model.
use super::{
    Deserialize, GatewayScope, NonZeroU64, ReadChunkTarget, ReadSessionError, ReadSessionIntent,
    ReadSessionLimits, ReadSessionTicket, ReadSessionUsage, ReadTarget, Serialize, UploadContext,
};
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_platform::Principal;
use ic_platform::principal::ExternalPrincipal;
#[cfg(test)]
use ic_storage::adapter::Storable;
use std::num::NonZeroU128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReadUsageRecord {
    count: u32,
    bytes: u64,
}
impl ReadUsageRecord {
    pub(crate) const fn empty() -> Self {
        Self { count: 0, bytes: 0 }
    }
    pub(crate) fn reserve(
        self,
        count: u32,
        bytes: u64,
        reply: u32,
    ) -> Result<Self, ReadSessionError> {
        let next = Self {
            count: self
                .count
                .checked_add(1)
                .ok_or(ReadSessionError::Capacity)?,
            bytes: self
                .bytes
                .checked_add(u64::from(reply))
                .ok_or(ReadSessionError::Capacity)?,
        };
        if next.count > count || next.bytes > bytes {
            return Err(ReadSessionError::Capacity);
        }
        Ok(next)
    }
    pub(crate) fn release(self, reply: u32) -> Result<Self, ReadSessionError> {
        Ok(Self {
            count: self
                .count
                .checked_sub(1)
                .ok_or(ReadSessionError::InvalidRecord)?,
            bytes: self
                .bytes
                .checked_sub(u64::from(reply))
                .ok_or(ReadSessionError::InvalidRecord)?,
        })
    }
    pub(crate) const fn view(self) -> ReadSessionUsage {
        ReadSessionUsage {
            sessions: self.count,
            reserved_bytes: self.bytes,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReadJournalRecord {
    version: u8,
    service: ExternalPrincipal,
    operator: ExternalPrincipal,
    cashier: ExternalPrincipal,
    payer: ExternalPrincipal,
    namespace: u128,
    limits: ReadSessionLimits,
    last: u64,
    usage: ReadUsageRecord,
}
impl ReadJournalRecord {
    pub(crate) fn new(config: &ServiceConfiguration, limits: ReadSessionLimits) -> Self {
        let b = config.bindings();
        Self {
            version: 1,
            service: b.service.into(),
            operator: b.operator.into(),
            cashier: config.billing().cashier().into(),
            payer: b.payment_account.into(),
            namespace: b.namespace.get(),
            limits,
            last: 0,
            usage: ReadUsageRecord::empty(),
        }
    }
    pub(crate) fn matches(&self, config: &ServiceConfiguration, limits: ReadSessionLimits) -> bool {
        let mut expected = Self::new(config, limits);
        expected.last = self.last;
        expected.usage = self.usage;
        *self == expected
    }
    pub(crate) fn reserve(&mut self) -> Result<u64, ReadSessionError> {
        let sequence = self
            .last
            .checked_add(1)
            .ok_or(ReadSessionError::Exhausted)?;
        let usage = self.usage.reserve(
            self.limits.sessions,
            self.limits.bytes,
            self.limits.reply_bytes,
        )?;
        self.last = sequence;
        self.usage = usage;
        Ok(sequence)
    }
    pub(crate) fn release(&mut self) -> Result<(), ReadSessionError> {
        self.usage = self.usage.release(self.limits.reply_bytes)?;
        Ok(())
    }
    pub(crate) const fn last(&self) -> u64 {
        self.last
    }
    pub(crate) const fn usage(&self) -> ReadUsageRecord {
        self.usage
    }
    pub(crate) fn valid_usage(&self) -> bool {
        let usage = self.usage.view();
        usage.sessions <= self.limits.sessions
            && usage.reserved_bytes
                == u64::from(usage.sessions) * u64::from(self.limits.reply_bytes)
            && usage.reserved_bytes <= self.limits.bytes
            && u64::from(usage.sessions) <= self.last
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReadSessionRecord {
    version: u8,
    sequence: u64,
    service: ExternalPrincipal,
    tenant: ExternalPrincipal,
    cashier: ExternalPrincipal,
    namespace: u128,
    object: u128,
    incarnation: u128,
    reference: u128,
    root: [u8; 32],
    gateway: ExternalPrincipal,
    index: u64,
    gateway_generation: u64,
    tenant_generation: u64,
}
impl ReadSessionRecord {
    pub(crate) fn new(sequence: u64, intent: &ReadSessionIntent) -> Self {
        let target = intent.chunk.target;
        let object = target.reference.object();
        let id = object.identity();
        Self {
            version: 1,
            sequence,
            service: object.service().into(),
            tenant: object.tenant().into(),
            cashier: intent.scope.cashier().into(),
            namespace: id.namespace.get(),
            object: id.object.get(),
            incarnation: id.incarnation.get(),
            reference: target.reference.reference().get().get(),
            root: *target.root.as_bytes(),
            gateway: target.gateway.into(),
            index: intent.chunk.index,
            gateway_generation: intent.gateway_generation,
            tenant_generation: intent.tenant_generation.get(),
        }
    }
    pub(crate) fn ticket(&self) -> Option<ReadSessionTicket> {
        if self.version != 1
            || self.sequence == 0
            || self.gateway == Principal::anonymous().into()
            || self.gateway == Principal::management_canister().into()
        {
            return None;
        }
        let namespace = NonZeroU128::new(self.namespace)?;
        let object = ObjectBinding::new(
            self.service.into(),
            self.tenant.into(),
            ObjectIdentity {
                namespace,
                object: NonZeroU128::new(self.object)?,
                incarnation: NonZeroU128::new(self.incarnation)?,
            },
        )
        .ok()?;
        Some(ReadSessionTicket {
            sequence: self.sequence,
            intent: ReadSessionIntent {
                context: UploadContext {
                    service: self.service.into(),
                    actor: self.tenant.into(),
                },
                scope: GatewayScope::new(self.service.into(), namespace, self.cashier.into())
                    .ok()?,
                chunk: ReadChunkTarget {
                    target: ReadTarget {
                        root: ProviderRootHash::try_from(self.root.as_slice()).ok()?,
                        reference: ReferenceKey::new(
                            object,
                            ReferenceId::new(NonZeroU128::new(self.reference)?),
                        ),
                        gateway: self.gateway.into(),
                    },
                    index: self.index,
                },
                gateway_generation: self.gateway_generation,
                tenant_generation: NonZeroU64::new(self.tenant_generation)?,
            },
        })
    }
}
macro_rules! storable {
    ($record:ty, $id:expr, $max:expr) => {
        crate::model::record_codec::storable!(
            $record, id: $id, bytes: $max, layout: ic_storage::adapter::storable::Bound::Bounded { max_size: $max as u32, is_fixed_size: false },
        );
    };
}
storable!(ReadJournalRecord, b"BLREADJ1", 4096);
storable!(ReadSessionRecord, b"BLREADS1", 2048);
storable!(ReadUsageRecord, b"BLREADU1", 128);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_session_identity_is_retained_and_exhaustion_never_blocks_release() {
        let principal = Principal::from_slice(&[1; 29]);
        let mut record = ReadJournalRecord {
            version: 1,
            service: principal.into(),
            operator: principal.into(),
            cashier: principal.into(),
            payer: principal.into(),
            namespace: u128::MAX,
            limits: ReadSessionLimits::new(
                2.try_into().unwrap(),
                2.try_into().unwrap(),
                64.try_into().unwrap(),
                128.try_into().unwrap(),
                128.try_into().unwrap(),
            )
            .unwrap(),
            last: u64::MAX - 1,
            usage: ReadUsageRecord::empty(),
        };
        assert_eq!(record.reserve(), Ok(u64::MAX));
        let before = record.clone();
        assert_eq!(record.reserve(), Err(ReadSessionError::Exhausted));
        assert_eq!(record, before);
        assert_eq!(ReadJournalRecord::from_bytes(record.to_bytes()), record);
        record.release().unwrap();
        assert_eq!(record.last(), u64::MAX);
        assert_eq!(record.usage(), ReadUsageRecord::empty());
        assert_eq!(record.reserve(), Err(ReadSessionError::Exhausted));
        assert!(record.valid_usage());
    }
}
