use blob_test_protocol::{
    funding::{FundingOperatorStatusView, FundingReconciliationView},
    status::OperatorStatusView,
};
use serde_json::{Value, json};

// Decimal strings preserve full-width counters and cycles in JSON consumers.
fn amount(value: Option<u128>) -> Option<String> {
    value.map(|v| v.to_string())
}

pub(super) fn authority(status: &OperatorStatusView) -> Value {
    json!({
        "service":status.service.to_text(), "namespace":status.namespace.to_string(),
        "fenced":status.fenced, "provider_qualified":status.provider_qualified,
        "billing_configured":status.billing_configured,
        "provider_balance":amount(status.provider_balance),
        "balance_observation":balance(&status.balance_observation),
        "billing":billing(&status.billing),
        "available_funding_cycles":amount(status.available_funding_cycles),
        "funding_activity":status.funding_activity,
        "gateways":status.gateways.iter().map(candid::Principal::to_text).collect::<Vec<_>>(),
        "pending_sync":status.pending_sync.map(|v| v.to_string()),
        "sync_source":status.sync_source.to_text(),
        "last_sync":status.last_sync.to_string(),
        "sync_revision":status.sync_revision.map(|v| v.to_string()),
        "pending_read":status.pending_read.as_ref().map(|read| json!({
            "token":read.token.to_string(), "valid":read.valid, "tenant":read.tenant.to_text(),
            "root":read.root, "index":read.index.to_string(), "gateway":read.gateway.to_text(),
        })),
        "catalogs":status.catalogs.iter().map(|catalog| json!({
            "catalog":catalog.catalog, "objects":catalog.objects.to_string(),
            "phases":catalog.phases.iter().map(|p| json!({
                "phase":p.phase, "objects":p.objects.to_string(),
            })).collect::<Vec<_>>(),
            "release_receipts":catalog.release_receipts.to_string(),
            "logical_bytes":catalog.logical.to_string(), "physical_bytes":catalog.physical.to_string(),
            "billing_liability_bytes":catalog.liability.to_string(),
        })).collect::<Vec<_>>(),
        "blockers":status.blockers, "warnings":status.warnings,
    })
}

pub(super) fn funding(status: &FundingOperatorStatusView) -> Value {
    json!({
        "service":status.service.to_text(), "peer":status.peer.to_text(),
        "fenced":status.fenced, "provider_qualified":status.provider_qualified,
        "billing_configured":status.billing_configured,
        "provider_balance":amount(status.provider_balance),
        "available_funding_cycles":amount(status.available_funding_cycles),
        "budget":budget(&status.budget),
        "funding_activity":status.funding_activity,
        "attempts":status.attempts.iter().map(|attempt| json!({
            "id":attempt.id.to_string(), "offered":attempt.offered.to_string(),
            "refunded":amount(attempt.refunded), "transport_accepted":amount(attempt.transport_accepted),
            "outcome":attempt.outcome, "provider_credit":amount(attempt.provider_credit),
            "reconciliation":reconciliation(attempt.reconciliation),
        })).collect::<Vec<_>>(),
        "receipts":status.receipts.iter().map(|receipt| json!({
            "id":receipt.id.to_string(), "available":receipt.available.to_string(),
            "accepted":receipt.accepted.to_string(),
        })).collect::<Vec<_>>(),
        "blockers":status.blockers, "warnings":status.warnings,
    })
}

pub(in crate::operator) fn reconciliation(value: FundingReconciliationView) -> Value {
    match value {
        FundingReconciliationView::CreditConfirmed {
            accepted_cycles,
            receipt_digest,
        } => json!({
            "kind":"CreditConfirmed", "cycles":accepted_cycles.to_string(),
            "receipt_digest":receipt_digest,
        }),
        FundingReconciliationView::NoTransfer => json!({"kind":"NoTransfer"}),
        FundingReconciliationView::CreditRequired(cycles) => {
            json!({"kind":"CreditRequired", "cycles":cycles.to_string()})
        }
        FundingReconciliationView::TransferUnknown(cycles) => {
            json!({"kind":"TransferUnknown", "cycles":cycles.to_string()})
        }
    }
}

