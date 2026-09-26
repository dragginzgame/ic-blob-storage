//! Conversion and explicitly substituted confirmation facts for local tests only.

use blob_test_protocol::obligations::{
    ObligationProbeFact, ObligationProbePhase, ObligationProbeView,
};
use ic_blob_storage::model::{catalog::tenant::UnsettledObjectView, lifecycle::LifecyclePhase};

pub(crate) fn view(value: UnsettledObjectView) -> ObligationProbeView {
    ObligationProbeView {
        root: value.root.as_bytes()[0],
        phase: match value.phase {
            LifecyclePhase::Live => ObligationProbePhase::Live,
            LifecyclePhase::DeletionPending => ObligationProbePhase::DeletionPending,
            LifecyclePhase::ProviderDeleted => ObligationProbePhase::ProviderDeleted,
            LifecyclePhase::Settled => unreachable!("unsettled view excludes settlement"),
        },
        physical_bytes: value.physical_bytes,
        liability_bytes: value.liability_bytes,
    }
}

pub(crate) fn confirm(value: u8, fact: ObligationProbeFact) -> bool {
    super::STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized fixture");
        let root = super::root(value);
        let Some(journal) = state.catalog.get(root) else {
            return false;
        };
        let object = journal.lifecycle().binding();
        match fact {
            ObligationProbeFact::Deleted => state.catalog.confirm_provider_deleted(root, object),
            ObligationProbeFact::BillingStopped => {
                state.catalog.confirm_billing_stopped(root, object)
            }
        }
        .is_ok()
    })
}
