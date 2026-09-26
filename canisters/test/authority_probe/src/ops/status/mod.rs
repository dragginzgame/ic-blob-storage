//! Convert a bounded current-owner snapshot into observations and passive views.
pub(crate) mod billing;

use crate::model::archive::AuthorityArchiveRecord;
use blob_test_protocol::{
    authority::{ArchiveCatalog, ArchivePhase, ArchivedReadView},
    status::{
        BillingBlockerView, BillingWarningView, CatalogStatusView, OperatorBlockerView,
        OperatorStatusView, OperatorWarningView, PhaseCountView,
    },
};
use ic_blob_storage::policy::{
    billing::{BillingBlocker, BillingWarning, RecoveryState},
    diagnostics::{
        BillingAssessment, OperatorBlocker, OperatorDiagnosis, OperatorObservation, OperatorWarning,
    },
};

pub(crate) fn observation<'a>(
    record: &AuthorityArchiveRecord,
    billing: &'a ic_blob_storage::policy::billing::balance::BalanceDiagnosis,
) -> OperatorObservation<'a> {
    OperatorObservation {
        recovery: Some(if record.fenced {
            RecoveryState::Fenced
        } else {
            RecoveryState::Reconciled
        }),
        provider_qualified: false,
        gateway_count: record.gateways.len(),
        // Local funds remain unobserved even when thresholds and provider reports exist.
        billing: BillingAssessment::BalanceOnly(billing),
        funding: None,
        sync_pending: record.pending_sync.is_some(),
        read_pending: record.pending_read.is_some(),
        uncertain_uploads: count(record, ArchivePhase::ExposurePossible),
        pending_deletions: count(record, ArchivePhase::DeletionPending),
        pending_billing: count(record, ArchivePhase::ProviderDeleted),
    }
}

fn count(record: &AuthorityArchiveRecord, phase: ArchivePhase) -> usize {
    record.objects.iter().filter(|o| o.phase == phase).count()
}

pub(crate) fn view(
    record: &AuthorityArchiveRecord,
    diagnosis: &OperatorDiagnosis,
    billing: &ic_blob_storage::policy::billing::balance::BalanceDiagnosis,
    now: u64,
) -> OperatorStatusView {
    OperatorStatusView {
        service: record.service,
        namespace: record.namespace,
        fenced: record.fenced,
        provider_qualified: false,
        billing_configured: record.balance.current_limits().is_some(),
        billing: self::billing::view(record, billing),
        provider_balance: record.balance.total(record.fenced, now),
        balance_observation: super::balance::view(record, now),
        available_funding_cycles: None,
        funding_activity: None,
        gateways: record.gateways.clone(),
        pending_sync: record.pending_sync,
        sync_source: record.sync_source,
        last_sync: record.last_sync,
        sync_revision: record.sync_control.revision,
        pending_read: record.pending_read.as_ref().map(|read| ArchivedReadView {
            token: read.token,
            valid: read.valid,
            tenant: read.tenant,
            root: read.root,
            index: read.index,
            gateway: read.gateway,
        }),
        catalogs: [
            ArchiveCatalog::Samples,
            ArchiveCatalog::Uploads,
            ArchiveCatalog::Journey,
        ]
        .into_iter()
        .map(|catalog| catalog_view(record, catalog))
        .collect(),
        blockers: diagnosis.blockers().iter().copied().map(blocker).collect(),
        warnings: diagnosis.warnings().iter().copied().map(warning).collect(),
    }
}

fn catalog_view(record: &AuthorityArchiveRecord, catalog: ArchiveCatalog) -> CatalogStatusView {
    let entries: Vec<_> = record
        .objects
        .iter()
        .filter(|o| o.catalog == catalog)
        .collect();
    let size = |count: usize| u64::try_from(count).expect("bounded fixture count");
    CatalogStatusView {
        catalog,
        objects: size(entries.len()),
        phases: [
            ArchivePhase::Reserved,
            ArchivePhase::ExposurePossible,
            ArchivePhase::Cancelled,
            ArchivePhase::Live,
            ArchivePhase::DeletionPending,
            ArchivePhase::ProviderDeleted,
            ArchivePhase::Settled,
        ]
        .into_iter()
        .map(|phase| PhaseCountView {
            phase,
            objects: size(entries.iter().filter(|o| o.phase == phase).count()),
        })
        .collect(),
        release_receipts: size(
            entries
                .iter()
                .filter(|o| o.release_receipt.is_some())
                .count(),
        ),
        logical: entries.iter().map(|o| u128::from(o.logical)).sum(),
        physical: entries.iter().map(|o| u128::from(o.physical)).sum(),
        liability: entries.iter().map(|o| u128::from(o.liability)).sum(),
    }
}

fn blocker(value: OperatorBlocker) -> OperatorBlockerView {
    match value {
        OperatorBlocker::RecoveryUnknown => OperatorBlockerView::RecoveryUnknown,
        OperatorBlocker::RecoveryFenced => OperatorBlockerView::RecoveryFenced,
        OperatorBlocker::ProviderUnqualified => OperatorBlockerView::ProviderUnqualified,
        OperatorBlocker::GatewaysMissing => OperatorBlockerView::GatewaysMissing,
        OperatorBlocker::BillingNotConfigured => OperatorBlockerView::BillingNotConfigured,
        OperatorBlocker::BillingUnavailable => OperatorBlockerView::BillingUnavailable,
        OperatorBlocker::SpendabilityUnknown => OperatorBlockerView::SpendabilityUnknown,
        OperatorBlocker::FundingUnknown => OperatorBlockerView::FundingUnknown,
        OperatorBlocker::FundingInProgress => OperatorBlockerView::FundingInProgress,
        OperatorBlocker::FundingUncertain => OperatorBlockerView::FundingUncertain,
        OperatorBlocker::Billing(value) => OperatorBlockerView::Billing(match value {
            BillingBlocker::RecoveryFenced => BillingBlockerView::RecoveryFenced,
            BillingBlocker::NotConfigured => BillingBlockerView::NotConfigured,
            BillingBlocker::GatewayPrincipalsMissing => {
                BillingBlockerView::GatewayPrincipalsMissing
            }
            BillingBlocker::BalanceUnavailable => BillingBlockerView::BalanceUnavailable,
            BillingBlocker::BalanceMalformed => BillingBlockerView::BalanceMalformed,
            BillingBlocker::InsufficientBalance => BillingBlockerView::InsufficientBalance,
            BillingBlocker::ReserveWouldBeViolated => BillingBlockerView::ReserveWouldBeViolated,
        }),
    }
}

fn warning(value: OperatorWarning) -> OperatorWarningView {
    match value {
        OperatorWarning::SyncPending => OperatorWarningView::SyncPending,
        OperatorWarning::ReadPending => OperatorWarningView::ReadPending,
        OperatorWarning::UploadCompletionUnknown => OperatorWarningView::UploadCompletionUnknown,
        OperatorWarning::DeletionPending => OperatorWarningView::DeletionPending,
        OperatorWarning::BillingCessationPending => OperatorWarningView::BillingCessationPending,
        OperatorWarning::Billing(value) => OperatorWarningView::Billing(match value {
            BillingWarning::GatewayPrincipalSetEmpty => {
                BillingWarningView::GatewayPrincipalSetEmpty
            }
            BillingWarning::BalanceUnavailable => BillingWarningView::BalanceUnavailable,
            BillingWarning::BalanceMalformed => BillingWarningView::BalanceMalformed,
        }),
    }
}
