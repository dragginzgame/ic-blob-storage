//! Independent totals reconstructed from observable operations after each action.
use super::*;

fn audit(owner: &UploadCatalog, admitted: &[UploadRequest]) {
    for tenant in [None, Some(p(2)), Some(p(3)), Some(p(9))] {
        let mut expected = UploadUsage::default();
        for input in admitted
            .iter()
            .filter(|input| tenant.is_none_or(|p| p == actor(**input)))
        {
            expected.operations += 1;
            match owner.phase(actor(*input), *input).unwrap() {
                UploadPhase::Reserved | UploadPhase::ExposurePossible => {
                    expected.active_reservations += 1;
                    expected.reserved_bytes += u128::from(input.object.bytes);
                    expected.logical_bytes += u128::from(input.object.bytes);
                    expected.physical_bytes += u128::from(input.object.bytes);
                    expected.liability_bytes += u128::from(input.object.bytes);
                }
                UploadPhase::Confirmed => {
                    let lifecycle = owner
                        .confirmed()
                        .get(input.object.root)
                        .unwrap()
                        .lifecycle();
                    expected.logical_bytes += u128::from(lifecycle.logical_bytes());
                    expected.physical_bytes += u128::from(lifecycle.physical_bytes());
                    expected.liability_bytes += u128::from(lifecycle.liability_bytes());
                }
                UploadPhase::Cancelled => {}
            }
        }
        assert_eq!(
            tenant.map_or_else(|| owner.usage(), |p| owner.tenant_usage(p)),
            expected
        );
    }
}

#[test]
fn totals_follow_interleaved_success_rejection_and_terminal_replay() {
    for ids in [[1, 2, 3, 4, 5, 6], [6, 5, 4, 3, 2, 1]] {
        let mut owner = owner();
        let mut admitted = Vec::new();
        for id in ids {
            let input = request(
                id,
                2 + id % 2,
                if id % 3 == 0 { 0 } else { u64::from(id) * 10 },
            );
            owner.reserve(actor(input), input).unwrap();
            admitted.push(input);
            audit(&owner, &admitted);
            let mut changed = input;
            changed.object.bytes += 1;
            assert_eq!(
                owner.reserve(actor(input), changed),
                Err(UploadError::RequestConflict)
            );
            assert_eq!(
                owner.confirm_upload(input),
                Err(UploadError::InvalidPhase(UploadPhase::Reserved))
            );
            assert_eq!(owner.cancel(p(9), input), Err(UploadError::Denied));
            audit(&owner, &admitted);
            if id % 3 == 0 {
                owner.cancel(actor(input), input).unwrap();
                audit(&owner, &admitted);
                assert_eq!(
                    owner.cancel(actor(input), input),
                    Ok(LifecycleChange::Unchanged)
                );
            } else {
                owner.mark_exposure_possible(actor(input), input).unwrap();
                audit(&owner, &admitted);
                assert_eq!(
                    owner.cancel(actor(input), input),
                    Err(UploadError::InvalidPhase(UploadPhase::ExposurePossible))
                );
                if id % 2 == 0 {
                    owner.confirm_upload(input).unwrap();
                    audit(&owner, &admitted);
                    assert_eq!(owner.confirm_upload(input), Ok(LifecycleChange::Unchanged));
                    release(&mut owner, input);
                    audit(&owner, &admitted);
                    owner
                        .confirm_provider_deleted(input.object.root, input.object.first.object())
                        .unwrap();
                    audit(&owner, &admitted);
                    owner
                        .confirm_billing_stopped(input.object.root, input.object.first.object())
                        .unwrap();
                    audit(&owner, &admitted);
                    assert_eq!(owner.confirm_upload(input), Ok(LifecycleChange::Unchanged));
                }
            }
            assert_eq!(
                owner.reserve(actor(input), input),
                Ok(UploadAdmission::Existing(
                    owner.phase(actor(input), input).unwrap()
                ))
            );
            audit(&owner, &admitted);
        }
    }
}