pub(in crate::operator) fn budget(
    view: &blob_test_protocol::funding::budget::FundingBudgetView,
) -> Value {
    json!({"scope":"local_attachment_budget", "allocated":view.allocated.to_string(), "reserve":view.reserve.to_string(),
        "operating_reserve":view.operating_reserve.to_string(), "other_liabilities":view.other_liabilities.to_string(),
        "revision":view.revision.to_string(), "available":view.available.to_string(),
        "accepted":view.accepted.to_string(), "refunded":view.refunded.to_string(),
        "not_enqueued":view.not_enqueued.to_string(), "reserved_or_uncertain":view.reserved_or_uncertain.to_string()})
}

pub(in crate::operator) fn target(target: &crate::operator::model::Target) -> Value {
    use crate::operator::model::Binding;
    let binding = match target.binding {
        Binding::Authority(namespace) => {
            json!({"kind":"authority", "namespace":namespace.to_string()})
        }
        Binding::Funding(peer) => json!({"kind":"funding", "peer":peer.to_text()}),
    };
    json!({
        "server":target.server.to_string(), "instance":target.instance.to_string(),
        "canister":target.canister.to_text(), "caller":target.caller.to_text(),
        "binding":binding,
    })
}

fn balance(status: &blob_test_protocol::balance::BalanceStatusView) -> Value {
    use blob_test_protocol::balance::BalanceOutcomeView;
    let scope = |s: blob_test_protocol::balance::BalanceScope| {
        json!({
            "service":s.service.to_text(), "namespace":s.namespace.to_string(),
            "source":s.source.to_text(), "account":s.account.to_text(),
        })
    };
    json!({
        "scope":"controlled_source", "configured":status.configured.map(scope),
        "revision":status.revision.to_string(), "max_age_ns":status.max_age_ns.to_string(),
        "usability":status.usability,
        "attempts":status.attempts.iter().map(|a| json!({
            "sequence":a.sequence.to_string(), "revision":a.revision.to_string(), "scope":scope(a.scope),
            "started_at":a.started_at.to_string(),
            "outcome":a.outcome.map(|o| match o {
                BalanceOutcomeView::Reported(v, time) => json!({"kind":"Reported", "received_at":time.to_string(),
                    "total":v.total.to_string(), "prepaid":v.prepaid.to_string(), "promotional":v.promotional.to_string(), "ledger":v.ledger.to_string()}),
                BalanceOutcomeView::Failed(e) => json!({"kind":"Failed", "error":e}),
            }),
        })).collect::<Vec<_>>(),
    })
}

fn billing(status: &blob_test_protocol::billing::BillingStatusView) -> Value {
    use blob_test_protocol::billing::FundingNeedView;
    json!({
        "current":status.current, "revision":status.revision.map(|v| v.to_string()),
        "scope":status.scope.map(|s| json!({"service":s.service.to_text(), "namespace":s.namespace.to_string(), "source":s.source.to_text(), "account":s.account.to_text()})),
        "limits":status.limits.map(|v| json!({"reserve":v.reserve.to_string(), "minimum":v.minimum.to_string(), "target":v.target.to_string()})),
        "funding_need":match status.funding_need {
            FundingNeedView::NotConfigured => json!({"kind":"NotConfigured"}),
            FundingNeedView::NotNeeded => json!({"kind":"NotNeeded"}),
            FundingNeedView::BalanceUnavailable => json!({"kind":"BalanceUnavailable"}),
            FundingNeedView::BalanceMalformed => json!({"kind":"BalanceMalformed"}),
            FundingNeedView::TopUp(cycles) => json!({"kind":"TopUp", "cycles":cycles.to_string()}),
        },
    })
}
