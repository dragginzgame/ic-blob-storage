//! Passive fixture projections; observations and policy decisions remain caller-owned.
use super::{BillingBlockerView, BillingWarningView, OperatorBlockerView, OperatorWarningView};
use ic_blob_storage::policy::{
    billing::{BillingBlocker, BillingWarning},
    diagnostics::{OperatorBlocker, OperatorWarning},
};

/// Preserve every shared diagnosis blocker without granting effect authority.
#[must_use]
pub fn blocker(value: OperatorBlocker) -> OperatorBlockerView {
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

/// Preserve every outstanding-work warning independently of readiness blockers.
#[must_use]
pub fn warning(value: OperatorWarning) -> OperatorWarningView {
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
