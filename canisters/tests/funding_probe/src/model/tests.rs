use super::*;
use blob_test_protocol::funding::FundingReplyMode;

fn journal() -> FundingJournalRecord {
    let p = |id| Principal::from_slice(&[id, 1]);
    FundingJournalRecord::new(p(1), p(2), p(3))
}

#[test]
fn restoration_rejects_wrong_identity_release_and_inconsistent_receipts() {
    let service = Principal::from_slice(&[1, 1]);
    let faults: &[fn(&mut FundingJournalRecord)] = &[
        |r| r.service = r.peer,
        |r| r.peer = r.service,
        |r| r.driver = Principal::anonymous().into(),
        |r| r.release[0] ^= 1,
        |r| r.receipts[0].available = 0,
        |r| r.receipts[0].available = MAX_OFFERED + 1,
        |r| r.receipts[0].accepted = 101,
        |r| r.receipts.push(r.receipts[0]),
    ];
    for fault in faults {
        let mut record = journal();
        record.record_acceptance(FundingReceiptRecord {
            id: 1,
            available: 100,
            accepted: 50,
        });
        assert_eq!(record.validate(service), Ok(()));
        fault(&mut record);
        assert!(record.validate(service).is_err());
    }
}

#[test]
fn receipt_capacity_and_restore_fence_never_forget_acceptance() {
    let mut record = journal();
    for id in 0..MAX_RECEIPTS as u64 {
        record.record_acceptance(FundingReceiptRecord {
            id,
            available: 100,
            accepted: 50,
        });
    }
    let driver = Principal::from_slice(record.driver.as_slice());
    assert_eq!(record.receipts(Principal::anonymous()), None);
    assert_eq!(record.receipts(driver).unwrap().len(), MAX_RECEIPTS);
    assert_eq!(
        record.validate(Principal::from_slice(record.service.as_slice())),
        Ok(())
    );
    let before = record.receipts(driver);
    let request = FundingRequest {
        id: MAX_RECEIPTS as u64,
        offered: 100,
        accept: 50,
        reply: FundingReplyMode::Success,
    };
    let peer = Principal::from_slice(record.peer.as_slice());
    assert!(std::panic::catch_unwind(|| record.check_receive(peer, request, 100)).is_err());
    record.fence();
    assert!(std::panic::catch_unwind(|| record.check_receive(peer, request, 100)).is_err());
    assert_eq!(record.receipts(driver), before);
}
