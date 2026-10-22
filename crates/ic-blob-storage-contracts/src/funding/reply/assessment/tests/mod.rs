use super::*;
use crate::dto::operator::{LocalFundingStatus, OperatorScope};
use ic_platform::Principal;

fn request() -> Request {
    Request {
        scope: OperatorScope {
            service: Principal::self_authenticating([1]),
            namespace: u128::MAX,
            cashier: Principal::self_authenticating([2]),
            payment_account: Principal::self_authenticating([3]),
        },
        operation: u128::MAX,
        offered: u128::MAX,
        target_balance: Some(u128::MAX),
    }
}
fn response() -> Response {
    Response {
        request: request(),
        journal: LocalFundingStatus {
            cumulative_allocation: u128::MAX,
            renewal_ceiling: u128::MAX,
            available_allocation: u128::MAX,
            attachment_allowance: u128::MAX - 1,
            transport_accepted: 0,
            uncredited_accepted: 0,
            provider_acknowledged: 0,
            refunded: 0,
            not_enqueued: 0,
            reserved_or_uncertain: 0,
            retained_intents: 0,
            intent_capacity: u64::MAX,
            last_operation: None,
            fenced: false,
        },
        blockers: vec![
            B::ProviderUnqualified,
            B::RecoveryUnknown,
            B::FundingUnknown,
            B::SpendabilityUnknown,
            B::AllocationReserve {
                transferable_cycles: u128::MAX - 1,
            },
        ],
    }
}
fn bytes(value: &Response) -> Vec<u8> {
    crate::wire::encode_response("blob_funding_preparation_assessment", Ok::<_, E>(value)).unwrap()
}
fn decode(request: Request, bytes: &[u8]) -> Result<Response, FundingAssessmentReplyError> {
    inspect(request, bytes, 4096.try_into().unwrap())
}

#[test]
fn decoder_refuses_changed_request_and_inconsistent_or_missing_blockers() {
    let original = response();
    let mut changed = original.clone();
    changed.request.target_balance = None;
    assert_eq!(
        decode(request(), &bytes(&changed)),
        Err(FundingAssessmentReplyError::Binding)
    );
    for i in 0..original.blockers.len() {
        changed = original.clone();
        changed.blockers.remove(i);
        assert_eq!(
            decode(request(), &bytes(&changed)),
            Err(FundingAssessmentReplyError::Invalid)
        );
    }
    for b in [
        B::ProviderUnqualified,
        B::JournalFenced,
        B::IdentityRetained,
        B::IdentityStale,
        B::JournalFull,
        B::JournalUncredited,
        B::AllocationReserve {
            transferable_cycles: 1,
        },
    ] {
        changed = original.clone();
        changed.blockers.push(b);
        assert_eq!(
            decode(request(), &bytes(&changed)),
            Err(FundingAssessmentReplyError::Invalid)
        );
    }
    changed = original.clone();
    changed.journal.attachment_allowance = 0;
    assert_eq!(
        decode(request(), &bytes(&changed)),
        Err(FundingAssessmentReplyError::Invalid)
    );
    assert_eq!(
        decode(request(), &[0]),
        Err(FundingAssessmentReplyError::Invalid)
    );
    assert_eq!(
        decode(request(), &vec![0; 4097]),
        Err(FundingAssessmentReplyError::Limit)
    );
    assert_eq!(
        decode(
            request(),
            &crate::wire::encode_response(
                "blob_funding_preparation_assessment",
                Err::<Response, _>(E::Conflict)
            )
            .unwrap()
        ),
        Err(FundingAssessmentReplyError::Remote(E::Conflict))
    );
}
#[test]
fn occupied_fenced_journal_keeps_uncredited_capacity_and_retained_identity_visible() {
    let mut reply = response();
    reply.journal.fenced = true;
    reply.journal.retained_intents = u64::MAX;
    reply.journal.last_operation = Some(u128::MAX);
    reply.journal.reserved_or_uncertain = 1;
    reply.journal.available_allocation -= 1;
    reply.journal.attachment_allowance -= 1;
    for blocker in &mut reply.blockers {
        if let B::AllocationReserve {
            transferable_cycles,
        } = blocker
        {
            *transferable_cycles = reply.journal.attachment_allowance;
        }
    }
    reply.blockers.extend([
        B::JournalFenced,
        B::JournalFull,
        B::JournalUncredited,
        B::IdentityRetained,
    ]);
    assert_eq!(decode(request(), &bytes(&reply)), Ok(reply.clone()));
    reply.blockers.push(B::IdentityStale);
    assert_eq!(
        decode(request(), &bytes(&reply)),
        Err(FundingAssessmentReplyError::Invalid)
    );
}

#[test]
fn confirmed_acceptance_clears_only_local_credit_blocker_and_invalid_totals_refuse() {
    let mut view = response();
    view.journal.transport_accepted = 100;
    view.journal.available_allocation -= 100;
    view.journal.attachment_allowance -= 100;
    for blocker in &mut view.blockers {
        if let B::AllocationReserve {
            transferable_cycles,
        } = blocker
        {
            *transferable_cycles = view.journal.attachment_allowance;
        }
    }
    view.journal.uncredited_accepted = 0;
    assert_eq!(decode(request(), &bytes(&view)), Ok(view.clone()));
    for journal in [
        LocalFundingStatus {
            intent_capacity: 0,
            ..view.journal
        },
        LocalFundingStatus {
            retained_intents: 1,
            ..view.journal
        },
    ] {
        let mut changed = view.clone();
        changed.journal = journal;
        assert_eq!(
            decode(request(), &bytes(&changed)),
            Err(FundingAssessmentReplyError::Invalid)
        );
    }
    view.journal.uncredited_accepted = 101;
    assert_eq!(
        decode(request(), &bytes(&view)),
        Err(FundingAssessmentReplyError::Invalid)
    );
}
