//! Read-only operator diagnosis over a single independently bound observation.
//! No storage, serialization, platform calls or authority to resume any effect.
use super::billing::{
    BillingBlocker, BillingReadiness, BillingWarning, RecoveryState,
    admission::FundingActivity,
    balance::{BalanceDiagnosis, FundingNeed},
};

/// Existing billing policy result, or the reason it cannot yet be evaluated.
#[derive(Clone, Copy, Debug)]
pub enum BillingAssessment<'a> {
    /// Validated provider billing configuration is absent.
    NotConfigured,
    /// Required observations/accounting are missing; do not substitute zeroes.
    Unavailable,
    /// Diagnosis obtained from the shared billing policy for this same snapshot.
    Observed(&'a BillingReadiness),
    /// Balance thresholds are observed, but local spendability is unknown.
    BalanceOnly(&'a BalanceDiagnosis),
}

/// Independently supplied facts for diagnosis, never a public request DTO.
/// All facts must refer to one service/provider namespace and observation boundary.
#[derive(Clone, Copy, Debug)]
pub struct OperatorObservation<'a> {
    /// Recovery status established by the owning workflow; absent means unknown.
    /// Restoring a journal alone does not establish safe identity/account allocation.
    pub recovery: Option<RecoveryState>,
    /// Whether the required provider contract has been independently qualified.
    /// This policy cannot establish qualification or authenticate that claim.
    pub provider_qualified: bool,
    /// Number of currently validated gateway principals.
    pub gateway_count: usize,
    /// Shared billing diagnosis, without recomputing reserve or funding policy.
    pub billing: BillingAssessment<'a>,
    /// Authoritative funding activity; absent means unknown, never clear.
    pub funding: Option<FundingActivity>,
    /// A gateway-list reply is outstanding.
    pub sync_pending: bool,
    /// A content reply remains outstanding, including invalidated callbacks.
    pub read_pending: bool,
    /// Upload authority escaped without established completion.
    pub uncertain_uploads: usize,
    /// Released objects awaiting physical deletion.
    pub pending_deletions: usize,
    /// Physically deleted objects awaiting separate final billing evidence.
    pub pending_billing: usize,
}

/// Blocker among these observations; absence of blockers is not service qualification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorBlocker {
    /// No authoritative recovery assessment was supplied.
    RecoveryUnknown,
    /// The owner must remain fenced until independent reconciliation succeeds.
    RecoveryFenced,
    /// Provider behavior required by the service is not qualified.
    ProviderUnqualified,
    /// No current gateway authority exists.
    GatewaysMissing,
    /// Billing configuration has not been validated and installed.
    BillingNotConfigured,
    /// Missing billing/accounting inputs prevent diagnosis.
    BillingUnavailable,
    /// No authoritative local spendable-cycle observation was supplied.
    SpendabilityUnknown,
    /// Existing billing policy found this blocker.
    Billing(BillingBlocker),
    /// Funding journal/accounting was not observed.
    FundingUnknown,
    /// An exact funding attempt is still reserved or running.
    FundingInProgress,
    /// An earlier funding effect remains unresolved.
    FundingUncertain,
}

/// Outstanding work is visible independently of global readiness blockers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorWarning {
    /// Current membership is retained while a sync reply is pending.
    SyncPending,
    /// A read slot remains occupied; diagnosis cannot clear or replay it.
    ReadPending,
    /// Completion must be reconciled; diagnosis cannot repeat uploads.
    UploadCompletionUnknown,
    /// Logical release has not established physical deletion.
    DeletionPending,
    /// Physical deletion has not established billing cessation.
    BillingCessationPending,
    /// Warning from the existing shared billing policy.
    Billing(BillingWarning),
}

/// Diagnostic result only. It cannot admit uploads, funding, deletion or restore.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorDiagnosis {
    blockers: Vec<OperatorBlocker>,
    warnings: Vec<OperatorWarning>,
}

impl OperatorDiagnosis {
    /// All observed blockers; empty does not establish omitted service guarantees.
    #[must_use]
    pub fn blockers(&self) -> &[OperatorBlocker] {
        &self.blockers
    }
    /// Outstanding work and billing warnings, without mutation or retry advice.
    #[must_use]
    pub fn warnings(&self) -> &[OperatorWarning] {
        &self.warnings
    }

    fn billing(&mut self, blockers: &[BillingBlocker], warnings: &[BillingWarning]) {
        for blocker in blockers {
            self.block(match blocker {
                BillingBlocker::RecoveryFenced => OperatorBlocker::RecoveryFenced,
                BillingBlocker::GatewayPrincipalsMissing => OperatorBlocker::GatewaysMissing,
                BillingBlocker::NotConfigured => OperatorBlocker::BillingNotConfigured,
                other => OperatorBlocker::Billing(*other),
            });
        }
        self.warnings
            .extend(warnings.iter().copied().map(OperatorWarning::Billing));
    }

    fn block(&mut self, blocker: OperatorBlocker) {
        if !self.blockers.contains(&blocker) {
            self.blockers.push(blocker);
        }
    }
}

/// Compose diagnosis without converting unknown balances/activity into zero/clear.
/// Billing recovery/gateway blockers are merged without losing a stricter fence.
/// Outstanding uploads/deletions/read-only calls are warnings: their own admission
/// and lifecycle policies still decide which individual operations may proceed.
#[must_use]
pub fn assess_operator(observation: OperatorObservation<'_>) -> OperatorDiagnosis {
    let mut result = OperatorDiagnosis {
        blockers: Vec::new(),
        warnings: Vec::new(),
    };
    match observation.recovery {
        None => result.block(OperatorBlocker::RecoveryUnknown),
        Some(RecoveryState::Fenced) => result.block(OperatorBlocker::RecoveryFenced),
        Some(RecoveryState::Reconciled) => {}
    }
    if !observation.provider_qualified {
        result.block(OperatorBlocker::ProviderUnqualified);
    }
    if observation.gateway_count == 0 {
        result.block(OperatorBlocker::GatewaysMissing);
    }
    match observation.funding {
        None => result.block(OperatorBlocker::FundingUnknown),
        Some(FundingActivity::InProgress) => result.block(OperatorBlocker::FundingInProgress),
        Some(FundingActivity::Uncertain) => result.block(OperatorBlocker::FundingUncertain),
        Some(FundingActivity::Clear) => {}
    }
    match observation.billing {
        BillingAssessment::NotConfigured => result.block(OperatorBlocker::BillingNotConfigured),
        BillingAssessment::Unavailable => result.block(OperatorBlocker::BillingUnavailable),
        BillingAssessment::BalanceOnly(billing) => {
            if billing.funding_need() != FundingNeed::NotConfigured {
                result.block(OperatorBlocker::SpendabilityUnknown);
            }
            result.billing(billing.blockers(), billing.warnings());
        }
        BillingAssessment::Observed(billing) => {
            result.billing(billing.blockers(), billing.warnings());
        }
    }
    for (present, warning) in [
        (observation.sync_pending, OperatorWarning::SyncPending),
        (observation.read_pending, OperatorWarning::ReadPending),
        (
            observation.uncertain_uploads > 0,
            OperatorWarning::UploadCompletionUnknown,
        ),
        (
            observation.pending_deletions > 0,
            OperatorWarning::DeletionPending,
        ),
        (
            observation.pending_billing > 0,
            OperatorWarning::BillingCessationPending,
        ),
    ] {
        if present {
            result.warnings.push(warning);
        }
    }
    result
}

#[cfg(test)]
mod tests;
