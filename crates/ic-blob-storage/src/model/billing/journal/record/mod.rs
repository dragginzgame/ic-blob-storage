//! Bounded v1 journal schemas and transitions; no storage or transport effects.
use ic_platform::principal::ExternalPrincipal;
pub(crate) mod response;
use super::credit::{FundingCreditConfirmation, FundingCreditError, FundingCreditReceipt};
use super::renewal::{FundingBudgetRenewal, FundingRenewalError};
use super::{
    FundingIntent, FundingIntentError, FundingIntentState, FundingIntentView,
    FundingTransportOutcome,
};
use crate::model::billing::allocation::FundingAllocation;
use crate::model::billing::allocation::FundingAllocationError;
use crate::model::billing::allocation::FundingAllocationView;
use crate::model::billing::allocation::record::FundingAllocationRecord;
use ic_blob_storage_contracts::configuration::service::ServiceConfiguration;
use ic_blob_storage_contracts::funding::transfer::FundingTransfer;
use ic_platform::candid::contract::CandidType;
use ic_platform::candid::contract::serde::Deserialize;
use response::FundingResponseRecord;
use std::num::NonZeroU128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) enum FundingPhaseRecord {
    Prepared,
    Uncertain,
    NotEnqueued,
    Callback { refunded: u128 },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
enum FundingCreditRecord {
    Required,
    Confirmed {
        accepted: u128,
        receipt_digest: [u8; 32],
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
enum FundingMethodRecord {
    AccountTopUpV1,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) struct FundingIntentRecord {
    version: u8,
    service: ExternalPrincipal,
    cashier: ExternalPrincipal,
    account: ExternalPrincipal,
    namespace: u128,
    operation: u128,
    offered: u128,
    method: FundingMethodRecord,
    target_balance: Option<u128>,
    phase: FundingPhaseRecord,
    response: FundingResponseRecord,
    credit: FundingCreditRecord,
    renewed_allocation: u128,
}
impl FundingIntentRecord {
    pub(crate) fn new(input: FundingIntent) -> Self {
        Self {
            version: 1,
            service: input.service.into(),
            cashier: input.cashier.into(),
            account: input.account.into(),
            namespace: input.namespace.get(),
            operation: input.operation.get(),
            offered: input.offered.get(),
            method: FundingMethodRecord::AccountTopUpV1,
            target_balance: input.target_balance.map(NonZeroU128::get),
            phase: FundingPhaseRecord::Prepared,
            response: FundingResponseRecord::Missing,
            credit: FundingCreditRecord::Required,
            renewed_allocation: 0,
        }
    }
    pub(crate) fn view(self) -> Option<FundingIntentView> {
        if self.version != 1 || !self.response.matches(self.phase) {
            return None;
        }
        let intent = FundingIntent {
            service: self.service.into(),
            cashier: self.cashier.into(),
            account: self.account.into(),
            namespace: NonZeroU128::new(self.namespace)?,
            operation: NonZeroU128::new(self.operation)?,
            offered: NonZeroU128::new(self.offered)?,
            target_balance: match self.target_balance {
                Some(value) => Some(NonZeroU128::new(value)?),
                None => None,
            },
        };
        let state = match self.phase {
            FundingPhaseRecord::Prepared => FundingIntentState::Prepared,
            FundingPhaseRecord::Uncertain => FundingIntentState::Uncertain,
            FundingPhaseRecord::NotEnqueued => FundingIntentState::NotEnqueued,
            FundingPhaseRecord::Callback { refunded } => {
                FundingTransfer::unbounded_callback(intent.offered, refunded).ok()?;
                FundingIntentState::Callback { refunded }
            }
        };
        if let FundingCreditRecord::Confirmed {
            accepted,
            receipt_digest,
        } = self.credit
        {
            let FundingIntentState::Callback { refunded } = state else {
                return None;
            };
            if accepted == 0
                || accepted != intent.offered.get() - refunded
                || receipt_digest == [0; 32]
            {
                return None;
            }
        }
        if self.renewed_allocation != 0
            && self
                .reconciled_amount()
                .is_none_or(|accepted| self.renewed_allocation > accepted.get())
        {
            return None;
        }
        Some(FundingIntentView { intent, state })
    }
    pub(crate) fn attempted(mut self) -> Result<Self, FundingIntentError> {
        if self.phase != FundingPhaseRecord::Prepared {
            return Err(FundingIntentError::AlreadyAttempted);
        }
        self.phase = FundingPhaseRecord::Uncertain;
        Ok(self)
    }
    pub(crate) fn complete(
        mut self,
        outcome: FundingTransportOutcome,
    ) -> Result<Self, FundingIntentError> {
        let phase = match outcome {
            FundingTransportOutcome::NotEnqueued => FundingPhaseRecord::NotEnqueued,
            FundingTransportOutcome::Callback { refunded } => {
                FundingTransfer::unbounded_callback(
                    NonZeroU128::new(self.offered).expect("validated offer"),
                    refunded,
                )?;
                FundingPhaseRecord::Callback { refunded }
            }
        };
        match self.phase {
            FundingPhaseRecord::Prepared => return Err(FundingIntentError::NotAttempted),
            FundingPhaseRecord::Uncertain => {}
            prior if prior == phase => return Ok(self),
            _ => return Err(FundingIntentError::OutcomeConflict),
        }
        self.phase = phase;
        Ok(self)
    }
    pub(crate) fn transfer(self) -> Option<FundingTransfer> {
        let view = self.view()?;
        Some(match view.state {
            FundingIntentState::Prepared | FundingIntentState::Uncertain => {
                FundingTransfer::unknown(view.intent.offered)
            }
            FundingIntentState::NotEnqueued => FundingTransfer::not_enqueued(view.intent.offered),
            FundingIntentState::Callback { refunded } => {
                FundingTransfer::unbounded_callback(view.intent.offered, refunded).ok()?
            }
        })
    }
    /// Matching successful callback, independent of the reported balance amount.
    /// A separately confirmed credit takes precedence so accounting is disjoint.
    pub(crate) fn acknowledged(self) -> Option<NonZeroU128> {
        if self.credit != FundingCreditRecord::Required
            || !matches!(self.response, FundingResponseRecord::Balance { .. })
        {
            return None;
        }
        let FundingPhaseRecord::Callback { refunded } = self.phase else {
            return None;
        };
        self.offered
            .checked_sub(refunded)
            .and_then(NonZeroU128::new)
    }
    fn reconciled_amount(self) -> Option<NonZeroU128> {
        self.credit()
            .map(FundingCreditReceipt::accepted_cycles)
            .or_else(|| self.acknowledged())
    }
    pub(crate) fn credit(self) -> Option<FundingCreditReceipt> {
        match self.credit {
            FundingCreditRecord::Required => None,
            FundingCreditRecord::Confirmed {
                accepted,
                receipt_digest,
            } => Some(FundingCreditReceipt {
                accepted_cycles: NonZeroU128::new(accepted).expect("validated credit receipt"),
                receipt_digest,
            }),
        }
    }
    pub(crate) fn confirm_credit(
        mut self,
        confirmation: FundingCreditConfirmation,
    ) -> Result<Self, FundingCreditError> {
        if self.acknowledged().is_some() {
            return Err(FundingCreditError::ProviderAcknowledged);
        }
        let accepted = self
            .transfer()
            .and_then(FundingTransfer::accepted)
            .and_then(NonZeroU128::new)
            .ok_or(FundingCreditError::AcceptanceRequired)?;
        if accepted != confirmation.accepted_cycles {
            return Err(FundingCreditError::AmountMismatch);
        }
        if confirmation.receipt_digest == [0; 32] {
            return Err(FundingCreditError::EmptyReceipt);
        }
        let next = FundingCreditRecord::Confirmed {
            accepted: accepted.get(),
            receipt_digest: confirmation.receipt_digest,
        };
        if self.credit != FundingCreditRecord::Required && self.credit != next {
            return Err(FundingCreditError::Conflict);
        }
        self.credit = next;
        Ok(self)
    }
    pub(crate) const fn renewed_allocation(self) -> Option<NonZeroU128> {
        NonZeroU128::new(self.renewed_allocation)
    }
    pub(crate) fn renew(
        mut self,
        input: FundingBudgetRenewal,
    ) -> Result<Self, FundingRenewalError> {
        let accepted = self
            .reconciled_amount()
            .ok_or(FundingRenewalError::CreditRequired)?;
        if input.additional.get() > accepted.get() {
            return Err(FundingRenewalError::Amount);
        }
        if self.renewed_allocation != 0 && self.renewed_allocation != input.additional.get() {
            return Err(FundingRenewalError::Conflict);
        }
        self.renewed_allocation = input.additional.get();
        Ok(self)
    }
    pub(crate) const fn response(self) -> FundingResponseRecord {
        self.response
    }
    pub(crate) fn with_response(
        mut self,
        response: FundingResponseRecord,
    ) -> Result<Self, FundingIntentError> {
        if !response.matches(self.phase)
            || (self.response != FundingResponseRecord::Missing && self.response != response)
        {
            return Err(FundingIntentError::OutcomeConflict);
        }
        self.response = response;
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
pub(crate) struct FundingJournalRecord {
    version: u8,
    service: ExternalPrincipal,
    operator: ExternalPrincipal,
    cashier: ExternalPrincipal,
    account: ExternalPrincipal,
    namespace: u128,
    allocation: FundingAllocationRecord,
    count: u64,
    last: u128,
}
impl FundingJournalRecord {
    pub(crate) fn new(config: &ServiceConfiguration, allocation: FundingAllocation) -> Self {
        let bindings = config.bindings();
        Self {
            version: 1,
            service: bindings.service.into(),
            operator: bindings.operator.into(),
            cashier: config.billing().cashier().into(),
            account: bindings.payment_account.into(),
            namespace: bindings.namespace.get(),
            allocation: FundingAllocationRecord::new(allocation),
            count: 0,
            last: 0,
        }
    }
    pub(crate) fn matches(
        self,
        config: &ServiceConfiguration,
        allocation: FundingAllocation,
    ) -> bool {
        let expected = Self::new(config, allocation);
        self.version == 1
            && self.service == expected.service
            && self.operator == expected.operator
            && self.cashier == expected.cashier
            && self.account == expected.account
            && self.namespace == expected.namespace
            && self.allocation.matches(allocation)
    }
    pub(crate) fn scope_matches(self, input: FundingIntent) -> bool {
        input.service == self.service.into()
            && input.cashier == self.cashier.into()
            && input.account == self.account.into()
            && input.namespace.get() == self.namespace
    }
    pub(crate) const fn count(self) -> u64 {
        self.count
    }
    pub(crate) const fn last(self) -> u128 {
        self.last
    }
    pub(crate) const fn limit(self) -> u64 {
        self.allocation.max_attempts()
    }
    pub(crate) const fn view(self) -> FundingAllocationView {
        self.allocation.view()
    }
    pub(crate) fn reserve(mut self, input: FundingIntent) -> Result<Self, FundingAllocationError> {
        if self.count >= self.limit() {
            return Err(FundingAllocationError::Capacity);
        }
        self.allocation = self.allocation.reserve(
            input.offered,
            usize::try_from(self.count).expect("portable lifetime bound"),
        )?;
        self.count += 1;
        self.last = input.operation.get();
        Ok(self)
    }
    pub(crate) fn renew(mut self, additional: NonZeroU128) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.renew(additional)?;
        Ok(self)
    }
    pub(crate) fn acknowledge(
        mut self,
        accepted: NonZeroU128,
    ) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.acknowledge(accepted)?;
        Ok(self)
    }
    pub(crate) fn confirm_credit(
        mut self,
        accepted: NonZeroU128,
    ) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.confirm_credit(accepted)?;
        Ok(self)
    }
    pub(crate) fn resolve(
        mut self,
        transfer: FundingTransfer,
    ) -> Result<Self, FundingAllocationError> {
        self.allocation = self.allocation.resolve(transfer)?;
        Ok(self)
    }
}
/// The accounting row and its bounded receipt index share one existing memory.
/// Zero digest belongs exclusively to accounting; receipts always have nonzero keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[candid_path("ic_platform::candid::contract")]
#[expect(
    clippy::large_enum_variant,
    reason = "One bounded persisted union keeps the sole totals row and receipt index together without heap boxing each accounting decode"
)]
pub(crate) enum FundingAccountingRecord {
    Totals(FundingJournalRecord),
    Receipt { operation: u128 },
}
impl FundingAccountingRecord {
    pub(crate) const fn totals(self) -> Option<FundingJournalRecord> {
        match self {
            Self::Totals(totals) => Some(totals),
            Self::Receipt { .. } => None,
        }
    }
}
macro_rules! codec {
    ($record:ty, $max:expr) => {
        crate::model::record_codec::storable!(
            $record, bytes: $max, work: 30_000, skipping: 1000,
            types: 64, header: $max, layout: ic_storage::adapter::storable::Bound::Bounded { max_size: $max as u32, is_fixed_size: false },
        );
    };
}
codec!(FundingIntentRecord, 1024);
codec!(FundingJournalRecord, 1024);

codec!(FundingAccountingRecord, 1024);
