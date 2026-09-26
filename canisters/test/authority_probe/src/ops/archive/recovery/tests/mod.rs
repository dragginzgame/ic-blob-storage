//! Record validation is independent of platform persistence and lifecycle tests.
use super::*;
use crate::model::archive::ReadRecord;

fn record() -> AuthorityArchiveRecord {
    let p = |id| Principal::from_slice(&[id, 1]);
    super::super::capture(&ops::fresh(p(1), p(2), p(3), p(4), p(5)))
}

#[test]
fn contradictory_bindings_accounting_receipts_and_history_reject() {
    let original = record();
    assert_eq!(validate(&original), Some(()));
    let faults: &[fn(&mut AuthorityArchiveRecord)] = &[
        |r| r.release[0] ^= 1,
        |r| r.namespace = 2,
        |r| r.sync_source = r.initial_gateway,
        |r| r.gateways.push(r.initial_gateway),
        |r| r.gateways[0] = Principal::anonymous(),
        |r| r.objects[0].tenant = r.tenants[1],
        |r| r.objects[0].id = 2,
        |r| r.objects[0].logical = 0,
        |r| r.objects[0].physical = 0,
        |r| r.objects[0].liability = 0,
        |r| r.objects[0].release_receipt = Some(true),
        |r| r.objects[0].phase = ArchivePhase::Settled,
        |r| r.objects.push(r.objects[0].clone()),
        |r| {
            r.objects.remove(0);
        },
        |r| r.objects.swap(0, 1),
        |r| r.objects[2].digest = Some([0; 32]),
        |r| r.objects[3].phase = ArchivePhase::Cancelled,
        |r| r.pending_sync = Some(1),
        |r| {
            r.last_sync = 1;
            r.pending_sync = Some(0);
        },
        |r| r.trap_read_token = Some(1),
        |r| r.armed_read_trap = Some([9; 32]),
        |r| {
            r.last_read = 1;
            r.pending_read = Some(ReadRecord {
                token: 1,
                valid: true,
                tenant: r.tenants[0],
                root: [1; 32],
                index: 0,
                gateway: r.initial_gateway,
            });
        },
    ];
    for fault in faults {
        let mut candidate = original.clone();
        fault(&mut candidate);
        assert_eq!(validate(&candidate), None);
    }
}

#[test]
fn fenced_and_exhausted_sequences_remain_retained_evidence() {
    let mut record = record();
    record.last_sync = u64::MAX;
    record.pending_sync = Some(u64::MAX);
    record.last_read = u64::MAX;
    record.fenced = true;
    assert_eq!(validate(&record), Some(()));
}
