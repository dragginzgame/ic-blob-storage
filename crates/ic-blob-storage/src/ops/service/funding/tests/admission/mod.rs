use super::*;
use crate::{
    model::billing::journal::FundingJournalScope,
    policy::billing::{
        RecoveryState,
        admission::{
            FundingActivity,
            evidence::{FundingAdmissionEvidence, FundingEvidenceBlocker},
            journal::{FundingHostEvidence, FundingPreparationBlocker as B},
        },
    },
    workflow::funding::{FundingPreparationResult, inspect_preparation, prepare_new},
};
fn evidence() -> FundingHostEvidence {
    let i = input(1, 1);
    FundingHostEvidence {
        scope: FundingJournalScope {
            service: i.service,
            cashier: i.cashier,
            account: i.account,
            namespace: i.namespace,
        },
        provider_qualified: true,
        admission: FundingAdmissionEvidence {
            available_cycles: Some(u128::MAX),
            recovery: Some(RecoveryState::Reconciled),
            activity: Some(FundingActivity::Clear),
        },
    }
}
#[test]
fn guarded_preparation_rechecks_current_journal_instead_of_accepting_a_prior_preview() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let intent = input(9, 100);
    let before = m.accounting.borrow().clone();
    assert!(
        inspect_preparation(&store, execution(), intent, evidence())
            .unwrap()
            .assessment
            .can_prepare_intent()
    );
    assert_eq!(*m.accounting.borrow(), before);
    let old = input(1, 900);
    store.prepare(execution(), old).unwrap();
    store.mark_attempted(execution(), old).unwrap();
    store
        .record_transport(
            execution(),
            old,
            source(),
            FundingTransportOutcome::Callback { refunded: 800 },
        )
        .unwrap();
    let totals = store.allocation(execution()).unwrap();
    let FundingPreparationResult::Blocked(blocked) =
        prepare_new(&mut store, execution(), intent, evidence()).unwrap()
    else {
        panic!("local acceptance blocks new reservation")
    };
    assert!(blocked.blockers().contains(&B::JournalUncredited));
    assert_eq!(store.lookup(execution(), intent), Ok(None));
    assert_eq!(store.allocation(execution()), Ok(totals));
}
#[test]
fn unknown_host_facts_cannot_be_filled_by_clear_local_state_or_gross_allocation() {
    let mut store = StableFundingJournal::install(memories(), config(), allocation()).unwrap();
    let intent = input(1, 100);
    let host = FundingHostEvidence {
        provider_qualified: false,
        admission: FundingAdmissionEvidence {
            available_cycles: None,
            recovery: None,
            activity: None,
        },
        ..evidence()
    };
    let before = store.allocation(execution()).unwrap();
    let FundingPreparationResult::Blocked(blocked) =
        prepare_new(&mut store, execution(), intent, host).unwrap()
    else {
        panic!("missing prerequisites")
    };
    for blocker in [
        B::ProviderUnqualified,
        B::Evidence(FundingEvidenceBlocker::RecoveryUnknown),
        B::Evidence(FundingEvidenceBlocker::FundingUnknown),
        B::Evidence(FundingEvidenceBlocker::SpendabilityUnknown),
    ] {
        assert!(blocked.blockers().contains(&blocker));
    }
    assert_eq!(store.lookup(execution(), intent), Ok(None));
    assert_eq!(store.allocation(execution()), Ok(before));
    assert_eq!(
        prepare_new(&mut store, execution(), intent, evidence()),
        Ok(FundingPreparationResult::Prepared(
            FundingIntentAdmission::Created
        ))
    );
    let FundingPreparationResult::Blocked(repeated) =
        prepare_new(&mut store, execution(), intent, evidence()).unwrap()
    else {
        panic!("exact identity remains retained")
    };
    assert!(repeated.blockers().contains(&B::IdentityRetained));
}
#[test]
fn bound_host_evidence_never_overrides_authority_scope_or_restoration() {
    let m = memories();
    let mut store = StableFundingJournal::install(copy(&m), config(), allocation()).unwrap();
    let intent = input(1, 100);
    let stranger = Principal::anonymous();
    assert!(matches!(
        inspect_preparation(
            &store,
            UploadContext {
                actor: stranger,
                ..execution()
            },
            intent,
            evidence()
        ),
        Err(FundingJournalError::NotOperator)
    ));
    for scope in [
        FundingJournalScope {
            service: stranger,
            ..evidence().scope
        },
        FundingJournalScope {
            cashier: stranger,
            ..evidence().scope
        },
        FundingJournalScope {
            account: stranger,
            ..evidence().scope
        },
        FundingJournalScope {
            namespace: n(2),
            ..evidence().scope
        },
    ] {
        assert_eq!(
            prepare_new(
                &mut store,
                execution(),
                intent,
                FundingHostEvidence {
                    scope,
                    ..evidence()
                }
            ),
            Err(FundingJournalError::Binding)
        );
    }
    drop(store);
    let mut restored = StableFundingJournal::open(copy(&m), config(), allocation()).unwrap();
    let FundingPreparationResult::Blocked(blocked) =
        prepare_new(&mut restored, execution(), intent, evidence()).unwrap()
    else {
        panic!("actual restore fence dominates host assertion")
    };
    assert!(blocked.blockers().contains(&B::JournalFenced));
    assert_eq!(restored.lookup(execution(), intent), Ok(None));
}
