//! Conversion and explicitly substituted confirmation facts for local tests only.

use blob_test_protocol::obligations::{
    ObligationProbeFact, ObligationProbePhase, ObligationProbeView,
};
use ic_blob_storage::model::catalog::admission::read::UploadPageLimits;
use ic_blob_storage::ops::service::uploads::read::UploadScanScope;
use ic_blob_storage_contracts::upload::history::LifecyclePhase;
use ic_blob_storage_contracts::upload::history::UploadScanFilter;

pub(crate) fn views(
    context: ic_blob_storage::policy::tenant::TenantAccessContext,
) -> Option<Vec<ObligationProbeView>> {
    super::read(|state| {
        state
            .catalog
            .scan(
                super::durable::context(context.service, context.actor),
                UploadScanScope::Tenant {
                    tenant: context.actor,
                    namespace: super::number(1),
                },
                UploadScanFilter::Outstanding,
                None,
                UploadPageLimits {
                    max_scan: super::bound(2),
                    max_results: super::bound(2),
                },
            )
            .ok()
            .map(|page| {
                page.entries
                    .into_iter()
                    .map(|entry| {
                        let phase = state
                            .catalog
                            .confirmed(
                                super::durable::context(context.service, context.actor),
                                entry.request,
                            )
                            .unwrap()
                            .phase;
                        ObligationProbeView {
                            root: u8::try_from(entry.request.id.get().get()).expect("fixture ID"),
                            phase: match phase {
                                LifecyclePhase::Live => ObligationProbePhase::Live,
                                LifecyclePhase::DeletionPending => {
                                    ObligationProbePhase::DeletionPending
                                }
                                LifecyclePhase::ProviderDeleted => {
                                    ObligationProbePhase::ProviderDeleted
                                }
                                LifecyclePhase::Settled => unreachable!("unsettled scan"),
                            },
                            physical_bytes: if phase == LifecyclePhase::ProviderDeleted {
                                0
                            } else {
                                entry.request.object.bytes
                            },
                            liability_bytes: entry.request.object.bytes,
                        }
                    })
                    .collect()
            })
    })
}

pub(crate) fn confirm(value: u8, fact: ObligationProbeFact) -> bool {
    super::mutate(|state| {
        let Some(request) = super::sample(state, value) else {
            return false;
        };
        match fact {
            ObligationProbeFact::Deleted => state.catalog.confirm_provider_deleted(request),
            ObligationProbeFact::BillingStopped => state.catalog.confirm_billing_stopped(request),
        }
        .is_ok()
    })
}
