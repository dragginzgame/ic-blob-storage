//! Bounded v1 records; all persisted transition arithmetic belongs to this model.
use super::{
    CandidType, Deserialize, GatewayScope, NonZeroU64, ReadChunkTarget, ReadSessionError,
    ReadSessionIntent, ReadSessionLimits, ReadSessionTicket, ReadSessionUsage, ReadTarget,
    UploadContext,
};
use candid::{DecoderConfig, Principal, decode_one_with_config};
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::binding::ObjectIdentity;
use ic_blob_storage_contracts::binding::ReferenceId;
use ic_blob_storage_contracts::binding::ReferenceKey;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::identity::ProviderRootHash;
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::{borrow::Cow, num::NonZeroU128};

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ReadJournalRecord {
    version: u8,
    service: Principal,
    operator: Principal,
    cashier: Principal,
    payer: Principal,
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
            service: b.service,
            operator: b.operator,
            cashier: config.billing().cashier(),
            payer: b.payment_account,
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
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ReadSessionRecord {
    version: u8,
    sequence: u64,
    service: Principal,
    tenant: Principal,
    cashier: Principal,
    namespace: u128,
    object: u128,
    incarnation: u128,
    reference: u128,
    root: [u8; 32],
    gateway: Principal,
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
            service: object.service(),
            tenant: object.tenant(),
            cashier: intent.scope.cashier(),
            namespace: id.namespace.get(),
            object: id.object.get(),
            incarnation: id.incarnation.get(),
            reference: target.reference.reference().get().get(),
            root: *target.root.as_bytes(),
            gateway: target.gateway,
            index: intent.chunk.index,
            gateway_generation: intent.gateway_generation,
            tenant_generation: intent.tenant_generation.get(),
        }
    }
    pub(crate) fn ticket(&self) -> Option<ReadSessionTicket> {
        if self.version != 1
            || self.sequence == 0
            || self.gateway == Principal::anonymous()
            || self.gateway == Principal::management_canister()
        {
            return None;
        }
        let namespace = NonZeroU128::new(self.namespace)?;
        let object = ObjectBinding::new(
            self.service,
            self.tenant,
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
                    service: self.service,
                    actor: self.tenant,
                },
                scope: GatewayScope::new(self.service, namespace, self.cashier).ok()?,
                chunk: ReadChunkTarget {
                    target: ReadTarget {
                        root: ProviderRootHash::try_from(self.root.as_slice()).ok()?,
                        reference: ReferenceKey::new(
                            object,
                            ReferenceId::new(NonZeroU128::new(self.reference)?),
                        ),
                        gateway: self.gateway,
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
    ($ty:ty, $max:expr) => {
        impl Storable for $ty {
            fn to_bytes(&self) -> Cow<'_, [u8]> {
                Cow::Owned(self.clone().into_bytes())
            }
            fn into_bytes(self) -> Vec<u8> {
                let bytes = candid::encode_one(self).expect("read record encoding");
                assert!(bytes.len() <= $max, "read record bound");
                bytes
            }
            fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
                assert!(bytes.len() <= $max, "read record bound");
                let mut config = DecoderConfig::new();
                config
                    .set_decoding_quota(100_000)
                    .set_skipping_quota(1000)
                    .set_max_type_len(32)
                    .set_max_header_len($max)
                    .set_full_error_message(false);
                decode_one_with_config(&bytes, &config).expect("valid same-release read record")
            }
            const BOUND: Bound = Bound::Bounded {
                max_size: $max as u32,
                is_fixed_size: false,
            };
        }
    };
}
storable!(ReadJournalRecord, 4096);
storable!(ReadSessionRecord, 2048);
storable!(ReadUsageRecord, 128);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_session_identity_is_retained_and_exhaustion_never_blocks_release() {
        let principal = Principal::from_slice(&[1; 29]);
        let mut record = ReadJournalRecord {
            version: 1,
            service: principal,
            operator: principal,
            cashier: principal,
            payer: principal,
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
