//! Shared constant-size lifecycle decisions for heap and stable owners.
use super::{
    LifecycleChange, LifecycleError, LifecyclePhase, ReferenceMutation, ReferenceState,
    binding::{ObjectBinding, ReferenceKey},
};

pub(crate) struct ReferenceStateView {
    pub binding: ObjectBinding,
    pub phase: LifecyclePhase,
    pub slots: usize,
    pub active: usize,
    pub limit: usize,
}
impl ReferenceStateView {
    pub(crate) fn plan(
        &self,
        retain: bool,
        known: Option<ReferenceState>,
        key: ReferenceKey,
    ) -> Result<Option<ReferenceMutation>, LifecycleError> {
        self.binding.check(key.object())?;
        let active = if retain {
            match known {
                Some(ReferenceState::Active) => return Ok(None),
                Some(ReferenceState::Released) => return Err(LifecycleError::ReferenceReleased),
                None => {}
            }
            if self.phase != LifecyclePhase::Live {
                return Err(LifecycleError::DeletionAlreadyQueued);
            }
            if self.slots >= self.limit {
                return Err(LifecycleError::ReferenceLimitReached);
            }
            self.active + 1
        } else {
            match known {
                Some(ReferenceState::Released) => return Ok(None),
                None => return Err(LifecycleError::UnknownReference),
                Some(ReferenceState::Active) => self.active - 1,
            }
        };
        Ok(Some(ReferenceMutation {
            reference: key.reference(),
            state: if retain {
                ReferenceState::Active
            } else {
                ReferenceState::Released
            },
            active_references: active,
        }))
    }
}
pub(crate) fn deleted(
    phase: LifecyclePhase,
) -> Result<(LifecyclePhase, LifecycleChange), LifecycleError> {
    match phase {
        LifecyclePhase::Live => Err(LifecycleError::LiveReferencesRemain),
        LifecyclePhase::DeletionPending => {
            Ok((LifecyclePhase::ProviderDeleted, LifecycleChange::Changed))
        }
        _ => Ok((phase, LifecycleChange::Unchanged)),
    }
}
pub(crate) fn settled(
    phase: LifecyclePhase,
) -> Result<(LifecyclePhase, LifecycleChange), LifecycleError> {
    match phase {
        LifecyclePhase::Live | LifecyclePhase::DeletionPending => {
            Err(LifecycleError::DeletionNotConfirmed)
        }
        LifecyclePhase::ProviderDeleted => Ok((LifecyclePhase::Settled, LifecycleChange::Changed)),
        LifecyclePhase::Settled => Ok((phase, LifecycleChange::Unchanged)),
    }
}
