//! Fixed-size confirmed lifecycle, individual reference and exact receipt records.
use super::codec;
use crate::model::lifecycle::LifecycleChange;
use crate::model::lifecycle::LifecycleError;
use crate::model::lifecycle::ReferenceMutation;
use crate::model::lifecycle::ReferenceState;
use crate::model::lifecycle::requests::ReferenceRequestError;
use crate::model::lifecycle::requests::admit_receipt;
use crate::model::lifecycle::transition::ReferenceStateView;
use candid::{CandidType, DecoderConfig, Deserialize, Principal, decode_one_with_config};
use ic_blob_storage_contracts::binding::ObjectBinding;
use ic_blob_storage_contracts::configuration::limits::CatalogLimits;
use ic_blob_storage_contracts::reference::binding::ReferenceRequest;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_memory::ic_stable_structures::{Storable, storable::Bound};
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
enum PhaseRecord {
    Live,
    DeletionPending,
    ProviderDeleted,
    Settled,
}
impl From<LifecyclePhase> for PhaseRecord {
    fn from(value: LifecyclePhase) -> Self {
        match value {
            LifecyclePhase::Live => Self::Live,
            LifecyclePhase::DeletionPending => Self::DeletionPending,
            LifecyclePhase::ProviderDeleted => Self::ProviderDeleted,
            LifecyclePhase::Settled => Self::Settled,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ConfirmedLifecycleRecord {
    version: u8,
    phase: PhaseRecord,
    references: u64,
    active: u64,
    receipts: u64,
    completion: CompletionRecord,
}
/// Current schema distinguishes low-level host facts from retained verifier evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) enum CompletionRecord {
    HostFact,
    Attested(AttestationRecord),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct AttestationRecord {
    pub(crate) verifier: Principal,
    pub(crate) content_digest: [u8; 32],
    pub(crate) observed_at_ns: u64,
    pub(crate) accepted_at_ns: u64,
}
impl CompletionRecord {
    pub(crate) fn valid(self) -> bool {
        match self {
            Self::HostFact => true,
            Self::Attested(r) => {
                r.verifier != Principal::anonymous()
                    && r.verifier != Principal::management_canister()
                    && r.observed_at_ns <= r.accepted_at_ns
            }
        }
    }
}
impl ConfirmedLifecycleRecord {
    pub(crate) const fn new(completion: CompletionRecord) -> Self {
        Self {
            version: 1,
            phase: PhaseRecord::Live,
            references: 1,
            active: 1,
            receipts: 0,
            completion,
        }
    }
    pub(crate) const fn completion(self) -> CompletionRecord {
        self.completion
    }
    pub(crate) const fn phase(self) -> LifecyclePhase {
        match self.phase {
            PhaseRecord::Live => LifecyclePhase::Live,
            PhaseRecord::DeletionPending => LifecyclePhase::DeletionPending,
            PhaseRecord::ProviderDeleted => LifecyclePhase::ProviderDeleted,
            PhaseRecord::Settled => LifecyclePhase::Settled,
        }
    }
    pub(crate) const fn counts(self) -> (u64, u64, u64) {
        (self.references, self.active, self.receipts)
    }
    pub(crate) fn valid(self, limits: CatalogLimits) -> bool {
        self.version == 1
            && self.completion.valid()
            && self.references > 0
            && self.active <= self.references
            && (self.phase == PhaseRecord::Live) == (self.active > 0)
            && self.references <= limits.max_references_per_object.get() as u64
            && self.receipts <= limits.max_receipts_per_object.get() as u64
            && self.active <= limits.max_receipts_per_object.get() as u64 - self.receipts
    }
    pub(crate) fn plan(
        self,
        binding: ObjectBinding,
        known: Option<ReferenceState>,
        request: ReferenceRequest,
        limits: CatalogLimits,
    ) -> Result<Result<Option<ReferenceMutation>, LifecycleError>, ReferenceRequestError> {
        binding.check(request.operation.key().object())?;
        let planned = ReferenceStateView {
            binding,
            phase: self.phase(),
            slots: usize::try_from(self.references).expect("validated portable reference count"),
            active: usize::try_from(self.active).expect("validated portable active count"),
            limit: limits.max_references_per_object.get(),
        }
        .plan(
            matches!(
                request.operation,
                ic_blob_storage_contracts::reference::binding::ReferenceOperation::Retain(_)
            ),
            known,
            request.operation.key(),
        );
        let active = match &planned {
            Ok(Some(m)) => m.active_references,
            _ => usize::try_from(self.active).expect("validated portable active count"),
        };
        admit_receipt(
            limits.max_receipts_per_object.get()
                - usize::try_from(self.receipts).expect("validated portable receipt count"),
            active,
        )?;
        Ok(planned)
    }
    pub(crate) fn commit(&mut self, mutation: Option<&ReferenceMutation>) {
        self.receipts += 1;
        if let Some(m) = mutation {
            if m.state == ReferenceState::Active {
                self.references += 1;
            }
            self.active = m.active_references as u64;
            if self.active == 0 {
                self.phase = PhaseRecord::DeletionPending;
            }
        }
    }
    pub(crate) fn confirm(&mut self, billing: bool) -> Result<LifecycleChange, LifecycleError> {
        let (phase, change) = if billing {
            crate::model::lifecycle::transition::settled(self.phase())?
        } else {
            crate::model::lifecycle::transition::deleted(self.phase())?
        };
        self.phase = phase.into();
        Ok(change)
    }
}
pub(crate) fn contribution(bytes: u64, phase: LifecyclePhase) -> (u128, u128, u128) {
    let b = u128::from(bytes);
    match phase {
        LifecyclePhase::Live => (b, b, b),
        LifecyclePhase::DeletionPending => (0, b, b),
        LifecyclePhase::ProviderDeleted => (0, 0, b),
        LifecyclePhase::Settled => (0, 0, 0),
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ReferenceRecord {
    version: u8,
    active: bool,
}
impl ReferenceRecord {
    pub(crate) const fn new(state: ReferenceState) -> Self {
        Self {
            version: 1,
            active: matches!(state, ReferenceState::Active),
        }
    }
    pub(crate) fn state(self) -> Option<ReferenceState> {
        (self.version == 1).then_some(if self.active {
            ReferenceState::Active
        } else {
            ReferenceState::Released
        })
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
enum ResultRecord {
    Changed,
    Unchanged,
    UnknownReference,
    ReferenceReleased,
    ReferenceLimitReached,
    DeletionAlreadyQueued,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub(crate) struct ReferenceReceiptRecord {
    version: u8,
    actor: Principal,
    reference: u128,
    retain: bool,
    result: ResultRecord,
}
impl ReferenceReceiptRecord {
    pub(crate) fn new(
        actor: Principal,
        request: ReferenceRequest,
        result: Result<LifecycleChange, LifecycleError>,
    ) -> Self {
        let result = match result {
            Ok(LifecycleChange::Changed) => ResultRecord::Changed,
            Ok(LifecycleChange::Unchanged) => ResultRecord::Unchanged,
            Err(LifecycleError::UnknownReference) => ResultRecord::UnknownReference,
            Err(LifecycleError::ReferenceReleased) => ResultRecord::ReferenceReleased,
            Err(LifecycleError::ReferenceLimitReached) => ResultRecord::ReferenceLimitReached,
            Err(LifecycleError::DeletionAlreadyQueued) => ResultRecord::DeletionAlreadyQueued,
            Err(_) => {
                unreachable!("binding checked before receipt admission; only reference transitions")
            }
        };
        Self {
            version: 1,
            actor,
            reference: request.operation.key().reference().get().get(),
            retain: matches!(
                request.operation,
                ic_blob_storage_contracts::reference::binding::ReferenceOperation::Retain(_)
            ),
            result,
        }
    }
    pub(crate) const fn reference(self) -> u128 {
        self.reference
    }
    pub(crate) fn matches(self, actor: Principal, request: ReferenceRequest) -> bool {
        self.actor == actor
            && self.reference == request.operation.key().reference().get().get()
            && self.retain
                == matches!(
                    request.operation,
                    ic_blob_storage_contracts::reference::binding::ReferenceOperation::Retain(_)
                )
    }
    pub(crate) fn valid(self, actor: Principal, state: Option<ReferenceState>) -> bool {
        if self.version != 1 || self.reference == 0 || self.actor != actor {
            return false;
        }
        match self.result {
            ResultRecord::Changed | ResultRecord::Unchanged => {
                if self.retain {
                    state.is_some()
                } else {
                    state == Some(ReferenceState::Released)
                }
            }
            ResultRecord::UnknownReference => !self.retain,
            ResultRecord::ReferenceReleased => {
                self.retain && state == Some(ReferenceState::Released)
            }
            ResultRecord::ReferenceLimitReached | ResultRecord::DeletionAlreadyQueued => {
                self.retain
            }
        }
    }
    pub(crate) const fn result(self) -> Result<LifecycleChange, LifecycleError> {
        match self.result {
            ResultRecord::Changed => Ok(LifecycleChange::Changed),
            ResultRecord::Unchanged => Ok(LifecycleChange::Unchanged),
            ResultRecord::UnknownReference => Err(LifecycleError::UnknownReference),
            ResultRecord::ReferenceReleased => Err(LifecycleError::ReferenceReleased),
            ResultRecord::ReferenceLimitReached => Err(LifecycleError::ReferenceLimitReached),
            ResultRecord::DeletionAlreadyQueued => Err(LifecycleError::DeletionAlreadyQueued),
        }
    }
}
codec!(
    ConfirmedLifecycleRecord,
    512,
    Bound::Bounded {
        max_size: 512,
        is_fixed_size: false
    }
);
codec!(
    ReferenceRecord,
    64,
    Bound::Bounded {
        max_size: 64,
        is_fixed_size: false
    }
);
codec!(
    ReferenceReceiptRecord,
    256,
    Bound::Bounded {
        max_size: 256,
        is_fixed_size: false
    }
);

#[cfg(test)]
mod tests;
